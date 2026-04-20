#!/usr/bin/env bash

set -e

echo "Running release binary..."
cargo run --release

echo "Starting Prometheus..."

podman run -d \
  --name prometheus \
  -p 9090:9090 \
  -v $(pwd)/prometheus.yml:/etc/prometheus/prometheus.yml:Z \
  docker.io/prom/prometheus:latest

echo "Starting Grafana..."

podman run -d \
  --name grafana \
  -p 3001:3000 \
  -v grafana-storage:/var/lib/grafana \
  docker.io/grafana/grafana:latest

echo "Observability stack started."
echo "Prometheus: http://localhost:9090"
echo "Grafana:    http://localhost:3001"