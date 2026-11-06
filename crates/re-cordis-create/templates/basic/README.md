# {{name}}

A small local Cordis application: a current-thread Tokio executor drives
`Runtime::run`; a typed loader plugin publishes a greeting service and logs it.
The application updates its typed config, reads the new service value, joins
loader disposal and the root drain, then checks Runtime cleanup diagnostics.

Run from this project directory:

```sh
cargo test --offline
cargo run --offline --locked
```

Rust 1.85 or newer is required. The first test resolves a Cargo.lock from cached
dependencies; if the cache is empty, run `cargo test` once with network access.
The generator itself performs no downloads or Cargo/Git processes.
The manifest contains canonical absolute paths to the explicit Cordis checkout.
Keep that checkout available; moving it requires editing those three paths.
The package is unpublished and has its own workspace boundary.

Edit `Settings`, `entry` and the greeting plugin in `src/app.rs` to build your
application. Preserve joined shutdown on both success and failure. Root cleanup
reports and `Runtime::take_failures` are separate error channels: a successful
joined waiter alone does not establish error-free cleanup.

The included tests exercise config update, service and writer release, cleanup
causes, and 100 deterministic lifecycle cycles. This basic template has no
file persistence, watcher, timer, signal loop, Git, install or run option.

## Template license

The generated starter contains MIT-licensed re-cordis template code. Retain
this notice when redistributing that code or substantial portions of it.
Your additions may have their own copyright notices.

MIT License

Copyright (c) 2026 re-cordis contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
