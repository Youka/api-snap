# Kubernetes setup
API Snap operates in a kubernetes cluster. Configuring cluster resources to work with this application is therefore required.

## Service exposure
API Snap isn't intended to work just internal. Its **service** should be exposed for external users to visit the web interface (see `/` for an overview).  
Have a look at kubernetes resource types [Service](https://kubernetes.io/docs/concepts/services-networking/service/) and [Ingress](https://kubernetes.io/docs/concepts/services-networking/ingress/) which ways exist. The application's helm chart allows variable overwriting to accomplish this.

## API discovery
API Snap looks for documentation by http endpoints. These are discovered by annotations on kubernetes services along the whole cluster.

Documents and how they're displayed differs by api type. Currently 3 of those are supported:

* [OpenAPI](https://www.openapis.org)
* [AsyncAPI](https://www.asyncapi.com)
* [GraphQL](https://graphql.org/)

Annotations are prefixed by their lowercase name:

```
openapi/???
asyncapi/???
graphql/???
```

An endpoint is defined by the service itself plus a port and path which are annotation postfixes:

```
???/port
???/path
```

A service needs **at least one** annotation to be discovered by API Snap!  
Example:

```
apiVersion: v1
kind: Service
metadata:
  name: petstore-provider
  namespace: test
  annotations:
    openapi/path: /petstore.yml
  ...
spec:
  ...
```

Default port is `80`.  
Default path is the api type lowercase name (`/openapi`, `/asyncapi`, `/graphql`).
