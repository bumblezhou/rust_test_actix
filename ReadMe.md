# A simple web app to demostrate how to use rust and actix

## How to install rust on Linux
```bash
curl -x your_proxy_setting --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
```

## How to build
```bash
cargo build
```

## How to run
```bash
cargo build
```

## How to release
```bash
cargo build --release
cp -r ./static/ ./target/release/
cp -r ./templates/ ./target/release/
```

[browser]
http://localhost:8080