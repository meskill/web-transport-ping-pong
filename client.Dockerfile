# build
FROM rust:1.70-bullseye as build

WORKDIR /app

COPY ./ ./

RUN apt update && \
  # required to build wtransport crate
  apt install -y cmake libclang1-11 libclang-common-11-dev
RUN cargo build --release -p client
RUN mkdir /release
RUN cp ./target/release/client /release

# runtime
# alpine will reqiure less space, but it requires additional setup to be able to build the create for musl
FROM debian:bullseye as runtime

WORKDIR /app
COPY --from=build /release .

ENTRYPOINT ["./client"]