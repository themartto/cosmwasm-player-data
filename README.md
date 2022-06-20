# cosmwasm-player-data

## Start a Chain

```
./build/mantrachaind keys add validator --keyring-backend test

MY_VALIDATOR_ADDRESS=$(./build/mantrachaind keys show validator -a --keyring-backend test)

./build/mantrachaind init mantrachain --chain-id mantrachain

./build/mantrachaind add-genesis-account $MY_VALIDATOR_ADDRESS 100000000000stake

./build/mantrachaind gentx validator 100000000stake --chain-id mantrachain --keyring-backend test

./build/mantrachaind collect-gentxs

./build/mantrachaind start
```

## Compile wasm

```
cargo wasm

cargo schema

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer-arm64:0.12.6
```
## Deploy

```
mantrachain tx wasm store ./my_first_contract.wasm --from validator --chain-id mantrachain --keyring-backend test --gas 10000000 -y --output json -b block

mantrachain query wasm list-code

mantrachain tx wasm instantiate 1 "{}" --from validator --label "woo" --chain-id mantrachain --keyring-backend test --gas auto --gas-adjustment 1.3 --no-admin

mantrachain query wasm list-contract-by-code 1 --output json
```

## Frontend

```
npm install
npm run serve
```
