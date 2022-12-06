# syntax = docker/dockerfile:1.3

FROM rust:1.65-buster as builder
ARG DAY_NUM
WORKDIR /aoc2022
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/aoc2022/target \
    cargo build --locked --release --bin day$DAY_NUM && cp /aoc2022/target/release/day$DAY_NUM /bin/day$DAY_NUM

FROM gcr.io/distroless/cc-debian11:nonroot
ARG DAY_NUM
COPY --from=builder /bin/day$DAY_NUM /aoc
ENTRYPOINT ["/aoc"]
