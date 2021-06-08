FROM rust:latest as builder
LABEL stage=builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl
RUN USER=root cargo new sleep_client
WORKDIR /usr/src/sleep_client
COPY Cargo.toml .
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine:latest
LABEL maintainer="Kozmo"
#Fix prmission through 'private' network proxy
RUN sed -i -e 's/https/http/' /etc/apk/repositories
RUN apk update && apk add mc tmux curl busybox-extras openssh-client apache2-utils vim
COPY --from=builder /usr/local/cargo/bin/sleep_client .
USER 1000
EXPOSE 8080
CMD ["./sleep_client", "-p", "8080"]