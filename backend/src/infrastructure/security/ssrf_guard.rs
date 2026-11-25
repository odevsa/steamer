use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use url::{Host, Url};

use crate::domain::errors::DomainError;
use crate::domain::ports::UrlValidator;

#[derive(Debug, Clone)]
pub struct SsrfGuard {
    allow_http_webhooks: bool,
}

impl SsrfGuard {
    pub fn new(allow_http_webhooks: bool) -> Self {
        Self {
            allow_http_webhooks,
        }
    }
}

#[async_trait::async_trait]
impl UrlValidator for SsrfGuard {
    async fn validate_outbound_url(&self, raw_url: &str) -> Result<(), DomainError> {
        let parsed = Url::parse(raw_url.trim())
            .map_err(|_| DomainError::Validation("invalid URL format".to_string()))?;

        match parsed.scheme() {
            "https" => {}
            "http" if self.allow_http_webhooks => {}
            "http" => {
                return Err(DomainError::Security(
                    "only HTTPS URLs are allowed".to_string(),
                ));
            }
            _ => {
                return Err(DomainError::Security("unsupported URL scheme".to_string()));
            }
        }

        if !parsed.username().is_empty() || parsed.password().is_some() {
            return Err(DomainError::Security(
                "URLs with embedded credentials are not allowed".to_string(),
            ));
        }

        let port = parsed.port_or_known_default().ok_or_else(|| {
            DomainError::Validation("URL port could not be determined".to_string())
        })?;

        let host = parsed
            .host()
            .ok_or_else(|| DomainError::Validation("URL must contain a host".to_string()))?;

        match host {
            Host::Ipv4(ip) => {
                if is_forbidden_ip(IpAddr::V4(ip)) {
                    return Err(DomainError::Security(
                        "outbound URL targets a forbidden IP address".to_string(),
                    ));
                }
            }
            Host::Ipv6(ip) => {
                if is_forbidden_ip(IpAddr::V6(ip)) {
                    return Err(DomainError::Security(
                        "outbound URL targets a forbidden IP address".to_string(),
                    ));
                }
            }
            Host::Domain(domain) => {
                let lower = domain.to_ascii_lowercase();
                if lower == "localhost"
                    || lower.ends_with(".localhost")
                    || lower.ends_with(".local")
                    || lower.ends_with(".internal")
                {
                    return Err(DomainError::Security(
                        "outbound URL targets a restricted local hostname".to_string(),
                    ));
                }

                let addrs = tokio::net::lookup_host((domain, port)).await.map_err(|_| {
                    DomainError::Security("unable to resolve outbound URL host".to_string())
                })?;

                let mut resolved_any = false;
                for addr in addrs {
                    resolved_any = true;
                    if is_forbidden_ip(addr.ip()) {
                        return Err(DomainError::Security(
                            "outbound URL resolves to a forbidden IP range".to_string(),
                        ));
                    }
                }
                if !resolved_any {
                    return Err(DomainError::Security(
                        "outbound URL did not resolve to any address".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}

pub fn is_forbidden_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_forbidden_ipv4(v4),
        IpAddr::V6(v6) => is_forbidden_ipv6(v6),
    }
}

fn is_forbidden_ipv4(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();
    let is_cgnat = octets[0] == 100 && (octets[1] & 0b1100_0000) == 64;
    let is_benchmark = octets[0] == 198 && (octets[1] == 18 || octets[1] == 19);
    let is_ietf_protocol = octets[0] == 192 && octets[1] == 0 && octets[2] == 0;
    let is_reserved_future = octets[0] >= 240;

    ip.is_loopback()
        || ip.is_private()
        || ip.is_link_local()
        || ip.is_multicast()
        || ip.is_broadcast()
        || ip.is_unspecified()
        || ip.is_documentation()
        || is_cgnat
        || is_benchmark
        || is_ietf_protocol
        || is_reserved_future
}

fn is_forbidden_ipv6(ip: Ipv6Addr) -> bool {
    if let Some(mapped) = ip.to_ipv4_mapped() {
        return is_forbidden_ipv4(mapped);
    }
    let segments = ip.segments();
    let is_unique_local = (segments[0] & 0xfe00) == 0xfc00;
    let is_unicast_link_local = (segments[0] & 0xffc0) == 0xfe80;

    ip.is_loopback()
        || ip.is_unspecified()
        || ip.is_multicast()
        || is_unique_local
        || is_unicast_link_local
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    use super::{is_forbidden_ip, SsrfGuard};
    use crate::domain::ports::UrlValidator;

    #[test]
    fn blocks_private_loopback_and_link_local_ips() {
        assert!(is_forbidden_ip(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
        assert!(is_forbidden_ip(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))));
        assert!(is_forbidden_ip(IpAddr::V4(Ipv4Addr::new(172, 16, 5, 1))));
        assert!(is_forbidden_ip(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10))));
        assert!(is_forbidden_ip(IpAddr::V4(Ipv4Addr::new(
            169, 254, 169, 254
        ))));
        assert!(is_forbidden_ip(IpAddr::V4(Ipv4Addr::new(100, 64, 0, 1))));
        assert!(is_forbidden_ip(IpAddr::V6(Ipv6Addr::LOCALHOST)));
        assert!(is_forbidden_ip(IpAddr::V6(
            "::ffff:127.0.0.1".parse().expect("valid ipv6")
        )));
        assert!(is_forbidden_ip(IpAddr::V6(
            "fe80::1".parse().expect("valid ipv6")
        )));
        assert!(is_forbidden_ip(IpAddr::V6(
            "fc00::1".parse().expect("valid ipv6")
        )));
    }

    #[test]
    fn allows_public_ip_addresses() {
        assert!(!is_forbidden_ip(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
        assert!(!is_forbidden_ip(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))));
        assert!(!is_forbidden_ip(IpAddr::V6(
            "2606:4700:4700::1111".parse().expect("valid ipv6")
        )));
    }

    #[tokio::test]
    async fn rejects_http_when_disabled_and_localhost_domains() {
        let guard = SsrfGuard::new(false);
        assert!(guard
            .validate_outbound_url("http://example.com/hook")
            .await
            .is_err());
        assert!(guard
            .validate_outbound_url("https://localhost/hook")
            .await
            .is_err());
        assert!(guard
            .validate_outbound_url("https://127.0.0.1/hook")
            .await
            .is_err());
        assert!(guard
            .validate_outbound_url("https://169.254.169.254/latest")
            .await
            .is_err());
        assert!(guard
            .validate_outbound_url("https://8.8.8.8/hook")
            .await
            .is_ok());
    }
}
