#!/bin/env bash

set -e

TS=$(date +%s)
time cargo run --release > output_${TS}.ppm && open output_${TS}.ppm
