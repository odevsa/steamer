---
name: release-umbrel
description: >-
  Workflow for validating multi-arch Docker builds, justfile versioning,
  GitHub Actions release-docker.yml, and UmbrelOS manifests.
---

# Release & UmbrelOS Packaging

## Steps

1. **Validate `justfile` & Versions**:
   - Verify `just version <X.Y.Z>` updates `backend/Cargo.toml` and `frontend/package.json` cleanly.
   - Run `just check` to ensure backend and frontend pass all checks.
2. **Validate Docker Build**:
   - Verify `docker/Dockerfile` builds cleanly (`docker build -f docker/Dockerfile .`).
   - Confirm runtime image runs as non-root user and exposes port `8080` with `/app/data` volume.
3. **Validate UmbrelOS Manifests**:
   - Check `umbrel/umbrel-app.yml` and `umbrel/docker-compose.yml` match image name, port `8080`, and `${APP_DATA_DIR}/data:/app/data`.
4. **Validate CI Workflow**:
   - Confirm `.github/workflows/release-docker.yml` builds `linux/amd64` and `linux/arm64` and merges digests into the multi-arch manifest.
