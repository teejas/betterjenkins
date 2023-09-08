# Introduction

I want to build a better version of Jenkins in Rust. Part of the project is learning and understanding Rust better, while also learning and understanding the requirements of a CI/CD software project.

Going to use PostgreSQL instead of Redis because it gives more flexibility in terms of having multiple tables, one for jobs and one for tasks to be run.

## Steps
1. Create a Rust program which can parse a file (i.e. toml or yaml) for tasks, then add those tasks to a Redis task queue
2. Create a web server that allows users to drop .yaml files and add tasks to the queue.
3. Create a Rust program which takes tasks off the queue and executes them (tasks are just shell commands)

## Database

### Tables
- Jobs (`jobs`)
  - Schema: (job_name, author, description)
- Tasks (`tasks`)
  - Schema: (job_name, stage_number, definition)


## Files