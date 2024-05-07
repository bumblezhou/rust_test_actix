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

## How to build a docker image:
```bash
docker build -t rust_test_actix .
```

## How to run the docker image:
```bash
docker run -it --name rust_test_actix_app_1 -p 8080:8080 rust_test_actix
CTRL+C
docker start rust_test_actix_app_1
```

## How to delete the docker container and docker image:
```bash
docker rm -f rust_test_actix_app_1
docker image rm -f rust_test_actix
```

[browser]
http://localhost:8080