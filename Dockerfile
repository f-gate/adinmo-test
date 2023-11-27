# Use the official Rust image as the base image
FROM rust:latest
WORKDIR /usr/src/adinmo_test

RUN apt-get update

# Copy the rest of the application code
COPY . ./

#Actix
EXPOSE 8080

RUN cargo build --release

# Command to run the application
CMD ["./target/release/adinmo-test"]