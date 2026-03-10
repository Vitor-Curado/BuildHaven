#!/bin/bash
set -e

cd /srv/personal-website

git pull

docker compose build
docker compose up -d