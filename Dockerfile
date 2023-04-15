# syntax=docker/dockerfile:1.3
FROM rust:1.65.0 AS builder

ENV IMAGE_NAME=hello-rest

ARG TARGETPLATFORM
ARG TARGET
ARG RUSTARGS

WORKDIR /root

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/root/target,id=${TARGETPLATFORM} \
    cargo build --release --target ${TARGET} ${RUSTARGS} && \
    cargo strip && \
    mv /root/target/${TARGET}/release/${IMAGE_NAME} /root

#FROM gcr.io/distroless/static:nonroot
FROM gcr.io/distroless/base:debug

ENV IMAGE_NAME=hello-rest

WORKDIR /${IMAGE_NAME}

# Copy our build
COPY --from=builder /root/${IMAGE_NAME} /${IMAGE_NAME}/${IMAGE_NAME}
EXPOSE 8080
ENTRYPOINT ["/hello-rest/hello-rest"]