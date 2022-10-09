USER_PEM="~/wallets/development.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"

deploy() {
    erdpy --verbose contract deploy --project=${PROJECT} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=20000000 \
    --arguments "str:TACC-73857e" \
    --send --outfile="deploy.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

build() {
    erdpy contract build
}

getUserStaked() {
    erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqtjt0puryxc4c68qhf9v387j379phnwrm4jwspypwnt \
    --function "getUserStakedFirstCollection" --arguments erd1e9nnlhee5rvn9k8y3ysmtsx490h8nw9jels5vjdlezl72yrtg3yq6vvj3r \
    --proxy=${PROXY} || return
}

getUserUnStaked() {
    erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqtjt0puryxc4c68qhf9v387j379phnwrm4jwspypwnt \
    --function "getUserUnstakedFirstCollection" --arguments erd1e9nnlhee5rvn9k8y3ysmtsx490h8nw9jels5vjdlezl72yrtg3yq6vvj3r \
    --proxy=${PROXY} || return
}

getUsersStaked() {
    erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqtjt0puryxc4c68qhf9v387j379phnwrm4jwspypwnt \
    --function "getUsersStakedFirstCollection" \
    --proxy=${PROXY} || return
}

getNumberUnstaked() {
    erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqtjt0puryxc4c68qhf9v387j379phnwrm4jwspypwnt \
    --function "getNumberUnstakedFirstCollection" \
    --proxy=${PROXY} || return
}

getUserStaked_second() {
    erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h \
    --function "getUserStakedSecondCollection" --arguments erd1xhxylx0rjefy578hkje5rg9xzlxweq07frprzujgvd0cxant2wnqte3jfs \
    --proxy=${PROXY} || return
}

getUsersStaked_second() {
    erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h \
    --function "getUsersStakedSecondCollection" \
    --proxy=${PROXY} || return
}

upgrade() {
    erdpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgqtjt0puryxc4c68qhf9v387j379phnwrm4jwspypwnt --project=${PROJECT} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=40000000 \
    --send --outfile="deploy.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

upgrade_mainnet() {
    erdpy --verbose contract upgrade erd1qqqqqqqqqqqqqpgqwjv6ru86mmlgvad54alm62xay0st5n5f4yuqdky79h --project=${PROJECT} \
    --recall-nonce --pem="~/wallets/TiredApeClub.pem" \
    --gas-limit=40000000 \
    --arguments str:TACC-73857e str:TIREDCLUB-b8cfb8 \
    --send --outfile="deploy.interaction.json" \
    --proxy="https://gateway.elrond.com" --chain=1 || return
}

test_Airdrop() {
    erdpy contract call erd19hcnc2djsjay3prvhuzr0phveducv93khj435pqjza73tcyu4jwsuqywdh \
    --recall-nonce --pem=${USER_PEM} \
    --value 0 --gas-limit=20000000 \
    --function "ESDTNFTTransfer" \
    --arguments str:HAHAHA-5c4481 198 1 erd1qqqqqqqqqqqqqpgqtjt0puryxc4c68qhf9v387j379phnwrm4jwspypwnt str:airdropSecond erd1e9nnlhee5rvn9k8y3ysmtsx490h8nw9jels5vjdlezl72yrtg3yq6vvj3r \
    --send --outfile="deploy.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

