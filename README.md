# rsblog

## Introduction

rsblog is a project that aims to practice Rust programming and build a web application using the Actix Web framework. It utilizes Tera templates for rendering views, MongoDB for data storage, and provides API documentation using various tools like Swagger UI, Redoc, RapiDoc, and Scalar.

## Features

- **Rust programming language**: The project is written in Rust, a systems programming language known for its performance, safety, and concurrency.

- **Actix Web**: A powerful, pragmatic, and extremely fast web framework for Rust, used for building the web application.

- **Tera templates**: A template engine inspired by Jinja and Django templates, used for rendering dynamic views.

- **MongoDB**: A popular NoSQL database, used for storing and retrieving data.

- **API documentation**: The project provides API documentation using the following tools:
  - **Swagger UI**: A popular open-source tool for visualizing and interacting with API resources.
  - **Redoc**: An OpenAPI-powered documentation UI with a responsive three-panel design.
  - **RapiDoc**: A customizable documentation UI with a focus on developer experience.
  - **Scalar**: A minimalist API documentation tool.

## Getting Started

To get started with the project, follow these steps:

1. **Install Rust**: Make sure you have Rust installed on your system. You can download it from the official Rust website: [Rust Installation](https://www.rust-lang.org/tools/install)

2. **Install MongoDB**: Install MongoDB on your system. You can download it from the official MongoDB website: [MongoDB Download](https://www.mongodb.com/try/download/community)

3. **Clone the repository**: Clone the rsblog repository from GitHub:

   ```bash
   git clone https://github.com/suraw00t/rsblog.git
   ```

4. **Build and run the project**: Navigate to the project directory and build the project using Cargo:

   ```bash
   cd rsblog
   cargo build
   ```

   Then, run the project:

   ```bash
   cargo run
   ```

   The application will start running on http://localhost:8080.

5. **Access the API documentation**: You can access the API documentation using the following URLs:

    - Swagger UI: http://localhost:8080/swagger
    - Redoc: http://localhost:8080/redoc
    - RapiDoc: http://localhost:8080/rapidoc
    - Scalar: http://localhost:8080/scalar

## Contributing

  Contributions are welcome! If you find any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License
  This project is licensed under the MIT License.
