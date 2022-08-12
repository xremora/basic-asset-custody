# Near Custody app

### Initialization of project

```bash
npx create-near-app --contract=rust custody-app
```

### Testing

```bash
cd contracts/
cargo test
```

### Build / compile the binary artifacts

- notice the target

```bash
env 'RUSTFLAGS=-C link-arg=-s' 
cargo build --target wasm32-unknown-unknown --release
```

### Features

1. Smart contract should take custody of the transactions
2. Timelock (receiver can only claim after certain time s)
3. Only supports native token(i.e. `NEAR`) for now (todo: extend to stablecoins(USDC) later)
4. Frontend for demonstration

### Steps

1. Alice (alice.testnet) sends tokens(`send_payment()`) to smart contract specifying `to: Bobs_address` & `after_time:unix_timestamp` paramters.
2. Smart contract takes the custory of the tokens before being claimed by Bob
3. After the specified time passes, Bob calls `accept_payment()`

### Extended features

- TODO

### structure of protocol

- use collections provided by `near-sdk` (`UnorderedMap` for rust projects)
- ref: <https://docs.near.org/concepts/storage/data-storage#rust-collection-types>

```
payment  {
    amount:
    from: 
    to: 
    time: 
}
```

### Methods / functions

- `send_payment(time, amount, to)` : Alice calls by specifying the parameters
- `accept_payment(amount)`: Bob most clim full amount at once, can't accept before the `time`
- `increase_payment_time()`: Alice can increase the payment time
- `cancel_payment()`: Alice can cancel the payment before being accepted by Bob.
- `get_payment_details()` : view function to get payment details : returns remaining `time`, `amount`, and `to`
  - this means every payment is to be represented by unique UUID ? (TODO) otherwise anyone can view anyone's details, limit to max payments details that can be retrieved ?

### Environment variables need

ref: <https://docs.near.org/develop/contracts/environment/>

- current caller : `env::predecessor_account_id()`
- msg.value: `env::attached_deposit()`
- current epoch : `env::epoch_height()`
- current timestamp : `env::block_timestamp()`

## TODOS

[ ] add time delayed payments (search with clock to use epoch or timestamp )
