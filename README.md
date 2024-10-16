# OpenResource

## About
OpenResource is an open source resource management tool

## Goal
Every company manages resources such as products, inventory, employees, clients, and schedules. OpenResource is designed to be a suitable replacement/alternative for more expensive enterprise solutions. Our hope is to provide a robust and reliable solution for small to medium buisnesses that can provide all the same utility as much larger expensive enterprise solutions.

## Architecture
All parts of the system are modular for maintainability and scailability. 

### Frontend - Built in nextjs and uses tailwind css for a simple but intuitive interface to all features exposed by the OpenResource API. It is a working reference implementation for anybody wanting to use the product as a starting point to build a custom solution themselves or for anybody who needs a reliable but flexible data model that should fit the needs of most small to medium buisnesses.

### Backend - Rust is our choice for a backend API using Axom as our request handler and Diesel as our ORM for PostgreSQL. Rust is a powerful, performant, and reliable language with a strict compiler and rich type system that reduces errors while allowing expressive data processing.

### License

This project is under the MIT License.
