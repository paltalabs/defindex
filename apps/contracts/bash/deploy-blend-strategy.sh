#!/usr/bin/env bash
#
# Deploy a Blend strategy contract on Stellar mainnet from a pre-uploaded WASM.
#
# Usage:
#   Interactive:  ./deploy-blend-strategy.sh
#   Positional:   ./deploy-blend-strategy.sh <asset> <blend_pool> [blend_token] [soroswap_router] [reward_threshold] [keeper]
#
# Missing optional positional args fall back to the defaults below.
# Pass "-" for an optional positional to explicitly accept its default.

set -euo pipefail

# --- Banner (single-quoted heredoc so $/backticks aren't expanded) ---

cat <<'BANNER'
░▒▓███████▓▒░░▒▓█▓▒░      ░▒▓████████▓▒░▒▓███████▓▒░░▒▓███████▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓███████▓▒░░▒▓█▓▒░      ░▒▓██████▓▒░ ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓███████▓▒░░▒▓████████▓▒░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓███████▓▒░

 ░▒▓███████▓▒░▒▓████████▓▒░▒▓███████▓▒░ ░▒▓██████▓▒░▒▓████████▓▒░▒▓████████▓▒░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░         ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░   ░▒▓█▓▒░     ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
░▒▓█▓▒░         ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░   ░▒▓█▓▒░     ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░
 ░▒▓██████▓▒░   ░▒▓█▓▒░   ░▒▓███████▓▒░░▒▓████████▓▒░ ░▒▓█▓▒░   ░▒▓██████▓▒░░▒▓█▓▒▒▓███▓▒░░▒▓██████▓▒░
       ░▒▓█▓▒░  ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░   ░▒▓█▓▒░     ░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
       ░▒▓█▓▒░  ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░   ░▒▓█▓▒░     ░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓███████▓▒░   ░▒▓█▓▒░   ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░   ░▒▓████████▓▒░▒▓██████▓▒░   ░▒▓█▓▒░
BANNER
echo

readonly NETWORK="mainnet"
readonly NETWORK_RPC_URL="https://rpc.lightsail.network"
readonly NETWORK_PASSPHRASE="Public Global Stellar Network ; September 2015"
readonly SOURCE_ACCOUNT="deployer"
readonly WASM_HASH="11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988"
readonly XLM_SAC_ID="CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA"

readonly DEFAULT_BLEND_TOKEN="CD25MNVTZDL4Y3XBCPCJXGXATV5WUHHOWMYFF4YBEGU5FCPGMYTVG5JY"
readonly DEFAULT_SOROSWAP_ROUTER="CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH"
readonly DEFAULT_REWARD_THRESHOLD="40"
readonly DEFAULT_KEEPER="GC7LSNNCRW4PNLLHIXE53RB7N5BKSQVXTGWYHIYBA7LAWY7XYPC4JGCP"

readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly CONTRACTS_JSON="$SCRIPT_DIR/../../../public/mainnet.contracts.json"

# --- Helpers ---

die() { echo "error: $*" >&2; exit 1; }

prompt_required() {
  local label="$1" answer
  while true; do
    read -rp "$label: " answer
    [[ -n "$answer" ]] && { printf '%s' "$answer"; return; }
    echo "  value required" >&2
  done
}

prompt_default() {
  local label="$1" default="$2" answer
  read -rp "$label [$default]: " answer
  printf '%s' "${answer:-$default}"
}

# Resolve a positional arg: prompt when unset, use default when empty or "-", otherwise use the supplied value.
resolve_with_default() {
  local label="$1" default="$2" supplied="${3-__UNSET__}"
  if [[ "$supplied" == "__UNSET__" ]]; then
    prompt_default "$label" "$default"
  elif [[ -z "$supplied" || "$supplied" == "-" ]]; then
    printf '%s' "$default"
  else
    printf '%s' "$supplied"
  fi
}

resolve_required() {
  local label="$1" supplied="${2-__UNSET__}"
  if [[ "$supplied" == "__UNSET__" ]]; then
    prompt_required "$label"
  else
    [[ -n "$supplied" ]] || die "$label is required"
    printf '%s' "$supplied"
  fi
}

# Run a command with a rotating spinner. The command's stdout+stderr are captured
# and replayed on stdout when it finishes; the spinner itself goes to stderr so
# this wrapper is safe inside $(...) command substitutions.
run_with_spinner() {
  local label="$1"; shift
  local tmp rc_file rc
  tmp="$(mktemp)"
  rc_file="$(mktemp)"

  ( "$@" >"$tmp" 2>&1; echo $? >"$rc_file" ) &
  local pid=$!

  local chars='|/-\' i=0
  while kill -0 "$pid" 2>/dev/null; do
    printf '\r  %s %s' "${chars:$((i++ % ${#chars})):1}" "$label" >&2
    sleep 0.1
  done
  wait "$pid" 2>/dev/null || true
  printf '\r\033[K' >&2

  rc="$(cat "$rc_file")"
  rm -f "$rc_file"
  cat "$tmp"
  rm -f "$tmp"
  return "${rc:-1}"
}

# --- Ensure network exists ---
ensure_network() {
  if stellar network ls 2>/dev/null | grep -qw "$NETWORK"; then
    echo "✓ network '$NETWORK' configured"
  else
    echo "→ adding network '$NETWORK'"
    stellar network add "$NETWORK" \
      --rpc-url "$NETWORK_RPC_URL" \
      --network-passphrase "$NETWORK_PASSPHRASE"
  fi
}

