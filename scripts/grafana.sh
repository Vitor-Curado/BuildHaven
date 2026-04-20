#!/usr/bin/env bash

set -e

echo "Starting Grafana..."

podman run \
  --name grafana \
  -p 3001:3000 \
  -v grafana-storage:/var/lib/grafana \
  docker.io/grafana/grafana:latest