# Troubleshooting
* > Pod logs mention "Environment variable 'API_SNAP_PORT=tcp://10.104.149.15:80': invalid digit found in string"!
  
  Kubernetes sets its own [environment variables for pods](https://kubernetes.io/docs/concepts/containers/container-environment/).  
  Don't name the release `api-snap` so kubernetes doesn't override environment variable `API_SNAP_PORT`!
