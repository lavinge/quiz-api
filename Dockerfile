FROM rust:1.77.2 as builder

WORKDIR /usr/src/app

COPY . .

RUN apt update && apt install musl-tools -y

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --target x86_64-unknown-linux-musl --release

# CMD ["ls", "/usr/src/app/target"]

FROM alpine:3.17 as runner

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/quiz-rocket-api /usr/local/bin/quiz-rocket-api

ENV ROCKET_ADDRESS=0.0.0.0

CMD ["quiz-rocket-api"]