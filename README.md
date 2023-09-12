# Introduction

I want to build a better version of Jenkins in Rust. Part of the project is learning and understanding Rust better, while also learning and understanding the requirements of a CI/CD software project.

# Architecture

`betterjenkins` runs best on Kubernetes but can be tested using docker-compose as explained below. The reason for it being more well-suited for container orchestration is because `betterjenkins` is itself a container orchestrator, specifically for executing tasks described as shell commands (i.e. [`/controller/examples/sample.yaml`](https://github.com/teejas/betterjenkins/blob/main/controller/examples/sample.yaml)), and a multi-service application. The permanent services `betterjenkins` runs is the controller, or `betterjenkins-server`, and the database server, or `betterjenkins-db`.

# Deploying

## Kubernetes [recommended]

Create a Kubernetes cluster anywhere (i.e. minikube, AWS, GCP, microk8s, etc), properly configure the `~/.kube/config` and set the current context to your desired cluster, make sure there are no existing namespaces called `betterjenkins`, then run `kubectl apply -k kustomize/` to deploy all the resources for betterjenkins.

Run `kubectl port-forward svc/betterjenkins-server 8080` to connect to the controller on http://localhost:8080. Similarly, the db can be connected to using `kubectl port-forward svc/betterjenkins-db 5432`.

An easy way to develop against a Kubernetes cluster is using [`mirrord`](https://mirrord.dev/) to connect local binaries to remote k8s resources such as pods or deployments. Run `cargo build && mirrord exec -t deployment/betterjenkins-db ./target/debug/betterjenkins` to get your local version of betterjenkins running against k8s resources (such as the Postgres server).

## docker-compose [for development only]
`docker-compose up` to spin up a Postgres server and betterjenkins controller.

The executor is run independent of the server and database, to build and run its image once the DB and controller are up run the following:
```
cd /executor
docker build -t betterjenkins:executor .
docker run --network betterjenkins_default betterjenkins:executor
```

# How it works

`betterjenkins` allows you to add tasks to be executed by defining them in task files, which further describe the order in which tasks should be executed. This will provide the basis for adding CI/CD capabilities to `betterjenkins`, allowing tasks to share access to a common workspace (such as files necessary to build an application binary).

Tasks to be executed are defined in task files such as those found in [`/controller/examples`](https://github.com/teejas/betterjenkins/tree/main/controller/examples). Once the steps for deploying are complete go to the controller main page at http://localhost:8080. Task files can be uploaded here, tasks are then parsed out of the file and added to the Postgres database, and executors are automatically on Kubernetes launched by the controller to complete all tasks in the database.

# To-do
1. Create a Rust program which can parse a file (i.e. toml or yaml) for tasks, then add those tasks to a Postgres DB [done]
2. Create a web server that allows users to drop .yaml files and add tasks to the queue. [done]
3. Create a simple Python program which takes tasks off the queue and executes them (tasks are just shell commands) [done]
4. Expand controller logic to interact with the Kubernetes control plane in order to automatically create new executor Jobs if there are any rows in the "tasks" table [done]
5. Allow for "workspaces" to share files across different tasks or stages of a job
   1. This would enable CI/CD capabilities where one stage builds a docker image using the files in a workspace, another stage runs tests agains that docker image, and a final stage deploys the image
   2. I'm thinking of some bucket integration, maybe use `minio` for testing. [in progress]
6. Have all the executors write their stdout somewhere, either back to the database or into some bucket storage [in progress]

# Database

## Tables
- `jobs`
  - Schema: (id, name, job_count [if there are duplicate instances of the same job], author, description)
- `tasks`
  - Schema: (id, job_name, stage_number, definition)


# Files
`/controller/src` contains all the Rust files, this includes the web server and logic for parsing yaml task files and enqueueing to the task (as a PostgreSQL database)

`/controller/web` contains any static files served by the web server

`/controller/examples` contains sample yaml task files and templates

`/controller/k8s_manifests` contains any K8s manifests the controller uses to create resources via the K8s API

`/executor` contains all files related to the task executor, this includes a Dockerfile to build the executor image and a Python script which simply takes a task from the "tasks" SQL table and runs it

`/kustomize` contains K8s manifests to deploy the betterjenkins and Postgres servers

# Contributing

This project welcomes contributions in the form of issues and pull requests. 

`betterjenkins` is lincensed under the [GNU General Public License v3.0](https://spdx.org/licenses/GPL-3.0-or-later.html).