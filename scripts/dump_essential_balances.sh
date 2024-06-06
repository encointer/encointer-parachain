#!/bin/bash
# make sure polkadot-js-api is in the path
# run zombienet locally
#
# example
# ---------------
# ./dump-essential-balances.sh > fee1.txt
# DO YOUR THING
# ./dump-essential-balances.sh > fee2.txt
#
# compare balances pre/post YOUR THING
# diff -y -W 80 fee1.txt fee2.txt

#ASSET_HUB="ws://127.0.0.1:9954"
ENCOINTER="ws://127.0.0.1:9944"
ROCOCO="ws://127.0.0.1:9999"

# dummy treasury subkey inspect --public 0x0000000000000000000000000000000000000000000000000000000000000000
TREASURY=5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM
POT=5EYCAe5cKPAoFh2HnQQvpKqRYZGqBpaA87u4Zzw89qPE58is

function print_balances() {
    echo "*** print balances for $1"
    # echo "Rococo ROC"
    # polkadot-js-api --ws $ROCOCO query.system.account $1 | jq .account.data.free
    echo "Encointer ROC"
    polkadot-js-api --ws $ENCOINTER query.system.account $1 | jq .account.data.free
}

echo "*** total supply of ROC@Encointer"
polkadot-js-api --ws $ENCOINTER query.balances.totalIssuance | jq .totalIssuance

echo "*** Alice"
print_balances 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
echo "*** Bob"
print_balances 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
echo "*** author1 "
print_balances EXHoFmksfPoyTyuFgcJ2c11uLj9KPgCZ2wWxshxuDrvpZuq
echo "*** author2 "
print_balances E8Np37TgMvVNST2Qj7YpEMvNon2kBSPNSkMM9D2TuKjwKQZ
echo "*** staking pot"
print_balances $POT






