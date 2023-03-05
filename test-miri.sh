#!/bin/bash
set -e

export MIRIFLAGS="-Zmiri-strict-provenance"

cargo +nightly miri test
