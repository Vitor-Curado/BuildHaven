# 🚀 Deployment Pipeline

The application is deployed using a registry-driven container workflow.

## Infrastructure

- **OS:** Fedora Linux
- **Service management:** systemd
- **Reverse proxy:** Nginx

## Containerization

- **Engine:** Podman
- **Runtime:** Podman
- **Base image:** Distroless
- **Registry:** GitHub Container Registry (GHCR)

Multi-stage container builds for minimal footprint.

## Flow Overview

```
Developer pushes to GitHub (main branch)
        ↓
Woodpecker CI runs checks:
    - cargo fmt
    - cargo clippy
    - cargo test
        ↓
Container image is built using Dockerfile
        ↓
Image is pushed to GitHub Container Registry (GHCR)
        ↓
Production server pulls latest image
        ↓
systemd restarts container
        ↓
Nginx serves updated application
```

---

## 🧰 Production Runtime

The production server uses:

* Podman for container runtime
* systemd for container lifecycle management
* GitHub Container Registry (GHCR) as image source
* Nginx as reverse proxy

The systemd unit automatically pulls the latest image before starting:

```ini
ExecStartPre=/usr/bin/podman pull ghcr.io/vitor-curado/personal-website:latest
```