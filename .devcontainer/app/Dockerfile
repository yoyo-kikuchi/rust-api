FROM rust:1.66.0-slim-bullseye

ARG UID=1000
ARG GID=1000

RUN apt-get update \
    && apt-get install -y git curl wget\
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid $GID vscode \
    && useradd --uid $UID --gid vscode \
    --shell /bin/bash --create-home vscode \
    && rustup component add clippy rls rust-analysis rust-src rust-docs rustfmt rust-analyzer

ENV TZ=Asia/Tokyo
