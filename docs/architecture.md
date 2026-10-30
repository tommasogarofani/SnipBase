# Backend Architecture

This document describes the backend architecture of **Snipbase**, developed in **Rust**[cite: 3].

## Selected Tech Stack

- **Web Framework — `Axum`**: Defines HTTP routes and handlers, extracts request data, and builds HTTP responses. It provides the web layer of the backend and integrates with `Tokio`.
- **Asynchronous Runtime — `Tokio`**: Runs asynchronous Rust code and manages non-blocking tasks, network operations, and concurrent requests.
- **Database & Queries — `SQLx`**: Connects the application to PostgreSQL, manages the connection pool, and executes type-checked SQL queries at compile time.
- **Serialization — `Serde` / `serde_json`**: Converts Rust structs into JSON responses and parses JSON request bodies into typed Rust data structures.
- **Configuration — `dotenvy`**: Loads environment variables from a `.env` file, such as the database connection string and other application settings.

## Module Structure (`backend/src/`)

### `domain/`:

    Contains the Rust structs that represent the data (e.g., Snippet, Tag) and the DTOs (Data Transfer Objects) used to validate incoming data. It knows nothing about the database or the web layer.

### `repository/`:

    This is the only layer that communicates directly with Database. It executes SQL queries (e.g., inserting a snippet and linking tags in the snippet_tag junction table) and returns raw data as domain structs.

### `service/`:

    Contains the application rules. It receives data from the controllers/handlers, applies any required logic (e.g., verifying that a snippet is not empty and managing SQL transactions through the repository), and returns the result ready to be presented.

### `handlers/`:

    Asynchronous Axum functions that capture HTTP requests (URL parameters, query strings, and JSON bodies), invoke the corresponding service, and map the result to an HTTP response (e.g., StatusCode::OK with a JSON payload).

### `router/`:
    Combines all routes under the /api prefix and configures middleware (such as CORS).

### `main.rs`:

    Initializes the database connection, creates the SQLx pool, builds the router, and starts the HTTP server.

```text
backend/
├── Cargo.toml
└── src/
    ├── main.rs
    │
    ├── domain/
    │
    ├── repository/
    │
    ├── service/
    │
    ├── handlers/
    │
    └── router/
```
