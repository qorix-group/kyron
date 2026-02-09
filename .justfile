# *******************************************************************************
# Copyright (c) 2026 Contributors to the Eclipse Foundation
#
# See the NOTICE file(s) distributed with this work for additional
# information regarding copyright ownership.
#
# This program and the accompanying materials are made available under the
# terms of the Apache License Version 2.0 which is available at
# https://www.apache.org/licenses/LICENSE-2.0
#
# SPDX-License-Identifier: Apache-2.0
# *******************************************************************************

# Just recipes for Kyron project
# Run `just --list` to see all available recipes
default: help

# ===============================================================================
# BAZEL BUILD COMMANDS
# ===============================================================================

# Build kyron targets for x86_64-linux
@build-x86-linux:
    echo "Building kyron targets for x86_64-linux..."
    bazel build --config x86_64-linux //...

# Build kyron targets for x86_64-qnx
@build-x86-qnx:
    echo "Building kyron targets for x86_64-qnx..."
    bazel build --config x86_64-qnx //...

# Build kyron targets for arm64-qnx
@build-arm64-qnx:
    echo "Building kyron targets for arm64-qnx..."
    bazel build --config arm64-qnx //...

# Build all kyron targets for all platforms
@build-all: build-x86-linux build-x86-qnx build-arm64-qnx
    echo "✓ All kyron targets built for all platforms"

# ===============================================================================
# BAZEL TEST COMMANDS
# ===============================================================================

# Run kyron tests for x86_64-linux
@test-x86-linux:
    echo "Running kyron tests for x86_64-linux..."
    bazel test --config x86_64-linux //src/...

# Run kyron tests for x86_64-qnx
@test-x86-qnx:
    echo "Running kyron tests for x86_64-qnx..."
    bazel test --config x86_64-qnx //src/...

# Run kyron tests for arm64-qnx
@test-arm64-qnx:
    echo "Running kyron tests for arm64-qnx..."
    bazel test --config arm64-qnx //src/...

# Run unit tests
@test-unit:
    echo "Running kyron unit tests..."
    bazel test --config x86_64-linux //:unit_tests

# Run component integration tests
@test-cit:
    echo "Running kyron component integration tests..."
    bazel test --config x86_64-linux //tests/test_cases:cit

# Run component integration tests with extended timeout (nightly)
@test-cit-repeat:
    echo "Running kyron component integration tests with repeat..."
    bazel test --config x86_64-linux //tests/test_cases:cit_repeat --test_timeout=1200

# Run component integration tests with extended timeout (nightly)
@test-all: test-x86-linux test-cit
    echo "✓ All kyron tests passed for all platforms"

# Build test scenarios
@test-scenarios-build:
    echo "Building kyron test scenarios for x86_64-linux..."
    bazel build --config x86_64-linux //tests/test_scenarios/rust:test_scenarios

# ===============================================================================
# CARGO BUILD COMMANDS
# ===============================================================================

# Cargo build
@cargo-build:
    echo "Building with Cargo..."
    cargo build

# Cargo build for test scenarios
@cargo-build-scenarios:
    echo "Building test scenarios with Cargo..."
    cargo build --manifest-path tests/test_scenarios/rust/Cargo.toml

# ===============================================================================
# CARGO TEST COMMANDS
# ===============================================================================

# Cargo test
@cargo-test:
    echo "Running tests with Cargo..."
    cargo test

# ===============================================================================
# CARGO CODE QUALITY COMMANDS
# ===============================================================================
# Format code with rustfmt
@fmt:
    echo "Formatting code with rustfmt..."
    cargo fmt

# Run clippy linter
@clippy:
    echo "Running clippy..."
    cargo clippy --all-targets --all-features -- -D warnings

# Clean Bazel build artifacts
@clean-bazel:
    echo "Cleaning Bazel artifacts..."
    bazel clean --expunge --async

# Clean Cargo build artifacts
@clean-cargo:
    echo "Cleaning Cargo artifacts..."
    cargo clean

# Clean everything
@clean: clean-bazel clean-cargo
    echo "✓ All build artifacts cleaned"

# ===============================================================================
# CI bazel checks
# ===============================================================================

# Format check
@ci-format-check:
    echo "Checking code formatting..."
    bazel test //:format.check

# Copyrights check
@ci-cr-check:
    echo "Checking copyrights..."
    bazel run //:copyright.check

# License check
@ci-license-check:
    echo "Checking licenses..."
    bazel run //:license-check

# All CI checks
@ci-all: ci-format-check ci-cr-check ci-license-check
    echo "✓ All CI checks passed"

# ===============================================================================
# HELP / DEFAULT
# ===============================================================================

# Show all available recipes
@help:
    just --list --unsorted
