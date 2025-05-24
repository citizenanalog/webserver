FROM arm64v8/alpine:latest

RUN apk update
RUN apk add tini

ADD ./target/aarch64-unknown-linux-musl/release/webserver /usr/local/bin/webserver

EXPOSE 8080

ENTRYPOINT tini webserver
