#!/bin/bash
set -euo pipefail

cd /srv/Personal-website

echo "Updating repository..."
git fetch origin
git reset --hard origin/main

echo "Pulling latest images..."
podman-compose pull

echo "Restarting services..."
podman-compose up -d

echo "Cleaning dangling images..."
podman image prune -f

echo "Done."