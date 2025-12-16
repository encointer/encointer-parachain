# Helper script to open HRMP channels between Encointer and Asset Hub.
# This script is meant to be run after the relay chain and parachains are spawned.

# Change for your bin path
POLKADOT_BIN=~/bin/polkadot
POLKADOT_PARACHAIN_BIN=~/bin/polkadot-parachain
FRAMEWORK_PATH=~/polkadot-sdk/bridges/testing/framework
source "$FRAMEWORK_PATH/utils/bridges.sh"

function ensure_binaries() {
    echo "*** Checking for required binaries"
    if [[ ! -f "$POLKADOT_BIN" ]]; then
        echo "  Required polkadot binary '$POLKADOT_BIN' does not exist!"
        echo "  You need to build it and copy to this location!"
        exit 1
    fi
    if [[ ! -f "$POLKADOT_PARACHAIN_BIN" ]]; then
        echo "  Required polkadot-parachain binary '$POLKADOT_PARACHAIN_BIN' does not exist!"
        echo "  You need to build it and copy to this location!"
        exit 1
    fi
    echo "*** All required binaries are present"
}

#function ensure_polkadot_js_api() {
#    echo "*** Checking for required polkadot-js-api"
#    if ! which polkadot-js-api &>/dev/null; then
#        echo ''
#        echo 'Required command `polkadot-js-api` not in PATH, please, install, e.g.:'
#        echo "npm install -g @polkadot/api-cli"
#        echo "      or"
#        echo "yarn global add @polkadot/api-cli"
#        echo ''
#        exit 1
#    fi
#}
#
#function open_hrmp_channels() {
#    local relay_url=$1
#    local relay_chain_seed=$2
#    local sender_para_id=$3
#    local recipient_para_id=$4
#    local max_capacity=$5
#    local max_message_size=$6
#    echo "  calling open_hrmp_channels:"
#    echo "      relay_url: ${relay_url}"
#    echo "      relay_chain_seed: ${relay_chain_seed}"
#    echo "      sender_para_id: ${sender_para_id}"
#    echo "      recipient_para_id: ${recipient_para_id}"
#    echo "      max_capacity: ${max_capacity}"
#    echo "      max_message_size: ${max_message_size}"
#    echo "      params:"
#    echo "--------------------------------------------------"
#    polkadot-js-api \
#        --ws "${relay_url?}" \
#        --seed "${relay_chain_seed?}" \
#        --sudo \
#        tx.hrmp.forceOpenHrmpChannel \
#        ${sender_para_id} \
#        ${recipient_para_id} \
#        ${max_capacity} \
#        ${max_message_size}
#}

# Check for binaries
ensure_binaries

# Check for polkadot-js-api cli
ensure_polkadot_js_api

## HRMP: NCTR - Asset Hub
#open_hrmp_channels \
#    "ws://127.0.0.1:9999" \
#    "//Alice" \
#    1003 1000 4 524288
#
## HRMP: Asset Hub - NCTR
#open_hrmp_channels \
#    "ws://127.0.0.1:9999" \
#    "//Alice" \
#    1000 1003 4 524288
#
## create USDC.p asset with Alice as owner
#force_create_foreign_asset \
#    "ws://127.0.0.1:9999" \
#    "//Alice" \
#    1000 \
#    "ws://127.0.0.1:9010" \
#    "$(jq --null-input '{ "parents": 2, "interior": { "X4": [ { "GlobalConsensus": "Polkadot" }, { "Parachain": 1000 }, { "PalletInstance": 50 }, { "GeneralIndex": 1337 } ] } }')" \
#    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" \
#    10000 \
#    true

# set metadata for USDC.p asset
polkadot-js-api \
    --ws "ws://127.0.0.1:9010" \
    --seed "//Alice" \
    tx.foreignAssets.setMetadata \
    "$(jq --null-input '{ "parents": 2, "interior": { "X4": [ { "GlobalConsensus": "Polkadot" }, { "Parachain": 1000 }, { "PalletInstance": 50 }, { "GeneralIndex": 1337 } ] } }')" \
    "USD Circle" \
    "USDC" \
    6

# mint 2M USDC to Alice
polkadot-js-api \
    --ws "ws://127.0.0.1:9010" \
    --seed "//Alice" \
    tx.foreignAssets.mint \
    "$(jq --null-input '{ "parents": 2, "interior": { "X4": [ { "GlobalConsensus": "Polkadot" }, { "Parachain": 1000 }, { "PalletInstance": 50 }, { "GeneralIndex": 1337 } ] } }')" \
    "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" \
    2000000000000

# mint 1001 USDC to MTA treasury
polkadot-js-api \
    --ws "ws://127.0.0.1:9010" \
    --seed "//Alice" \
    tx.foreignAssets.mint \
    "$(jq --null-input '{ "parents": 2, "interior": { "X4": [ { "GlobalConsensus": "Polkadot" }, { "Parachain": 1000 }, { "PalletInstance": 50 }, { "GeneralIndex": 1337 } ] } }')" \
    "Cpm6kN65xUxgxn5s1F4Sszo3ux5qQNoX6LW7qGBpy48CDCD" \
    1001000000

# put 1234 KSM into local MTA community treasury on NCTR. (use runtime API treasuriesApi to find accountId)
polkadot-js-api \
    --ws "ws://127.0.0.1:9944" \
    --seed "//Alice" \
    tx.balances.transferKeepAlive \
    "D2RGN78sWWAYZPRxJCZregQ58PHqq68QrSg9CgEuwC1Jkic" \
    1234000000000000

 # put 12 KSM into global treasury on NCTR. (use runtime API treasuriesApi to find accountId)
polkadot-js-api \
    --ws "ws://127.0.0.1:9944" \
    --seed "//Alice" \
    tx.balances.transferKeepAlive \
    "FyioZqQFj4MJ6YZtmvadKp2bGvffNZKArqUymMUVmpdHEFQ" \
    12000000000000

