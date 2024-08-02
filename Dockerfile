# Build Stage
FROM rust:alpine AS builder
ENV APP_DIR=/bwfs

RUN USER=root cargo new --bin bwfs
WORKDIR ${APP_DIR}

COPY Cargo.toml Cargo.lock ./

RUN apk add --no-cache musl-dev \
 && rustup target add x86_64-unknown-linux-musl

COPY src ${APP_DIR}/src
RUN cargo build --target x86_64-unknown-linux-musl --release

# Final Stage
FROM alpine:latest
ENV APP_DIR=/bwfs

WORKDIR ${APP_DIR}

RUN apk --no-cache add ca-certificates \
 && addgroup -S appgroup && adduser -S appuser -G appgroup \
 && chown -R appuser:appgroup ${APP_DIR} \
 && chmod 0755 ${APP_DIR} \
 && ls -alR

USER appuser

# Copy the build artifact from the builder stage
COPY --from=builder ${APP_DIR}/target/x86_64-unknown-linux-musl/release/bad_word_svr ${APP_DIR}/bad_word_svr

RUN rm -rf ${APP_DIR}/target

# Set the startup command
CMD ["./bad_word_svr"]
