FROM python:3-slim

COPY ./ ./
RUN pip install -r requirements.txt

ENV DB_HOST=db DB_NAME=betterjenkins DB_PASSWORD=postgres DB_USER=postgres

CMD ["python", "run_task.py"]