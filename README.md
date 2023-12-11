# Benchmarking Database Performance with JMeter and Rust

## Overview
This repository contains a Rust application designed to benchmark the performance of ScyllaDB and MongoDB using JMeter for load testing. The application utilizes the Rocket framework for building REST APIs and follows the repository pattern for a structured and maintainable codebase.

## Usage
1. Clone the repository: `git clone https://github.com/AnuragProg/Scylla-Mongo-Benchmark.git`
2. Navigate to the project directory: `cd Scylla-Mongo-Benchmark`

### Running with Docker
3. **For ScyllaDB**: Run the application with ScyllaDB using Docker:
    ```bash
    docker-compose -f compose.scylla.yaml up
    ```
4. **For MongoDB**: Run the application with MongoDB using Docker:
    ```bash
    docker-compose -f compose.mongo.yaml up
    ```

### Running without Docker
3. Set environment variables:
   - **For MongoDB**: Set `MONGO_URI` variable.
   - **For ScyllaDB**: Set `SCYLLA_URI` variable.
4. Build the application: `cargo build --release`
5. Run the application: `cargo run --release`

## CLI Arguments
- Use the `-db` argument to specify the database (`scylla` or `mongo`).
  ```bash
  cargo run --release -- -db scylla
  ```

## Postman Collection
Explore and test the API endpoints using the [Postman Collection](link_to_your_postman_collection.json).

## JMeter Test Plan
Benchmark the performance with the [JMeter Test Plan](link_to_your_jmeter_test_plan.jmx).

## Benchmarking
1. Use JMeter to load test the API endpoints.
2. Analyze the performance metrics and compare ScyllaDB and MongoDB.

## Conclusion
The benchmarking results were inconclusive due to validation check failures when running ScyllaDB on localhost due to which performance of ScyllaDB was worse than that of MongoDB on read and write operations.

