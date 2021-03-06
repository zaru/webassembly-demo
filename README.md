# WebAssembly demo

## demo

Chrome 61+

### Rust

- https://zaru.github.io/webassembly-demo/rust-canvas/canvas.html
- https://zaru.github.io/webassembly-demo/rust-func/func.html
- https://zaru.github.io/webassembly-demo/rust-helloworld/helloworld.html

## C++

- https://zaru.github.io/webassembly-demo/cpp-helloworld/helloworld.html
- https://zaru.github.io/webassembly-demo/cpp-func/func.html

## Rust, Hello World

```
cd rust-helloworld
docker run --rm -v $(pwd):/src tomaka/rustc-emscripten rustc \
  --target wasm32-unknown-emscripten helloworld.rs -o helloworld.js
php -S 0.0.0.0:9999
```

## C++, Hello World

```
cd cpp-helloworld
docker run --rm -v $(pwd):/src trzeci/emscripten:sdk-tag-1.37.19-64bit \
  emcc helloworld.cpp -o helloworld.js -s WASM=1
php -S 0.0.0.0:9999
```

## Rust, call function

```
cd rust-func
docker run --rm -v $(pwd):/src tomaka/rustc-emscripten rustc \
  --target wasm32-unknown-emscripten func.rs -o func.js \
  -C link-args="-s EXPORTED_FUNCTIONS=['_add']"
php -S 0.0.0.0:9999
```

## C++, call function

```
cd cpp-func
docker run --rm -v $(pwd):/src trzeci/emscripten:sdk-tag-1.37.19-64bit \
  emcc func.cpp -o func.js -s WASM=1 \
  -s "EXPORTED_FUNCTIONS=['_add']"
php -S 0.0.0.0:9999
```

## Rust, `<canvas>`

```
cd rust-canvas
docker run --rm -v $(pwd):/src tomaka/rustc-emscripten rustc \
  --target wasm32-unknown-emscripten canvas.rs -o canvas.js \
  -C link-args="-s EXPORTED_FUNCTIONS=['_update']" \
  -C opt-level=3 -C debug_assertions=no
php -S 0.0.0.0:9999
```
