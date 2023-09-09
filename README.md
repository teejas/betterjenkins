# Introduction

I want to build a better version of Jenkins in Rust. Part of the project is learning and understanding Rust better, while also learning and understanding the requirements of a CI/CD software project.

Going to use PostgreSQL instead of Redis because it gives more flexibility in terms of having multiple tables, one for jobs and one for tasks to be run.

# Steps
1. Create a Rust program which can parse a file (i.e. toml or yaml) for tasks, then add those tasks to a Redis task queue
2. Create a web server that allows users to drop .yaml files and add tasks to the queue.
3. Create a Rust program which takes tasks off the queue and executes them (tasks are just shell commands)
   1. first iteration is just going to be a shell script

# Database

## Tables
- Jobs (`jobs`)
  - Schema: (id, job_name, author, description)
- Tasks (`tasks`)
  - Schema: (id, job_name, stage_number, definition)


# Files
`/src` contains all the Rust files, this includes the web server and logic for parsing yaml task files and enqueueing to the task (as a PostgreSQL database)

`/web` contains any static files served by the web server

`/executor` contains all files related to the task executor, this includes a Dockerfile to build the executor image and a shell script which simply takes a task from the "tasks" SQL table and runs it

`/examples` contains sample yaml task files and templates

# Useful Commands

The executor is run independent of the server and database, to build and run its image run the following:
```
cd /executor
docker build -t betterjenkins:executor .
docker run --network betterjenkins_default betterjenkins:executor
```