#!/usr/bin/env bash

set -e

podman stop prometheus || true
podman rm prometheus || true

echo "Starting Prometheus..."

podman run \
  --name prometheus \
  -p 9090:9090 \
  -v $(pwd)/prometheus.yml:/etc/prometheus/prometheus.yml:Z \
  docker.io/prom/prometheus:latest