USER_PEM="~/wallets/development.pem"
PROXY="https://gateway.elrond.com"
CHAIN_ID="1"

deploy() {
    mxpy --verbose contract deploy --project=${PROJECT} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=20000000 \
    --arguments "str:TACC-73857e" \
    --send --outfile="deploy.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

build() {
    mxpy contract build
}

getUserUnStaked() {
    mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqtjt0puryxc4c68qhf9v387j379phnwrm4jwspypwnt \
    --function "getUserUnstakedFirstCollection" --arguments erd1e9nnlhee5rvn9k8y3ysmtsx490h8nw9jels5vjdlezl72yrtg3yq6vvj3r \
    --proxy=${PROXY} || return
}

getNumberUnstaked() {
    mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h \
    --function "getNumberUnstakedFirstCollection" \
    --proxy=${PROXY} || return
}

getNumberUnstakedSecond() {
    mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h \
    --function "getNumberUnstakedSecondCollection" \
    --proxy=${PROXY} || return
}

getUserStaked_second() {
    mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h \
    --function "getUserStakedSecondCollection" --arguments erd1e8xe4h3wm38tgapuyzc9xwfsspf0jkfpzht0pa2xavetq3mxn0rqdseqhj \
    --proxy=${PROXY} || return
}






getNumberStakedFirstCollection() {
    mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h \
    --function "getNumberStakedFirstCollection" \
    --proxy=${PROXY} || return
}
getNumberStakedSecondCollection() {
    mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h \
    --function "getNumberStakedSecondCollection" \
    --proxy=${PROXY} || return
}

getUsersStaked() {
    mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqtjt0puryxc4c68qhf9v387j379phnwrm4jwspypwnt \
    --function "getUsersStakedFirstCollection" \
    --proxy=${PROXY} || return
}

getUserStaked() {
    mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h \
    --function "getUserUnstakedSecondCollection" --arguments erd1qjlu8anps2hhhw2p0wyfuaawa63awgl6pzuh6fh8tkj7x7cf34ystznqxd \
    --proxy=${PROXY} || return
}

getUserRewards() {
    mxpy --verbose contract query erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h \
    --function "userRewards" --arguments erd1pp0nynmlrn5weqzyxve2vmlwgr8svx2jdve3f45drxj9ctu9wsmqlk2pss \
    --proxy=${PROXY} || return
}




getgenerico() {
    erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h \
    --function "getGln"  \
    --proxy=${PROXY} || return
}



upgrade() {
    mxpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgqtjt0puryxc4c68qhf9v387j379phnwrm4jwspypwnt --project=${PROJECT} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=80000000 \
    --send --outfile="deploy.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

upgrade_mainnet() {
    mxpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h --project=${PROJECT} \
    --recall-nonce --pem="~/wallets/TiredClub.pem" \
    --gas-limit=80000000 \
    --send --outfile="deploy.interaction.json" \
    --proxy="https://gateway.elrond.com" --chain=1 || return
}

test() {
    mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqtjt0puryxc4c68qhf9v387j379phnwrm4jwspypwnt \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=200000000 \
    --function "sl" \
    --send --outfile="deploy.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}


test_mainnet() {
    mxpy --verbose contract call erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h \
    --recall-nonce --pem="~/wallets/TiredClub.pem" \
    --gas-limit=150000000 \
    --value 2000000000000000000 --function "distributeRoyaltiesSecondCollection" \
    --simulate --outfile="deploy.interaction.json" \
    --proxy="https://gateway.elrond.com" --chain=1 || return
}

