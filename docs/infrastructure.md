# 🖥️ Infrastructure

The application is deployed using containerized Linux infrastructure.

## Host Environment

* **Operating System:** Fedora Linux
* **Service Management:** systemd
* **Reverse Proxy:** Nginx

---

## Containerization

* **Container Engine:** Podman
* **Container Runtime:** Podman
* **Base Image:** Distroless
* **Image Registry:** GitHub Container Registry (GHCR)

Container images are built using multi-stage builds for minimal runtime footprint.

---

## Container Builds

Uses a multi-stage Dockerfile with:

```
cargo chef
```

to optimize build caching and dependency reuse.

---
