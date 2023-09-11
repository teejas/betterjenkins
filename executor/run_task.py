#!/usr/bin/python
import psycopg2
import subprocess
import os
from dotenv import load_dotenv

load_dotenv()

DB_NAME = os.getenv("DB_NAME")
DB_PASSWORD = os.getenv("DB_PASSWORD")
DB_USER = os.getenv("DB_USER")
DB_HOST = os.getenv("DB_HOST")

conn = psycopg2.connect(
  "dbname={} user={} host={} password={} port=5432".format(
    DB_NAME, DB_USER, DB_HOST, DB_PASSWORD
    )
  )

cur = conn.cursor()

cur.execute("SELECT * FROM jobs")

# Retrieve query results
records = cur.fetchall()
print(records[0])
job_name = records[0][1]
job_count = records[0][2]
job_name += "_" + str(records[0][2])
print(job_name)

cur.execute(
  """
  DELETE FROM tasks
  WHERE id IN
  (SELECT id FROM tasks
    WHERE job_name=%(str)s
    ORDER BY stage_number ASC
    LIMIT 1)
  RETURNING (definition);
  """, {'str': job_name }
  )

conn.commit()
task_def = cur.fetchall()[0][0]
# print(task_def) # name of task to execute

subprocess.run(task_def, shell=True, check=True)

# check if all tasks are done, if so delete job from jobs table
cur.execute(
  """
  SELECT COUNT(*) FROM tasks
    WHERE job_name=%(str)s
  """, {'str': job_name }
  )
task_count = cur.fetchall()[0][0]

print(task_count)
if task_count == 0:
  cur.execute(
  """
  DELETE FROM jobs
    WHERE name=%(str)s AND job_count=%(int)s
  """, {'str': job_name.split('_')[0], 'int': job_count}
  )
  conn.commit()

cur.close()
conn.close()