FROM rust:1.70.0 as builder

WORKDIR /usr/app
RUN USER=root cargo new --bin fair_teams
WORKDIR /usr/app/fair_teams

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/fair_teams*
RUN cargo build --release

# -----
FROM debian:bullseye-slim

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
ENV ROCKET_ENV=prod

RUN apt-get update 
RUN apt-get install -y curl && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

WORKDIR /usr/app
COPY --from=builder /usr/app/fair_teams/target/release/fair_teams /usr/app/fair_teams

CMD ["/usr/app/fair_teams"]