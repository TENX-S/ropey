#!/bin/sh

cargo nextest r --no-default-features --features "simd, metric_lines_lf" "$@"
