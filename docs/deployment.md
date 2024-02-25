# Deployment
Deploying API Snap happens in 2 major steps:

* Compile & upload the docker image
* Package & upload the helm chart

Both are required for users to easily install the application in their infrastructure and vendors keep enough control for continuous updates + secure channels.

Deployment resources are located in directory [/deploy/](../deploy/).

## Docker
You need [docker](https://www.docker.com/products/docker-desktop/) to build the image and an account on [docker hub](https://hub.docker.com/) for sharing.

First build the image with fitting tags:
```sh
docker build -t youkadev/api-snap -t youkadev/api-snap:0.1.0 -f deploy/Dockerfile .
```
_(Replace `0.1.0` with the current application version.)_

The image should be tested:
* Run as temporary container:
  ```sh
  docker run -it --rm -p 8080:80 -v "$(pwd)/tests/invalid_kubeconfig.yml:/api-snap/kubeconfig.yml" -e KUBECONFIG=/api-snap/kubeconfig.yml --name api-snap youkadev/api-snap
  ```
  _(Replace `$(pwd)` with `%CD%` for Windows.)_
* Visit `http://localhost:8080` to validate the application is running and accessible

Create an access token in your docker hub account security and login locally:
```sh
docker login -u youkadev -p dckr_pat_C2...
```
* `youkadev` is the username and must be the prefix of the docker image
* `dckr_pat_C2...` is the (truncated) token used as password

Create a repository with description in docker hub. The name should equal your image (for this example `api-snap` with `youkadev` as account).

Now you can push the image up to docker hub:
```sh
docker push youkadev/api-snap
```
Do this for all new tags (in our example we also had `youkadev/api-snap:0.1.0`).

## Helm
You need [helm](https://helm.sh/) to package & index the chart and a **webserver** for sharing.

First package the chart:
```sh
helm package deploy/helm --destination tmp
```
A file `api-snap-<VERSION>.tgz` should be created in temporary directory `tmp`.

Next transform the directory to a helm repository by creating an index:
```sh
helm repo index tmp
```

Move the directory content to a static files webserver (for example `https://charts.youka.dev`). Share the url as your helm chart repository for potential users.

Users can now add the repository to their registry:
```sh
helm repo add youkadev https://charts.youka.dev
```

With the repository in scope the chart is installable to a cluster:
```sh
helm upgrade my-api-snap youkadev/api-snap --namespace=api-snap --create-namespace --install --atomic
```
