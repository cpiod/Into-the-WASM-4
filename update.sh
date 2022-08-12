RUSTFLAGS="-C lto -C embed-bitcode=yes -C opt-level=z -C link-arg=--import-memory -C link-arg=--initial-memory=65536 -C link-arg=--max-memory=65536 -C link-arg=-zstack-size=14752" cargo build --release --package cart --target wasm32-unknown-unknown && twiggy top -n 20 target/wasm32-unknown-unknown/release/cart.wasm
wasm-snip --snip-rust-panicking-code target/wasm32-unknown-unknown/release/cart.wasm -o target/wasm32-unknown-unknown/release/cart.wasm
wasm-opt -Oz --zero-filled-memory --strip-producers --dce -o target/wasm32-unknown-unknown/release/cart.wasm target/wasm32-unknown-unknown/release/cart.wasm
wasm-strip -o target/wasm32-unknown-unknown/release/cart.wasm target/wasm32-unknown-unknown/release/cart.wasm
ls -alh target/wasm32-unknown-unknown/release/cart.wasm
