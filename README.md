# API Snap
![Logo](docs/logo.svg)  
An application which collects API documents by kubernetes service discovery and displays them in a web interface.

## Requirements
_TODO_

## Usage
_TODO_

### QA
* `cargo clippy`
* `cargo test`
* ```sh
  cargo install cargo-tarpaulin
  cargo tarpaulin --out html --output-dir target --engine llvm --skip-clean
  ```

### Build
* `cargo update`
* `cargo run`
* `cargo build --release --no-default-features --locked`

### K8s
* ```sh
  kubectl apply -f tests/k8s_test_namespaces.yml
  kubectl apply -f tests/k8s_test_resources.yml
  ```

## References
_TODO_
