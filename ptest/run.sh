#!/bin/bash
cargo run --release -- \
    --users 5000 \
    --run-time 60s \
    --startup-time 1s \
    --host http://172.18.0.21:8000 \
    --report-file=report.html \
    --no-reset-metrics
