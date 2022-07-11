# commands
- cargo install wasm-server-runner
- cargo install wasm-bindgen
- lld linux: sudo apt-get install lld
- lld windows: cargo install -f cargo-binutils; rustup component add llvm-tools-preview
- deps linux: sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
- refresh github web build: cargo build --release --target wasm32-unknown-unknown; wasm-bindgen --out-dir ./docs/ --target web ./target/wasm32-unknown-unknown/release/renameme.wasm
