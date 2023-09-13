#!/usr/bin/python
import io
import psycopg2
import subprocess
import os
from dotenv import load_dotenv
from minio import Minio
from minio.error import S3Error

load_dotenv()

DB_NAME = os.getenv("DB_NAME")
DB_PASSWORD = os.getenv("DB_PASSWORD")
DB_USER = os.getenv("DB_USER")
DB_HOST = os.getenv("DB_HOST")
MINIO_URL = os.getenv("MINIO_URL")
MINIO_ACCESS_KEY = os.getenv("MINIO_ACCESS_KEY")
MINIO_SECRET_KEY = os.getenv("MINIO_SECRET_KEY")

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
  RETURNING *
  """, {'str': job_name }
  )

conn.commit()
records = cur.fetchall()
if len(records) == 0:
  cur.execute(
    """
    DELETE FROM jobs
      WHERE name=%(str)s AND job_count=%(int)s
    """, {'str': job_name.split('_')[0], 'int': job_count}
    )
  conn.commit()

task_def = records[0][3]
stage_num = records[0][2]
print(task_def) # name of task to execute
print("stage number: " + str(stage_num))

m_client = Minio(
    MINIO_URL,
    access_key=MINIO_ACCESS_KEY,
    secret_key=MINIO_SECRET_KEY,
    secure=False
)

found = m_client.bucket_exists("betterjenkins")

result = subprocess.run(task_def, shell=True, check=True, capture_output=True, text=True)

buf = io.BytesIO(bytes("""
stdout: {}
----------------------------------------------------------------
stderr: {}
    """.format(result.stdout, result.stderr), 'utf-8'))

if found:
  m_client.put_object(
    bucket_name="betterjenkins", 
    object_name="/{}/{}/exec.log".format(job_name, str(stage_num)),
    data=buf,
    length=buf.getbuffer().nbytes
    )

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