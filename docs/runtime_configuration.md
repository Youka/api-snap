# Runtime configuration
Configuring the runtime behaviour of API Snap is possible by **environment variables**.

## Server address
* **Environment variable:** `API_SNAP_ADDRESS`
* **Default value:** `127.0.0.1`
* **Description:** Address of the internal web server to listen. Set to `0.0.0.0` for external access.

## Server port
* **Environment variable:** `API_SNAP_PORT`
* **Default value:** `8080`
* **Description:** Port of the internal web server to listen. Traffic is always http (unsecured) so ports like `80` or `8080` are recommended values.

## Client timeout
* **Environment variable:** `API_SNAP_CLIENT_TIMEOUT`
* **Default value:** `30`
* **Description:** Timeout in seconds until a client cancels a connection try. Set a low value so the application threads aren't blocked for long when fetching content but high enough to give slow servers some time.

## Cache lifespan
* **Environment variable:** `API_SNAP_CACHE_LIFESPAN`
* **Default value:** `10`
* **Description:** Duration in seconds until a cache refreshes. Set a low value so users don't get old content delivered but high enough to reduce client load.

## Logging
* **Environment variable:** `RUST_LOG`
* **Default value:** _undefined_
* **Description:** Define logging level, target, etc. See [env_logger](https://docs.rs/env_logger/latest/env_logger/).

## Kubernetes client configuration
* **Environment variable:** `KUBECONFIG`
* **Default value:** _undefined_
* **Description:** Set the path to the kubernetes client configuration file. See [the kubernetes concept](https://kubernetes.io/docs/concepts/configuration/organize-cluster-access-kubeconfig/#the-kubeconfig-environment-variable).
