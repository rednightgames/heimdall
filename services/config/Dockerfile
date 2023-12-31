ARG TAG=latest

FROM heimdall/base:${TAG} AS builder

WORKDIR /app/services/config

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive \
  && apt-get install --no-install-recommends --assume-yes \
  protobuf-compiler libssl-dev pkg-config

COPY . .

RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-unknown-linux-gnu --release

FROM gcr.io/distroless/static-debian11
WORKDIR /app

EXPOSE 666 6666

COPY --from=builder --chown=nonroot:nonroot /app/services/config/target/x86_64-unknown-linux-gnu/release/config /app

ENTRYPOINT ["/app/config"]
