
# This game is made with ❤️: and the following tools:
  - [Rust](https://www.rust-lang.org/), a language empowering everyone to build reliable and efficient software.
  - [Bevy](https://bevyengine.org/), a refreshingly simple data-driven game engine built in Rust.
  - [LDTK](https://ldtk.io/), A modern 2D level editor with a strong focus on user-friendliness.
  - [kurbos/bevy-shell-template](https://github.com/kurbos/bevy-shell-template), a template for Bevy with cross-platform CI/CD and native & WASM launchers.


## 📦 Multi-platform CI/CD
We use GitOps to deploy a multi-platform release.
GitHub Actions powers [testing pipelines](https://github.com/adamazing/combine/actions) and [release deployment](https://github.com/adamazing/combine/releases).

### Automated testing
Pushing to the main branch automatically triggers CI pipelines:
- Unit Testing - Ubuntu (latest)
- Build - Ubuntu (latest)
- ~Build - Windows (latest)~
- Build - MacOS (latest)
- Build - WebAssembly

⚠️ Github merge protection is turned on to prevent merging failing PRs to main.

### Automated dependency management
[Renovate](https://github.com/marketplace/renovate) is used to automatically create pull requests for dependency updates. The [renovate settings file](.github/renovate.json) is configured to auto-merge minor and patch updates providing all CI tests pass.

### Release cutting
Creating a tagged release will trigger the release pipeline and create [packages](https://github.com/adamazing/combine/releases/latest) for each of:
  - MacOS
  - Linux; and
  - Windows

It will also create/update the branch `gh-pages` with the WASM bundle to be served by Github Pages: [demo](https://adamazing.github.io/combine).

⚠️  Releases use semantic versioning, e.g. `v0.1.2`, not `v1` or `1.2`

### Local testing

#### WASM
The WASM launcher depends on the [trunk](https://trunkrs.dev/) crate.

To build and run the WASM app locally:
> Serve with `trunk serve` and open [`http://127.0.0.1:8080`](http://127.0.0.1:8080) in your browser
- Assets are streamed through the hosting provider, so that the initial WASM bundle is smaller.
- We use all the WASM optimizations discussed described [here](https://rustwasm.github.io/book/reference/code-size.html) in the Rust and WebAssembly book.
- There is an initial loading screen provided through [Yew](https://yew.rs) while the WASM bundle loads.

#### Native (Windows, MacOS, Linux)
> Run with `cargo run`
- Assets are bundled with the release when cut.
- There is no loading screen.

