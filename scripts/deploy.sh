#!/bin/bash
set -euo pipefail

cd /srv/Personal-website

echo "Updating repository..."
git fetch origin
git reset --hard origin/main

echo "Pulling latest image from GHCR..."
podman pull ghcr.io/vitor-curado/buildhaven:latest

echo "Restarting service..."
systemctl restart buildhaven

echo "Cleaning dangling images..."
podman image prune -f

echo "Done."