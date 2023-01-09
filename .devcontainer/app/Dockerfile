FROM rust:1.66.0-slim-bullseye

ARG UID=1000
ARG GID=1000

RUN groupadd --gid $GID vscode \
    && useradd --uid $UID --gid vscode\
    --shell /bin/bash --create-home vscode

ENV TZ=Asia/Tokyo
