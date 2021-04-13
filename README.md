# OCT token

## Building

To build run:

```bash
./build.sh
```

## Testing

To test run:

```bash
cargo test --package oct-token -- --nocapture
```

## Deploy

To deploy run:

```bash
near dev-deploy
```

Init contract:

```
near call $TOKEN_CONTRACT_ID new '{"owner_id": "'$CONTRACT_ID'", "total_supply": "100000000000000000000000000000000", "metadata": {"spec": "ft-1.0.0", "name": "OCTToken", "symbol": "OCT", "decimals": 24}}' --accountId YOUR_ID
```
