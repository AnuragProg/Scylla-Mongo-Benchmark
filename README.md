# Benchmarking Database Performance with JMeter and Rust

## Overview
This repository contains a Rust application designed to benchmark the performance of ScyllaDB and MongoDB using JMeter for load testing. The application utilizes the Rocket framework for building REST APIs and follows the repository pattern for a structured and maintainable codebase.

## Folder Structure
- **src**
  - **clients**: Contains database clients for ScyllaDB and MongoDB.
  - **models**: Defines data models and request structures.
  - **repository**: Implements the repository pattern for database operations.
  - **routes**: Specifies REST API routes.
- **target**: Rust target directory.
- **.dockerignore**: Docker ignore file.
- **.gitignore**: Git ignore file.
- **Benchmark Test Plan.jmx**: JMeter test plan for benchmarking.
- **Cargo.lock**: Cargo lock file.
- **Cargo.toml**: Cargo configuration file.
- **Dockerfile**: Docker configuration file.
- **README.md**: Project documentation.
- **compose.mongo.yaml**: Docker Compose file for MongoDB.
- **compose.scylla.yaml**: Docker Compose file for ScyllaDB.

## Usage
1. Clone the repository: `git clone https://github.com/yourusername/benchmark-db-rust.git`
2. Navigate to the project directory: `cd benchmark-db-rust`
3. Build the application: `cargo build --release`
4. Run the application: `cargo run --release`
5. Access the REST API at `http://localhost:3000`

## CLI Arguments
- Use the `-db` argument to specify the database (`scylla` or `mongo`).
  ```bash
  cargo run --release -- -db scylla
  ```

## API Endpoints
- `/users/save`: Save user data.
- `/users/get`: Retrieve user data.
- `/health`: Check the health of the application.

## Routes
### Get Users
```rust
#[get("/?<query..>")]
pub async fn get_users(query: UserQuery, user_repository: &rocket::State<Box<dyn UserRepository>>) -> Custom<RawJson<String>>{
    // Route implementation code here
}
```

### Save Users
```rust
#[post("/", data="<users>", format="json")]
pub async fn save_users(user_repository: &rocket::State<Box<dyn UserRepository>>, users: Json<UserInsertRequests>) -> Custom<RawJson<String>>{
    // Route implementation code here
}
```

## Benchmarking
1. Use JMeter to load test the API endpoints.
2. Analyze the performance metrics and compare ScyllaDB and MongoDB.

## Conclusion
The benchmarking results were inconclusive due to validation check failures when running ScyllaDB on localhost. Despite expectations, ScyllaDB, which was supposed to outperform MongoDB, performed worse. Further investigation is recommended to identify and address the issues.

## Issues and Contributions
If you encounter any issues or would like to contribute, please open an [issue](https://github.com/yourusername/benchmark-db-rust/issues) or submit a [pull request](https://github.com/yourusername/benchmark-db-rust/pulls).

## License
This project is licensed under the [MIT License](LICENSE).
```

Feel free to customize the content further based on your specific project details and preferences.
