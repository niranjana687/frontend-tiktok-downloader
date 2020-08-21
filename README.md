# frontend-tiktok-downloader
Frontend for https://github.com/arjun069/tiktok-downloader-w-wm using Valerie

To run (nightly version of rust required, see <a href="https://doc.rust-lang.org/book/appendix-07-nightly-rust.html">docs</a>)

```sh
   $wasm-pack build --target web --out-name wasm --out-dir ./static
   $miniserve ./static --index index.html
```
