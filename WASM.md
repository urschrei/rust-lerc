# WebAssembly (WASM) Support for rust-lerc

This document describes how to compile the rust-lerc library to WebAssembly for use in browsers and Node.js.

## Prerequisites

1. **Rust toolchain** with WASM targets:
   ```bash
   rustup target add wasm32-unknown-emscripten
   ```

2. **Emscripten SDK** (emsdk):

   The `source` command adds it to your `$PATH`

   ```bash
   git clone https://github.com/emscripten-core/emsdk.git
   cd emsdk
   ./emsdk install latest
   ./emsdk activate latest
   source ./emsdk_env.sh
   ```

## Building for WASM

### Library Build

Build the library as a static library for WASM (no need to modify $PATH if the `source` command from the previous step was run):

```bash
export PATH="/path/to/emsdk:/path/to/emsdk/upstream/emscripten:$PATH"

CXX_wasm32_unknown_emscripten=em++ \
CC_wasm32_unknown_emscripten=emcc \
AR_wasm32_unknown_emscripten=emar \
cargo build --target wasm32-unknown-emscripten --release
```

### Example Binary (with JavaScript glue)

Build the example that produces both `.wasm` and `.js` files:

```bash
CXX_wasm32_unknown_emscripten=em++ \
CC_wasm32_unknown_emscripten=emcc \
AR_wasm32_unknown_emscripten=emar \
cargo build --example wasm_example --target wasm32-unknown-emscripten --release
```

Output files will be in:
- `target/wasm32-unknown-emscripten/release/examples/wasm_example.wasm`
- `target/wasm32-unknown-emscripten/release/examples/wasm_example.js`

### Running in Node.js (tested using v22.15.0)

```bash
node target/wasm32-unknown-emscripten/release/examples/wasm_example.js
```

## Architecture Notes

### Emscripten

The `wasm32-unknown-emscripten` target is required because:

1. **C++ Support**: The LERC library is written in C++ and requires:
   - C++ standard library headers (`<cstring>`, `<vector>`, etc.)
   - Exception handling support

2. **Limitations of `wasm32-unknown-unknown`**:
   - Cannot compile C/C++ code directly
   - No standard library support for C++

### Modifications

The patched `lerc-sys` crate includes WASM-specific modifications:

1. **Pre-generated Bindings**: For WASM targets, pre-generated FFI bindings are used since `bindgen` cannot parse headers for WASM targets.

2. **Exception Handling**: The C++ code requires exceptions, so `-fexceptions` is enabled for WASM builds.

3. **Type Mappings**: `std::os::raw` types are replaced with `core::ffi` equivalents for WASM compatibility.

## Output Sizes (Release Build)

Typical sizes for the example:
- `wasm_example.wasm`: ~487 KB
- `wasm_example.js`: ~64 KB (Emscripten glue code)

## Perf

WASM performance is comparable to native for LERC operations in my testing:
- Encoding: ~10x compression ratio for typical raster data
- Lossless round-trip with exact precision
- Lossy compression with configurable error bounds

## Current Limitations

1. **No wasm-bindgen**: The `wasm32-unknown-emscripten` target doesn't work well with wasm-bindgen. Use Emscripten's built-in JavaScript interop instead.
