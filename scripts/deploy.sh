#!/bin/bash
set -e

cd /srv/Personal-website

echo "Updating repository..."
git fetch origin
git reset --hard origin/main

echo "Stopping current containers..."
podman compose down

echo "Building new image..."
podman compose build --pull

echo "Starting containers..."
podman compose up -d

echo "Cleaning unused images..."
podman image prune -f

echo "Deployment complete."