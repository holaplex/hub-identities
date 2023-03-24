# Hub Identities

Hub identities service manages all globally unique user identities and is responsible for user authentication. It provides a secure, robust, and scalable user authentication and management system.

GraphQL API for interacting with [Ory Kratos](https://www.ory.sh/docs/kratos/ory-kratos-intro) admin API.

# Workspaces

```
/api # API server
/core # shared dependencies
/ory # ory admin SDK
/src # app entry point
```

# Getting Started

Run docker compose to startup Ory Kratos in development mode with Ory self service UI for creating users via username+password. Kratos configuration files are located in the `kratos` directory.

```
docker-compose up
```

| Service           | Endpoint              |
| ----------------- | --------------------- |
| Kratos Public     | http://localhost:4433 |
| Kratos Admin      | http://localhost:4434 |
| Ory Standalone UI | http://localhost:4455 |

Next startup the identities API

```
cargo run -- --kratos-admin-endpoint=http://localhost:4434
```