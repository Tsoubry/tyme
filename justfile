default: release

fmt:
    cargo +nightly fmt

serve:
    trunk serve

build:
    trunk build

release:
    trunk build --release

check:
    cargo check --target wasm32-unknown-unknown

check-fmt:
    cargo +nightly fmt --all -- --check

clippy:
    cargo clippy --target wasm32-unknown-unknown -- -D warnings

test:
    cargo test -- --nocapture

check-all: check check-fmt clippy test
