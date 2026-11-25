use crate::domain::value_objects::MoneyCents;

pub fn format_money(cents: MoneyCents, currency: &str) -> String {
    let raw = cents.value();
    let major = raw / 100;
    let minor = raw % 100;
    format!("{currency} {major}.{minor:02}")
}

pub fn escape_html(raw: &str) -> String {
    raw.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
