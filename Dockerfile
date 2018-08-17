FROM rust:1.28

WORKDIR /app

ADD . /app

RUN cargo build

EXPOSE 3000

CMD ["cargo", "run"]