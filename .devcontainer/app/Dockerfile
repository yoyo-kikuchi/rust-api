FROM rust:1.66.0-slim-bullseye

ARG UID=1000
ARG GID=1000

RUN apt-get update \
    && apt-get install -y git curl wget libssl-dev pkg-config\
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --gid $GID vscode \
    && useradd --uid $UID --gid vscode \
    --shell /bin/bash --create-home vscode \
    && rustup component add clippy rls rust-analysis rust-src rust-docs rustfmt rust-analyzer

ENV TZ=Asia/Tokyo \
    CARGO_TARGET_DIR=/tmp/target \
    LANG=ja_JP.utf8 \
    LC_CTYPE=ja_JP.utf8
