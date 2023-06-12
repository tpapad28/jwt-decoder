RUSTFLAGS="-C target-feature=+crt-static" cargo build --profile static --target x86_64-unknown-linux-gnu
upx --best target/x86_64-unknown-linux-gnu/static/jwt-decode
ls -lh target/x86_64-unknown-linux-gnu/static/jwt-decode
