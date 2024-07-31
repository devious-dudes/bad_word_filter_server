# Stage 1: Build the application
FROM rust:latest AS builder

# Create a new empty shell project
RUN USER=root cargo new --bin bad_word_svr
WORKDIR /bad_word_svr

# Copy the manifest file
COPY Cargo.toml Cargo.lock ./

# Build dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY src ./src

# Build the project
RUN cargo build --release

# Stage 2: Create the final image
FROM alpine:latest

# Install required packages
RUN apk --no-cache add ca-certificates

# Create a non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Copy the build artifact from the builder stage
COPY --from=builder /bad_word_svr/target/release/bad_word_svr /usr/local/bin/bad_word_svr

# Set permissions and change to non-root user
RUN chown -R appuser:appgroup /usr/local/bin/bad_word_svr
USER appuser

# Set the startup command
CMD ["/usr/local/bin/bad_word_svr"]
