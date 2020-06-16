#!/bin/bash

# Run a test to forcefully create the build environment
./tools/devtool test -- integration_tests/build/test_binary_size.py

# Build the container
docker build -t fc$1 -f tools/buildctr/Dockerfile