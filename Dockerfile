FROM rust:slim-stretch
WORKDIR /opt/
COPY . .
CMD cargo test --release -- --nocapture
