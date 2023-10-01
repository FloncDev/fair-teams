FROM rust:1.70.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV ROCKET_ENV=prod

EXPOSE 8000

COPY . /app
WORKDIR /app

RUN cargo build

CMD ["cargo", "run"]