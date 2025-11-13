✅ Summary

Service1 is a Rust-based microservice that handles medication data, stores records in a PostgreSQL database, and sends event messages to service2 through RabbitMQ.

✅ Description

Service1 is a Rust microservice responsible for managing medication entities within the system. It provides:

• GET endpoint for retrieving information about a specific medication from the database instace postgres1 enabled through docker-compose.
• POST endpoint for inserting a new medication record into the postgres1 database
• RabbitMQ integration, acting as a sender that publishes messages to the message broker after successful operations (e.g., creating a medication). The sender is always deleted after completion!. (For better results we need an AppState for the sender and receiver).