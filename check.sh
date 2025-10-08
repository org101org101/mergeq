#!/usr/bin/env bash

set -e

cargo check

git diff --exit-code
