#!/bin/bash

ADMIN_FILE=/workspace/.soroban/identity/admin.toml
NETWORK_TESTNET_FILE=/workspace/.soroban/network/testnet.toml
NETWORK_STANDALONE_FILE=/workspace/.soroban/network/standalone.toml

NETWORK="$1"

echo "-----------------------------------------"
echo "        Using $NETWORK                   "
echo "-----------------------------------------"

check_and_setup() {
    local file=$1
    local setup_cmd=$2

    if [ -f "$file" ]; then
        echo "$file exists."
    else
        echo "$file does not exist. Running setup command: $setup_cmd"
        eval "$setup_cmd"
        echo "Setup completed."
        echo "-----------------------------------------"
    fi
}

# Check and set up the appropriate network file
if [ "$NETWORK" == "testnet" ]; then
  echo "Checking Config"
  check_and_setup "$NETWORK_TESTNET_FILE" "soroban config network add --rpc-url https://soroban-testnet.stellar.org/ --network-passphrase 'Test SDF Network ; September 2015' testnet"
  FRIENDBOT_URL=https://friendbot.stellar.org/
  echo "-----------------------------------------"
elif [ "$NETWORK" == "standalone" ]; then
  echo "Checking Config"
  check_and_setup "$NETWORK_STANDALONE_FILE" "soroban config network add --rpc-url http://stellar:8000/soroban/rpc --network-passphrase 'Standalone Network ; February 2017' standalone"
  FRIENDBOT_URL=http://stellar:8000/friendbot
  echo "-----------------------------------------"
else
    echo "Unknown network: $NETWORK"
    exit 1
fi

echo "Continuing with the rest of the script..."
echo "-----------------------------------------"

# Check and set up the admin file
check_and_setup "$ADMIN_FILE" "soroban keys generate --network '$NETWORK' admin"

# funding admin account
ADMIN_ADDRESS="$(soroban config identity address admin)"
echo "-----------------------------------------"
echo "Funding admin account"
echo "Address: $ADMIN_ADDRESS"
echo "-----------------------------------------"
curl -s -o /dev/null -X POST "$FRIENDBOT_URL?addr=$ADMIN_ADDRESS" 2>&1

echo "Deploying Xycloans Pool for XLM"
echo "-----------------------------------------"
POOL_ADDRESS="$(soroban contract deploy --source admin --network "$NETWORK" --wasm ./adapters/xycloans/xycloans_pool.wasm)"
echo "$POOL_ADDRESS"

echo "Initializing Xycloans Pool"
echo "-----------------------------------------"
# this may fail if it is alrady wrapped
soroban lab token wrap --asset native --network standalone --source admin > /dev/null 2>&1
XLM_ADDRESS="$(soroban lab token id --asset native --network standalone --source admin)"

soroban contract invoke --id "$POOL_ADDRESS" --network standalone --source admin -- initialize --token "$XLM_ADDRESS"
echo "Xycloans Initialized"
echo "$POOL_ADDRESS" > /workspace/.soroban/xycloans_id
echo "Contract ID can be found in /workspace/.soroban/xycloans_id"