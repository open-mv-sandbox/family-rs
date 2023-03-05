#!/bin/bash
set -e

cargo +nightly miri test
