#!/usr/bin/env bash

set -e

cargo test

git diff --exit-code
