FROM rust:1-buster

WORKDIR /bgprust
COPY . .
RUN rustup default nightly && \
    cargo build
CMD ["./target/debug/bgprust", "64513 10.200.100.3 64512 10.200.100.2 passive 10.100.220.0/24"]