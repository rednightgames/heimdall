ARG TAG=latest

FROM heimdall/base:${TAG} AS builder

WORKDIR /app/services/gateway

RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive \
  && apt-get install --no-install-recommends --assume-yes \
  protobuf-compiler

COPY . .

RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-unknown-linux-gnu --release

FROM gcr.io/distroless/static-debian11
WORKDIR /app

EXPOSE 443 80

COPY --from=builder --chown=nonroot:nonroot /app/services/gateway/target/x86_64-unknown-linux-gnu/release/gateway /app

ENTRYPOINT ["/app/gateway"]
