FROM alpine:3.19
RUN apk update && apk upgrade && apk add --no-cache rust cargo openssl-dev
WORKDIR /app
COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src
RUN cargo build --release
EXPOSE 8080
CMD ["./geomk"]
ENTRYPOINT ["./target/release/geomk"]
