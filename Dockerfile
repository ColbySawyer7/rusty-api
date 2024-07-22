FROM rust:1.79

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["app"]
