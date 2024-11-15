#!/bin/bash

# Function to connect to defindex-soroban container
connect_to_container() {
  echo "Connecting to defindex-soroban container..."
  docker exec --tty --interactive defindex-soroban bash
}

if [[ $# -eq 0 ]]; then
  # No arguments, connect to defindex-soroban
  connect_to_container
elif [[ $1 == "--no-blockchain" || $1 == "--nb" ]]; then
  # With --no-blockchain, start only defindex-soroban container and connect
  echo "Starting only defindex-soroban container..."
  docker-compose up -d defindex-soroban
  connect_to_container
else
  # Any other argument, just connect to defindex-soroban
  connect_to_container
fi
