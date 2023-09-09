#!/usr/bin/python
import psycopg2
import subprocess

conn = psycopg2.connect("dbname=betterjenkins user=postgres host=localhost port=5432")

cur = conn.cursor()

cur.execute("SELECT (name) FROM jobs")

# Retrieve query results
records = cur.fetchall()
print(records[0][0]) # name of the job

cur.execute(
  """
  SELECT (definition) FROM tasks
  WHERE job_name=%(str)s
  ORDER BY stage_number ASC;
  """, {'str': records[0][0] }
  )

task_def = cur.fetchall()[0][0]
print(task_def) # name of task to execute

subprocess.run(task_def, shell=True, check=True)