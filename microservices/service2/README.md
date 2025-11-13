✅ Summary

Service2 is a Rust-based microservice that retrieves data from its postgres2 instance database, consumes messages from RabbitMQ, and stores received events into the postgres2 database.

✅ Description

Service2 is a Rust microservice responsible for processing and storing event-related data. It provides:

GET endpoint for fetching records from the Postgres2 database

RabbitMQ consumer that listens for messages sent by other services (e.g., Service1)

Database integration that inserts consumed RabbitMQ messages into the Postgres2 database for persistent storage

The service is implemented using:

Rocket as the web API framework

Diesel for structured database interaction

RabbitMQ for message consumption and event-driven communication