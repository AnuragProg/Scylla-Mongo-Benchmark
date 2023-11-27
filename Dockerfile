FROM rust:1.74

WORKDIR /usr/src/benchmark

COPY . .

EXPOSE 3000

ENV SCYLLA_URI=scylla:9042

ENV MONGO_URI=mongodb://mongo:27017

RUN cargo build --release
