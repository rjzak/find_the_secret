# Find the Secret!

This application is very simple, it generates a UUID (the secret) and wants you to find it! Do a simple HTTP GET with the correct UUID, and the server will respond with either `Secret validated!`, or `The secret is still safe!`. The purpose is to demonstrate the capabilities of [Enarx](https://github.com/enarx/enarx/), which keeps application code and data encrypted in memory (a Keep) when running with Intel [SGX](https://www.intel.com/content/www/us/en/architecture-and-technology/software-guard-extensions.html) or AMD [SEV](https://developer.amd.com/sev/). Use this application with a memory dump utility when run with Enarx to see if you can get the secret!

## Testing as native code
Compiling in debug mode, `cargo build`, shows the secret when the program runs, so you can validate that the application is truthful. Re-compile with `cargo build --release` and run with Enarx on SGX/SEV and see if you can find the secret!

* `cargo run`
* Get the secret from output
* `curl -i http://localhost:8080/<secret-here>` and see a confirmation that the secret was correct.
* Close and re-compile as `cargo build --release`
* Find the secret?
* Use the same `curl` command to check your answer.

## Testing with Enarx & WebAssembly
Enarx uses WebAssembly to run applications, so recompile with the additional argument `--target=wasm32-wasi`. Run `rustup target add wasm32-wasi` if you haven't compiled Rust applications for WebAssembly previously.
* Install Enarx from https://github.com/enarx/enarx/releases/latest.
* You should then be able to run `cargo run --target=wasm32-wasi`, or `cargo run --release --target=wasm32-wasi` for release mode.
* If Enarx isn't in your $PATH, compile using the commands above, and run as `/path/to/enarx run --wasmcfgfile Enarx.toml target/wasm32-wasi/release/find_the_secret.wasm`.
* Add to Enarx `--backend=sgx` or `--backend=sev` depending on your hardware.
* Running `enarx platform info` shows if Enarx detected or supports your system.
