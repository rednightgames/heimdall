FROM rust:slim-bullseye

ARG TAG=latest
ARG GIT_SHA=latest

ENV HEIMDALL_VERSION ${TAG}

ENV ROOT_DIR /app

WORKDIR ${ROOT_DIR}/proto

COPY . .
