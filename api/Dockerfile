FROM rust:1.66
WORKDIR /api
COPY . .

RUN cargo build --release
RUN cargo install --path .

CMD ["api"]