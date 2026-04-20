#!/usr/bin/env bash

set -euo pipefail

URL="${1:-http://127.0.0.1:3000/}"
DURATION="${DURATION:-30s}"
THREADS="${THREADS:-16}"
CONNECTIONS="${CONNECTIONS:-1000}"

TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")

OUT="benchmarks/${TIMESTAMP}.txt"

mkdir -p benchmarks

{
echo "=============================="
echo "Benchmark Timestamp: $TIMESTAMP"
echo "URL: $URL"
echo "Threads: $THREADS"
echo "Connections: $CONNECTIONS"
echo "Duration: $DURATION"
echo "=============================="
echo ""

wrk \
  -t$THREADS \
  -c$CONNECTIONS \
  -d$DURATION \
  --latency \
  "$URL"

echo ""
echo "=============================="
echo ""

} | tee "$OUT"

echo "Saved to $OUT"