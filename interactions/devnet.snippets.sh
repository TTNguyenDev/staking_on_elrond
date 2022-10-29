USER_PEM="./wallet/wallet-owner.pem"
PROXY="https://devnet-gateway.elrond.com"
CHAIN_ID="D"

USER_ADDRESS=erd1uj9kavql5s9ayt8tz73xdjsar9stksmjvdvd7u3un9d5yrzvdynsmm4myt
SC_ADDRESS=erd1qqqqqqqqqqqqqpgqn0a6624ycfxwhcdvfppl726puhs8z8xrdyns9j03v4
STAKE_AMOUNT=1000

deploy() {
  arch -x86_64 erdpy --verbose contract deploy --project=${PROJECT} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=10000000 \
    --send --outfile="deploy-devnet.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return 
}

stake() {
  arch -x86_64 erdpy --verbose contract call ${SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} \
    --send --recall-nonce --pem=${USER_PEM} \
    --gas-limit=10000000 \
    --value=${STAKE_AMOUNT} \
    --function="stake"
}

getStakeForAddress() {
  arch -x86_64 erdpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStakingPosition" \
    --arguments ${USER_ADDRESS}
}

getAllStakers() {
  arch -x86_64 erdpy --verbose contract query ${SC_ADDRESS} \
    --proxy=${PROXY} \
    --function="getStakedAddresses" 
}
