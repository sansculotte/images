# from this enlightening blogpost:
# with a view modifications
# https://christine.website/blog/how-i-start-rust-2020-03-15

FROM rustlang/rust:nightly-slim AS build

WORKDIR /var/src/application

COPY . .

RUN cargo build --release

FROM ubuntu:18.04

COPY --from=build /var/src/application/target/release/images /opt/images

EXPOSE 8000

CMD ["opt/images"]
