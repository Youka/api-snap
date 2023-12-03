#!/bin/sh

# Versions
SWAGGER_UI_VERSION='5.10.5'
ASYNCAPI_REACT_VERSION='1.2.11'

# Download swagger-ui
curl -L "https://github.com/swagger-api/swagger-ui/archive/refs/tags/v$SWAGGER_UI_VERSION.tar.gz" | tar -xz "swagger-ui-$SWAGGER_UI_VERSION/dist/" --strip-components 1
mkdir swagger-ui || rm swagger-ui/*
mv dist/* swagger-ui/
rmdir dist

# Download asyncapi-react
mkdir asyncapi-react
curl -o asyncapi-react/default.min.css "https://unpkg.com/@asyncapi/react-component@$ASYNCAPI_REACT_VERSION/styles/default.min.css"
curl -o asyncapi-react/index.js "https://unpkg.com/@asyncapi/react-component@$ASYNCAPI_REACT_VERSION/browser/standalone/index.js"
curl -o asyncapi-react/favicon.ico https://www.asyncapi.com/favicon.ico
