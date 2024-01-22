# Rust Actix-Web Task Management Project

## Introduction

This Rust project is a simple web application built with the Actix-Web framework, designed to manage tasks and users. The application exposes a set of RESTful API endpoints to perform various operations such as retrieving tasks, users, adding tasks, marking tasks as done, and deleting tasks. It utilizes a PostgreSQL database for persistent data storage.

## Project Structure

The project is organized into three main modules: `handler`, `model`, and the main application module.

- **`handler` Module**: Contains sub-modules `model` and `route`.

  - `model`: Defines the application state (`AppState`) and data structures for tasks and users.
  - `route`: Contains the route handlers for different HTTP endpoints.

- **Main Application Module (`main` Function)**: Serves as the entry point, setting up the web server, configuring logging, establishing a database connection, and defining the routes.

## Functionality

### Route Handlers

1. **Health Checker**

   - **Endpoint**: `/healthchecker` (GET)
   - **Function**: `health_checker_handler`
   - **Description**: Returns a JSON response indicating the health status of the server.

2. **Task Endpoints**
   - **Get Tasks by User ID**: Retrieves tasks for a given user ID.
   - **Get All Users**: Retrieves all users.
   - **Get All Tasks**: Retrieves all tasks.
   - **Add Task**: Adds a new task for a user, updating user task counts.
   - **Task Done**: Marks a task as done, updating user task counts.
   - **Get User by ID**: Retrieves user details by user ID.
   - **Delete Task**: Deletes a task, updating user task counts if the task is done.

# User Guide

## 1. Environment Setup:

- Ensure that the required environment variables are set, including `DATABASE_URL` for the PostgreSQL connection.

## 2. Running the Server:

- Execute the `main` function to start the Actix web server.
- The server runs on "http://127.0.0.1:8000" by default.

## 3. Routes:

### Health Check

- **Endpoint:** `/healthchecker`
- **Method:** GET
- **Response:** Server health status.

### Task Operations

- **Get All Tasks:**

  - **Endpoint:** `/get_all_tasks`
  - **Method:** GET
  - **Response:** List of all tasks.

- **Get Tasks for a User:**

  - **Endpoint:** `/get_tasks/{user_id}`
  - **Method:** GET
  - **Response:** Tasks for the specified user.

- **Add Task:**

  - **Endpoint:** `/add_task`
  - **Method:** POST
  - **Body:** JSON payload with `user_id` and `task_name`.
  - **Response:** Added task details.

- **Mark Task as Done:**

  - **Endpoint:** `/task_done`
  - **Method:** POST
  - **Body:** JSON payload with `user_id` and `task_id`.
  - **Response:** Success or error message.

- **Delete Task:**
  - **Endpoint:** `/delete_task`
  - **Method:** POST
  - **Body:** JSON payload with `user_id` and `task_id`.

### Model

- **AppState Struct**: Holds the database connection pool.

- **Data Structures**:
  - `Task`: Represents a task with task ID, name, completion status, and associated user ID.
  - `User`: Represents a user with user ID, name, remaining tasks, and completed tasks.
  - `CreateTask`: Data structure for creating a new task.
  - `CreateUser`: Data structure for creating a new user.
  - `TaskAction`: Data structure for performing actions on a task (e.g., marking as done, deleting).

## Technologies Used

- **Actix-Web**: Asynchronous web framework for Rust.
- **SQLx**: Asynchronous SQL library for Rust with support for PostgreSQL.
- **dotenv**: Library for loading environment variables from a .env file.
- **serde**: Serialization and deserialization library for Rust.

## Conclusion

This Rust Actix-Web project provides a foundation for building a scalable and efficient task management system. It adheres to best practices in Rust development, including modularization, error handling, and documentation. Developers can extend and customize this project to suit their specific requirements for task and user management in web applications.
