# Rust on Kubernetes with Minikube

This repository provides a guide on deploying and scaling a Rust application on Kubernetes using Minikube.

## Prerequisites

- [Minikube](https://minikube.sigs.k8s.io/docs/start/)
- [Docker](https://docs.docker.com/get-docker/)
- [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl/)


## Setup

1. Start Minikube cluster:
    ```bash
    minikube start
    ```

2. Set up Docker environment to use Minikube's Docker daemon:
    ```bash
    eval $(minikube docker-env)
    ```

3. Get Minikube IP address and replace it in deployment.yaml file, line 17:
    ```bash
    minikube ip
    ```

4. Enable Metrics in Minikube:
    ```bash
    minikube addons enable metrics-server
    ```

## Building and Deploying

1. Build Docker image:
    ```bash
    docker build --tag $(minikube ip):5000/hello-world-rust .
    ```

4. Apply deployment and service configurations:
    ```bash
    kubectl apply -f deployment.yaml
    kubectl apply -f service.yaml
    ```

## Scaling

1. Set up Horizontal Pod Autoscaler (HPA):
    ```bash
    kubectl apply -f hpa.yaml
    ```

## Load Testing

2. Apply load generator deployment:
    ```bash
    kubectl apply -f load-generator.yaml
    ```

## Monitoring

1. Monitor the HPA to see the Increase in CPU utilization:
    ```bash
    kubectl get hpa hello-world-hpa --watch
    ```

2. Wait a few minutes until the Autoscaler scale up the deployment, then check the number of new pods creted:
    ```bash
    kubectl get pods
    ```

2. Once the HPA testing is done, delete load-generator to scale down the number of pods of the deployment:
    ```bash
    kubectl delete deployment load-generator
    ```

## Vulnerability Scanning with trivy

To ensure the security of the Docker image, this repository includes a GitHub Actions workflow for continuous integration of Docker images. The workflow is triggered on pushes and pull requests to the `main` branch. It builds the Docker image and scans it for vulnerabilities using the open source software "Aqua Security's Trivy scanner".

Here's the workflow configuration located in .github/workflows/docker-image.yaml:

```yaml
name: Docker Image CI

on:
  push:
    branches: [ "main" ]
  pull_request:
    branches: [ "main" ]

jobs:

  test:

    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v4
    - name: Test the Docker image
      run: docker build . --file Dockerfile --tag hello-world-rust &amp;&amp; docker run -v /var/run/docker.sock:/var/run/docker.sock aquasec/trivy image hello-world-rust:latest
```
