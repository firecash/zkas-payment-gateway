# ZKas Payment Gateway

Merchant payment infrastructure built on the ZKas SDK. The design follows the
proven invoice/webhook model used by BTCPay Server and the unique-address model
used by privacy-coin gateways, adapted to Orchard and Kaspa BlockDAG finality.

- `core/` — invoice state machine, diversified addresses, idempotency, payment
  reconciliation, confirmation policy, and signed webhook events.
- `service/` — runnable HTTP API, checkout, wallet observer, persistence, and webhooks.
- `integrations/woocommerce/` — WooCommerce payment method.
- `integrations/web/` — generic browser checkout launcher.
- `docs/INTEGRATION.md` — short merchant integration guide and API flow.
- `docs/OPERATIONS.md` — self-hosting: env vars, TLS proxy, backups.
- `docs/ARCHITECTURE.md` / `docs/STATUS.md` — design and implemented scope.

## Build

```bash
cargo build --release -p zkas-gateway   # the merchant service binary
cargo test  -p zkas-payment-gateway     # the invoice state-machine core
```

The two chain crates it needs — `kaspa-addresses` (address encoding, runtime)
and `kaspa-shielded-core` (FVK helpers, tests only) — are git dependencies on
[`firecash-rusty`](https://github.com/firecash/zkas-rusty), so the gateway
always shares the node's exact address/keys format and can never drift from it.
Everything else is crates.io. Building against a private `firecash-rusty` needs
git credentials for that repo (e.g. a `url."https://TOKEN@github.com/".insteadOf`
config or an SSH remote).

## Run

See `docs/OPERATIONS.md`. In short: run a watch-only walletd for the merchant
FVK, set the `ZKAS_GATEWAY_*` env vars, and `cargo run --release -p zkas-gateway`
behind a TLS-terminating reverse proxy.
