# Use the official Rust image as the base image
FROM rust:latest
WORKDIR /usr/src/adinmo_test

RUN apt-get update

# Copy the rest of the application code
COPY . ./

#Actix
EXPOSE 8080
#RUN mysql -u root -p Password123! < ./scripts/sql_init.sql

RUN cargo build

HEALTHCHECK --interval=30s --timeout=30s CMD curl -f http://localhost:8080/ || exit 1

# Command to run the application
CMD ["./target/debug/adinmo-test"]