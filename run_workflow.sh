#!/bin/bash
set -e

if [ "$1" == "local" ]; then
  echo "Running GitHub Actions workflow locally..."
  act -W .github/workflows/test-contracts.yml -j build_and_test --container-architecture linux/amd64
  #cd apps/contracts && make test
elif [ "$1" == "remote" ]; then
  echo "Running GitHub Actions workflow remote..."
  cd apps/contracts
  make test || exit 1

else
  echo "Invalid parameter. Use 'local' or 'remote'."
  exit 1
fi