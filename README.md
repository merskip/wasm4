# WASM-4

## Setup

### Example `.cargo/config.toml`

```toml
[alias]
cart-build = "build --profile cartridge"
cart-run = "run --profile cartridge"

[profile.cartridge]
inherits = "release"
opt-level = "z"
lto = true

[build]
target = "wasm32-unknown-unknown"

[target.wasm32-unknown-unknown]
runner = "w4 run"
rustflags = [
    # Import memory from WASM-4
    "-C", "link-arg=--import-memory",
    "-C", "link-arg=--initial-memory=65536",
    "-C", "link-arg=--max-memory=65536",

    # Reserve 8192 bytes of stack space, offset from 6560.
    # Bump this value, 16-byte aligned, if the framebuffer gets corrupted.
    "-C", "link-arg=-zstack-size=14752",
]
```
