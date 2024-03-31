# GoWithDev API

A robust RESTful API built with Rust and Axum, powering [gowithdev.in](https://gowithdev.in) - a technical blog platform.

## Features

- Secure authentication system for admin operations
- Complete blog post management (CRUD operations)
- Tag-based categorization for posts
- High-performance Rust-based backend
- RESTful API architecture
- Efficient post retrieval and filtering

## Tech Stack

- **Language**: Rust
- **Framework**: Axum
- **Authentication**: JWT (JSON Web Tokens)
- **Database**: Postgres

## Prerequisites

- Rust (latest stable version)
- Cargo package manager
- Postgres

## Installation

1. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the Repository**
   ```bash
   git clone https://github.com/devak997/gowithdev-rsapi.git
   cd gowithdev-rsapi
   ```

3. **Environment Setup**
   ```bash
   cp .env.example .env
   # Update .env with your configuration
   ```

4. **Build and Run**
   ```bash
   cargo build
   cargo run
   ```

The API will be available at `http://localhost:8000` (or your configured port).

## API Documentation

### Endpoints

#### Public Endpoints

| Endpoint | Method | Description |
|----------|---------|-------------|
| `/posts` | GET | Fetch all published posts |
| `/posts/:id` | GET | Get a specific post |
| `/login` | POST | Authenticate user |

#### Protected Endpoints (Requires Authentication)

| Endpoint | Method | Description |
|----------|---------|-------------|
| `/admin/posts` | POST | Create new post |
| `/admin/posts/:id` | PUT | Update post |
| `/admin/posts/:id` | DELETE | Delete post |
| `/admin/categories` | GET | List all categories |

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Contribution Guidelines

- Write tests for new features
- Follow Rust coding standards
- Update documentation for any API changes
- Use conventional commits

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

- Developer: Deva Kumar Kilim
- Email: devakumar997@gmail.com
- Blog: [gowithdev.in](https://gowithdev.in)

## Acknowledgments

- Axum framework
- Rust community
- SeaORM