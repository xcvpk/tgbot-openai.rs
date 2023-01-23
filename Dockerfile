FROM rust:1-alpine3.17

WORKDIR /app

COPY . .

CMD ["cargo", "run"]