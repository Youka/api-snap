# API Snap
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

### Docker
* `docker build -t youka/api-snap:0.1.0 -f deploy/Dockerfile .`
* `docker run -it --rm -p 8080:80 --name api-snap youka/api-snap:0.1.0`

### K8s
* ```sh
  kubectl apply -f tests/k8s_test_namespaces.yml
  kubectl apply -f tests/k8s_test_resources.yml
  ```
* `helm template deploy/helm`
* `helm upgrade my-release ./deploy/helm --namespace=api-snap --create-namespace --install --atomic`

### Environment variables
* `API_SNAP_ADDRESS`
* `API_SNAP_PORT`
* `API_SNAP_CLIENT_TIMEOUT`
* `API_SNAP_CACHE_LIFESPAN`
* `RUST_LOG`
* `KUBECONFIG`

## References
_TODO_
