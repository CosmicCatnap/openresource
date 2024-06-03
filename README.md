# Description
OpenResource is an open source resource manager designed to be performant and scalable

OpenResource consists of an API in rust that processes transaction to a database

# Architecture

We use what we call a Microlyth which has a "core" binary handling the bulk of operations and secondary workers for scheduled tasks and scalable workloads. Our hope is that any number of cores and workers can be containerized or virtualized as desired without interferance.

OpenResource has a backend built in rust becuase it is a well established language for safe, performant, and reliable code which we hope will produce safe, performant, and reliable applications. It uses Axum because I trust the people who built Tokio. It also provides a batteries included build system with cargo and a strict compiler with verbose and well documented output reducing a number of common mistakes and frustrations found in other languages.

OpenResource has a frontend built in React because it is the industry standard for responsive web applications and Material UI because it is a popular choice for design elements.

# Licensing
This project along with all software used in this project is on the MIT License