# --- Ensure deployer identity exists ---
ensure_deployer() {
  if ! stellar keys ls 2>/dev/null | grep -qw "$SOURCE_ACCOUNT"; then
    die "identity '$SOURCE_ACCOUNT' not found. Create one first with:
    stellar keys generate $SOURCE_ACCOUNT --network $NETWORK     # new key
    stellar keys add $SOURCE_ACCOUNT --secret-key                # import existing
  Then fund the account with XLM before re-running this script."
  fi

  local pub
  pub="$(stellar keys public-key "$SOURCE_ACCOUNT")"
  echo "✓ identity '$SOURCE_ACCOUNT' → $pub"

  # Read XLM balance via the native SAC. `balance` returns an i128 in stroops.
  local balance_output balance_stroops balance_xlm
  if ! balance_output="$(run_with_spinner "checking XLM balance..." \
    stellar contract invoke \
      --id "$XLM_SAC_ID" \
      --source-account "$SOURCE_ACCOUNT" \
      --network "$NETWORK" \
      --send no \
      -- balance --id "$pub")"; then
    die "failed to fetch XLM balance for $pub:
$balance_output"
  fi

  balance_stroops="${balance_output//\"/}"

  if [[ -z "$balance_stroops" || "$balance_stroops" == "0" ]]; then
    die "deployer has 0 XLM on $NETWORK. Fund $pub before retrying."
  fi

  balance_xlm="$(awk -v s="$balance_stroops" 'BEGIN { printf "%.7f", s / 10000000 }')"
  echo "  balance: $balance_xlm XLM ($balance_stroops stroops)"
}

# --- Collect args ---
ensure_network
ensure_deployer
echo

ASSET=$(resolve_required      "Underlying asset address for the strategy" "${1-__UNSET__}")
BLEND_POOL=$(resolve_required "Blend pool address"                        "${2-__UNSET__}")
BLEND_TOKEN=$(resolve_with_default      "Blend (BLND) token address" "$DEFAULT_BLEND_TOKEN"      "${3-__UNSET__}")
SOROSWAP_ROUTER=$(resolve_with_default  "Soroswap router address"    "$DEFAULT_SOROSWAP_ROUTER"  "${4-__UNSET__}")
REWARD_THRESHOLD=$(resolve_with_default "Reward threshold (i128)"    "$DEFAULT_REWARD_THRESHOLD" "${5-__UNSET__}")
KEEPER=$(resolve_with_default           "Keeper address"             "$DEFAULT_KEEPER"           "${6-__UNSET__}")

# init_args is Vec<Val>, so each element must be a type-tagged ScVal object
# (e.g. {"address": "..."}, {"i128": "..."}) — bare strings/numbers are rejected.
INIT_ARGS_JSON=$(printf '[{"address":"%s"},{"address":"%s"},{"address":"%s"},{"i128":"%s"},{"address":"%s"}]' \
  "$BLEND_POOL" \
  "$BLEND_TOKEN" \
  "$SOROSWAP_ROUTER" \
  "$REWARD_THRESHOLD" \
  "$KEEPER")

echo
echo "──────────────────────────────────────"
echo " Deploy summary"
echo "──────────────────────────────────────"
echo " network:          $NETWORK"
echo " source:           $SOURCE_ACCOUNT"
echo " wasm hash:        $WASM_HASH"
echo " asset:            $ASSET"
echo " blend pool:       $BLEND_POOL"
echo " blend token:      $BLEND_TOKEN"
echo " soroswap router:  $SOROSWAP_ROUTER"
echo " reward threshold: $REWARD_THRESHOLD"
echo " keeper:           $KEEPER"
echo " init_args json:   $INIT_ARGS_JSON"
echo "──────────────────────────────────────"

read -rp "Deploy now? [y/N] " confirm
[[ "$confirm" =~ ^[yY]$ ]] || { echo "aborted"; exit 0; }

# Capture stdout (the contract address) — stellar CLI prints progress to stderr,
# which still streams to the user's terminal in real time.
DEPLOYED_ADDRESS="$(stellar contract deploy \
  --wasm-hash "$WASM_HASH" \
  --source-account "$SOURCE_ACCOUNT" \
  --network "$NETWORK" \
  -- \
  --asset "$ASSET" \
  --init_args "$INIT_ARGS_JSON")"

echo
echo "✅ Deployed contract: $DEPLOYED_ADDRESS"
echo

read -rp "Store this address in public/mainnet.contracts.json? [y/N] " store_confirm
[[ "$store_confirm" =~ ^[yY]$ ]] || exit 0

command -v jq >/dev/null 2>&1 \
  || { echo "jq is required to update the contracts file (install: brew install jq). Skipping."; exit 0; }

[[ -f "$CONTRACTS_JSON" ]] || die "contracts file not found: $CONTRACTS_JSON"

read -rp "Name to store it under (e.g. xlm_blend_autocompound_strategy): " contract_name
[[ -n "$contract_name" ]] || die "name is required"

existing="$(jq -r --arg name "$contract_name" '.ids[$name] // empty' "$CONTRACTS_JSON")"
if [[ -n "$existing" ]]; then
  read -rp "'$contract_name' already exists ($existing). Overwrite? [y/N] " overwrite
  [[ "$overwrite" =~ ^[yY]$ ]] || { echo "aborted"; exit 0; }
fi

tmp="$(mktemp)"
jq --arg name "$contract_name" --arg addr "$DEPLOYED_ADDRESS" \
  '.ids[$name] = $addr' "$CONTRACTS_JSON" > "$tmp"
mv "$tmp" "$CONTRACTS_JSON"
echo "✓ stored '$contract_name' → $DEPLOYED_ADDRESS in $CONTRACTS_JSON"
