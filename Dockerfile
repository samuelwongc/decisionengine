FROM rust:1.28

WORKDIR /app

ADD . /app

RUN cargo install diesel_cli
RUN cargo build
RUN diesel migration run

EXPOSE 3000

CMD ["cargo", "run"]