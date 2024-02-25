# Development
API Snap is a [rust](https://www.rust-lang.org/) project with **web assets** and **kubernetes clusters** as target.  
This document mentions basic tooling and major commands to work with it.

## Tools
* [VSCode](https://code.visualstudio.com/)  
  The recommended IDE (but optional). Also install **recommended plugins**.
* [Rust](https://www.rust-lang.org/tools/install)  
  Install the rust toolchain with **rustup** including the component `clippy`.
* [Docker Desktop](https://www.docker.com/products/docker-desktop/)  
  For containerization and kubernetes install this convenient toolkit.
* [Helm](https://helm.sh/)  
  Install this small tool to apply & package charts for kubernetes deployment.

## Commands

### Quality Assurance
* Lint code:  
  `cargo clippy`
* Prepare test environment:  
  ```sh
  kubectl apply -f tests/k8s_test_namespaces.yml
  kubectl apply -f tests/k8s_test_resources.yml
  ```
* Run tests:  
  `cargo test`
* Install code coverage tool:  
  `cargo install cargo-tarpaulin`
* Report code coverage:  
  `cargo tarpaulin --out html --output-dir target --engine llvm --skip-clean`
* Render chart templates:  
  `helm template deploy/helm`

### Build
* Update dependencies:  
  `cargo update`
* Run application:  
  `cargo run`
* Build application (optimized):  
  `cargo build --release --no-default-features --locked`

### More
See [deployment](./deployment.md) for further commands.
