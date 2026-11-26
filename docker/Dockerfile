FROM node:22-alpine AS frontend-builder

WORKDIR /app/frontend
RUN corepack enable && corepack prepare pnpm@10 --activate

COPY frontend/package.json ./
RUN pnpm install --no-frozen-lockfile

COPY frontend/ ./
RUN pnpm exec paraglide-js compile --project ./project.inlang --outdir ./src/lib/paraglide \
    && pnpm build

FROM rust:1-bookworm AS backend-builder

WORKDIR /app/backend

COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

COPY --from=frontend-builder /app/frontend/build /app/frontend/build
COPY backend/build.rs ./
COPY backend/migrations ./migrations
COPY backend/src ./src
RUN touch src/main.rs && cargo build --release --locked

FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid 1000 steamer \
    && useradd --uid 1000 --gid 1000 --create-home --shell /usr/sbin/nologin steamer \
    && mkdir -p /app/data \
    && chown -R steamer:steamer /app

WORKDIR /app

COPY --from=backend-builder /app/backend/target/release/steamer /usr/local/bin/steamer

ENV HOST=0.0.0.0 \
    PORT=8080 \
    DATA_DIR=/app/data \
    RUST_LOG=info

VOLUME ["/app/data"]
EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -fsS http://127.0.0.1:8080/api/v1/health || exit 1

USER steamer

CMD ["/usr/local/bin/steamer"]
