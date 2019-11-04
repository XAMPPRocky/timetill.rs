FROM rust:latest
WORKDIR /app
ENV USER docker
RUN rustup component add rustfmt
RUN cargo init --lib
COPY Cargo.toml /app
RUN cargo build --lib
RUN rm -rf src/ target/debug/libserver* target/debug/deps/libserver*
ADD migrations /app/migrations
ADD src /app/src
RUN cargo fmt -- --check
RUN cargo build --tests
RUN cargo test
CMD ["cargo", "run"]

