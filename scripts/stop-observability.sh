#!/usr/bin/env bash

echo "Stopping containers..."

podman stop prometheus || true
podman stop grafana || true

podman rm prometheus || true
podman rm grafana || true

echo "Observability stack stopped."