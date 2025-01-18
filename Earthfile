VERSION 0.8
ARG RUST_VERSION=1.83.0
FROM rust:${RUST_VERSION}
WORKDIR /workspace

rust-deps:
    COPY Cargo.lock ./
    COPY Cargo.toml ./
    COPY rust-toolchain.toml ./
    RUN cargo fetch
    RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
    RUN cargo binstall --no-confirm trunk leptosfmt just

node-deps:
    FROM node:22-slim
    WORKDIR /workspace
    COPY package.json ./
    COPY package-lock.json ./
    RUN npm ci
    SAVE ARTIFACT node_modules node_modules

playwright-deps:
    FROM node:22-slim
    WORKDIR /workspace
    RUN apt update && apt install --no-install-recommends --yes ca-certificates
    RUN npx --yes playwright install --with-deps

deps:
    FROM +rust-deps
    COPY +node-deps/node_modules ./node_modules
    COPY ./src/ ./src
    COPY ./style/ ./style
    COPY ./index.html ./
    COPY ./package.json ./
    COPY ./package-lock.json ./
    COPY ./tailwind.config.js ./
    COPY ./justfile ./

lint:
    FROM +deps
    RUN just lint

test:
    FROM +playwright-deps
    COPY +node-deps/node_modules ./node_modules
    COPY +build-release/dist dist/
    COPY ./package.json ./
    COPY ./package-lock.json ./
    COPY ./playwright.config.ts ./
    COPY ./e2e/ ./e2e
    RUN npx playwright test

build:
    FROM +deps
    RUN just build
    SAVE ARTIFACT ./dist dist AS LOCAL ./dist

build-release:
    FROM +deps
    RUN just build-release
    SAVE ARTIFACT ./dist dist AS LOCAL ./dist

docker:
    FROM nginx:latest
    COPY +build-release/dist /usr/share/nginx/html
    ARG EARTHLY_GIT_SHORT_HASH
    ARG tag=${EARTHLY_GIT_SHORT_HASH}
    ARG registry_prefix
    SAVE IMAGE --push ${registry_prefix}adriencos/leptos-tutorial:${tag}

gh:
    FROM debian:bookworm
    RUN apt update && apt install -y --no-install-recommends wget ca-certificates
    RUN wget https://github.com/cli/cli/releases/download/v2.49.2/gh_2.49.2_linux_amd64.tar.gz
    RUN tar xf gh_2.49.2_linux_amd64.tar.gz
    RUN mv ./gh_2.49.2_linux_amd64/bin/gh /bin/

create-gh-release:
    FROM +gh
    ARG --required tag
    BUILD +package
    BUILD +pex
    COPY +package/* dist/
    COPY +pex/* dist/
    RUN --push \
        --secret GH_TOKEN \
        gh release create \
            --repo AdrienCos/sapling \
            --draft \
            --verify-tag \
            --title $tag \
            $tag ./dist/*
