FROM rust:1-alpine3.17

WORKDIR /app

COPY . .


ENV TELOXIDE_TOKEN 5876618728:AAGqmQYWqBtDri-Ug-Qo1sNV1sN9cN5BdDc

CMD ["cargo", "run"]