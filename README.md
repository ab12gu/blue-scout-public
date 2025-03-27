# blue_scout

**blue_scout** is a Rust project designed for FRC scouting. This project uses **Leptos** for building the frontend. This README explains how to set up the development environment using Docker and Docker Compose.

---

## Prerequisites

Before you begin, make sure you have the following installed:

- **Docker**: [Install Docker](https://docs.docker.com/get-docker/)
- **Docker Compose**: [Install Docker Compose](https://docs.docker.com/compose/install/)

---

## Getting Started

### 1. Clone the Repository

Clone the **blue_scout** project to your local machine:

```sh
git clone https://github.com/Team4682CyBears/blue_scout.git
cd blue_scout
```

### 2. Build and Run the Docker Container

First, build the Docker container:

```sh
docker compose build
```

After building, start the container in detached mode:

```sh
docker compose up -d
```

This will:
- Build the Docker image from the `Dockerfile` (only the first time).
- Start a container named `blue_scout` in the background with the Rust development environment.

For subsequent runs, you can simply use `docker compose up -d` without rebuilding unless you've made changes to the Dockerfile.

### 3. Access the Container

To interact with the running container, use one of the following commands:

```sh
# Using bash shell
docker exec -it blue_scout bash

# Using fish shell
docker exec -it blue_scout fish
```

This opens a shell inside the container, where you can run commands like `cargo leptos serve`, `cargo leptos watch`, and other necessary tasks.

### 4. Working with the Project

Once inside the container, you can compile and run your project using `cargo` commands:

- **Run the project**:
  ```sh
  cargo leptos serve
  ```

- **Run the project with auto recompile (Optional)**:
  ```sh
  cargo leptos watch
  ```

Any changes made to your **blue_scout** files locally will automatically be reflected inside the container due to the volume mount (`-v "$(pwd)":/app`).

## Stopping the Container

To stop the container without removing it, run:

```sh
docker compose down
```

This will stop and remove the container.

---

## Additional Notes

- The project directory is mounted into the container (`-v "$(pwd)":/app`), so any changes to files on your local machine will be available in the container in real time.
- If you need to reinstall any dependencies or add new ones, you can do so from inside the container.
- Only rebuild the container (`docker compose build`) when you need to update the Docker environment, like when changing dependencies in the Dockerfile.

---

## Troubleshooting

If you encounter any issues:
- Ensure Docker and Docker Compose are correctly installed.
- If you've made changes to the Dockerfile, rebuild the container with:
  ```sh
  docker compose build
  docker compose up -d
  ```
- If the container crashes or hangs, try restarting it:
  ```sh
  docker restart blue_scout
  ```
