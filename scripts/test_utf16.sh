#!/bin/sh

cargo nextest r --no-default-features --features "simd, metric_utf16" "$@"
