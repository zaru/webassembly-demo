# WebAssembly demo

## Rust

```
docker run --rm -v $(pwd):/src tomaka/rustc-emscripten rustc --target wasm32-unknown-emscripten helloworld.rs -o helloworld.js
php -S 0.0.0.0:9999
open http://0.0.0.0:9999/helloworld.html
```
