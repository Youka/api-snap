# API Snap

| ![Logo](docs/logo.svg) | An application which collects API documents by kubernetes service discovery and displays them in a web interface. |
|---|---|

[![Artifact Hub](https://img.shields.io/endpoint?url=https://artifacthub.io/badge/repository/youkadev)](https://artifacthub.io/packages/helm/youkadev/api-snap/)
[![Docker Size](https://badgen.net/docker/size/youkadev/api-snap?icon=docker&label=Docker%20Size&color=blue)](https://hub.docker.com/r/youkadev/api-snap)
[![GitHub Latest Tag](https://badgen.net/github/tag/youka/api-snap?icon=github&label=Latest%20Tag&color=black)](https://github.com/Youka/api-snap/tags)  
[![GitHub License](https://badgen.net/github/license/micromatch/micromatch?icon=github&label=License&color=green)](./LICENSE)
[![GitHub Actions](https://github.com/youka/api-snap/actions/workflows/main.yml/badge.svg)](https://github.com/Youka/api-snap/actions)
[![GitHub Last Commit](https://badgen.net/github/last-commit/youka/api-snap?icon=github&label=Last%20Commit)](https://github.com/Youka/api-snap/commits)

## Quickstart
_API Snap is mainly used by kubernetes cluster administrators. Experience is expected._  
See [ArtifactHUB](https://artifacthub.io/packages/helm/youkadev/api-snap/) how to add the helm repository and customize chart installations.

For a simple installation just two commands are required:
```sh
helm repo add youkadev https://charts.youka.dev
helm upgrade latest youkadev/api-snap --namespace=api-snap --create-namespace --install --atomic
```

**Preview:** [User interfaces](./docs/user_interfaces.md)

## Configuration
* How to use the application: [Kubernetes setup](./docs/kubernetes_setup.md)
* How to configure the application: [Runtime configuration](./docs/runtime_configuration.md)

## Development
* How to work on this project: [Development](./docs/development.md)
* How to deploy artifacts of this project: [Deployment](./docs/deployment.md)

## Troubleshooting
See [Troubleshooting](./docs/troubleshooting.md).

## Contributing
See [CONTRIBUTING](./CONTRIBUTING.md).

## License
See [LICENSE](./LICENSE).

## Changelog
See [CHANGELOG](./CHANGELOG.md).
