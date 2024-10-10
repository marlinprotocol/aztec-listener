FROM rust:1.80 AS builder

ENV USER=rustuser
ENV USER_HOME=/home/$USER

RUN adduser --disabled-password --gecos '' $USER

COPY . /home/$USER/aztec-listener
WORKDIR /home/$USER/aztec-listener
RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m myuser

WORKDIR /home/myuser

COPY --from=builder /home/rustuser/aztec-listener/target/release/aztec-listener .

RUN chown myuser:myuser aztec-listener

USER myuser

EXPOSE 8001

CMD ["./aztec-listener"]
