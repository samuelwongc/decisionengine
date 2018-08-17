FROM rust:1.28

WORKDIR /app

ADD . /app

RUN cargo install diesel_cli
RUN cargo install cargo-watch
RUN cargo build
RUN diesel migration run

EXPOSE 3000

CMD ["cargo", "watch", "-x", "run"]