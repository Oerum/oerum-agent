#!/usr/bin/env sh
set -eu
echo "Running Unix install smoke test"
test -f install/bootstrap.sh
echo "Smoke check passed"
