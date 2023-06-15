FROM rust:1.70-bookworm as builder

WORKDIR /build

RUN apt update && \
    apt install -y python3-pip && \
    pip3 install cargo-lambda --break-system-packages

ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src

RUN cargo lambda build --release --arm64

# copy artifacts to a clean image
FROM public.ecr.aws/lambda/provided:al2-arm64

COPY --from=builder /build/target/lambda/rust-aws-lambda/bootstrap /bootstrap

ENTRYPOINT ["/bootstrap"]
