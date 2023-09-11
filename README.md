# Introduction

I want to build a better version of Jenkins in Rust. Part of the project is learning and understanding Rust better, while also learning and understanding the requirements of a CI/CD software project.

Going to use PostgreSQL instead of Redis because it gives more flexibility in terms of having multiple tables for related dawta, one for jobs and one for tasks (stages of a job) to be run.

# Steps
1. Create a Rust program which can parse a file (i.e. toml or yaml) for tasks, then add those tasks to a Postgres DB [done]
2. Create a web server that allows users to drop .yaml files and add tasks to the queue. [done]
3. Create a simple Python program which takes tasks off the queue and executes them (tasks are just shell commands) [done]
4. Expand controller logic to interact with the Kubernetes control plane in order to automatically create new executor Jobs if there are any rows in the "tasks" table [done]
5. Allow for "workspaces" to share files across different tasks or stages of a job
   1. This would enable CI/CD capabilities where one stage builds a docker image using the files in a workspace, another stage runs tests agains that docker image, and a final stage deploys the image
   2. I'm thinking of some bucket integration, maybe use `minio` for testing. [in progress]

# Database

## Tables
- Jobs (`jobs`)
  - Schema: (id, name, job_count [if there are duplicate instances of the same job], author, description)
- Tasks (`tasks`)
  - Schema: (id, job_name, stage_number, definition)


# Files
`/controller/src` contains all the Rust files, this includes the web server and logic for parsing yaml task files and enqueueing to the task (as a PostgreSQL database)

`/controller/web` contains any static files served by the web server

`/controller/examples` contains sample yaml task files and templates

`/controller/k8s_manifests` contains any K8s manifests the controller uses to create resources via the K8s API

`/executor` contains all files related to the task executor, this includes a Dockerfile to build the executor image and a shell script which simply takes a task from the "tasks" SQL table and runs it

`/kustomize` contains K8s manifests to deploy the betterjenkins and Postgres servers

# Useful Commands

`docker-compose up` to spin up a Postgres server and betterjenkins controller.

The executor is run independent of the server and database, to build and run its image once the DB and controller are up run the following:
```
cd /executor
docker build -t betterjenkins:executor .
docker run --network betterjenkins_default betterjenkins:executor
```