# SnipBase

SnipBase is a lightweight hub for storing, organizing, and retrieving code snippets. It is designed to provide a simple central place for developers to keep reusable pieces of code and access them from a web interface.

> **Status:** Early development. The project structure and technical direction are defined, while the core features are still being implemented.

## Overview

SnipBase will provide a web-based interface for managing code snippets, with a Rust backend and a lightweight frontend built with standard web technologies.

Planned capabilities include:

- Create, view, edit, and delete snippets
- Organize snippets by language, tags, or categories
- Search and filter stored snippets
- Store snippet metadata such as title, description, and creation date

## Tech Stack

- **Backend:** Rust
- **Frontend:** HTML, CSS, and JavaScript
- **Database:** PostgreSQL
- **Infrastructure:** Docker

## Project Structure

```text
.
├── backend/          # Rust backend application
│   ├── Cargo.toml
│   └── src/
├── frontend/         # Web frontend
│   └── public/
├── test/              # Tests
└── docs/
    └── README.md
```

## Getting Started

The application is currently in its initial setup phase. Detailed setup and usage instructions will be added as the backend, frontend, and PostgreSQL services are implemented.

### Prerequisites

The planned development environment requires:

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
- Docker Compose
- A modern web browser

### Planned Setup

```bash
# Clone the repository
git clone <repository-url>
cd SnipBase

# Start the PostgreSQL service
docker compose up -d

# Run the backend
cd backend
cargo run
```

Frontend startup instructions will be documented once the frontend build and serving workflow are defined.

## Configuration

Environment variables and database configuration will be documented here once the PostgreSQL container and backend connection settings are finalized.

## Development

Contributions and improvements are welcome while the project is being shaped. Before submitting changes:

1. Keep changes focused and documented.
2. Add or update tests for new behavior.
3. Verify that the backend builds successfully.
4. Update this README when setup or project conventions change.

## Roadmap

- [ ] Define the snippet data model
- [ ] Configure the PostgreSQL Docker service
- [ ] Implement the Rust backend API
- [ ] Build the snippet management interface
- [ ] Add search, filtering, and categorization
- [ ] Add automated tests
- [ ] Document deployment and production configuration

## License

No license has been selected yet. Licensing information will be added in a future iteration.