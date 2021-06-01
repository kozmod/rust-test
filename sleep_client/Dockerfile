FROM rust:1.52.1 AS builder
WORKDIR /usr/src
# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl
# Create a dummy project and build the app's dependencies.
# If the Cargo.toml or Cargo.lock files have not changed,
# we can use the docker build cache and skip these (typically slow) steps.
RUN USER=root cargo new sleep_client
WORKDIR /usr/src/sleep_client
COPY Cargo.toml ./
RUN cargo build --release

# Copy the source and build the application.
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM alpine:latest
LABEL maintainer="Kozmo"
#Fix prmission through 'private' network proxy
RUN sed -i -e 's/https/http/' /etc/apk/repositories
RUN apk update && apk add mc tmux curl busybox-extras openssh-client apache2-utils vim
COPY --from=builder /usr/local/cargo/bin/sleep_client .
USER 1000
EXPOSE 8080
CMD ["./sleep_client"]