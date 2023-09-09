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

cur.execute("SELECT (name) FROM jobs")

# Retrieve query results
records = cur.fetchall()

cur.execute(
  """
  DELETE FROM tasks
  WHERE id IN
  (SELECT id FROM tasks
    WHERE job_name=%(str)s
    ORDER BY stage_number ASC
    LIMIT 1)
  RETURNING (definition);
  """, {'str': records[0][0] }
  )

conn.commit()
task_def = cur.fetchall()[0][0]
# print(task_def) # name of task to execute

subprocess.run(task_def, shell=True, check=True)

cur.close()
conn.close()