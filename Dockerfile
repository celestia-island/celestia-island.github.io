# Preload dependencies,
# used to speed up repeated builds and reduce traffic consumption of libraries
FROM rust:latest AS stage-deps

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli@0.2.93

COPY ./Cargo.toml /home/Cargo.toml
COPY ./.cargo /home/.cargo
COPY ./packages /home/packages

WORKDIR /home
RUN cargo fetch
RUN mkdir -p /home/target/cache/static-resources

# Stage 1 for client build, used to compile wasm file
FROM stage-deps AS stage-client-build1

WORKDIR /home
RUN cargo build -r --target wasm32-unknown-unknown -p _client_bootstrap_web

# Stage 2 for client build, used to process wasm file for browser platform
FROM stage-deps AS stage-client-build2

COPY --from=stage-client-build1 /home/target/wasm32-unknown-unknown/release/_client_bootstrap_web.wasm /home/target/wasm32-unknown-unknown/release/_client_bootstrap_web.wasm
WORKDIR /home
RUN wasm-bindgen --out-dir ./target/wasm32-html --out-name app --target no-modules --no-typescript --no-modules-global wasm_vendor_entry ./target/wasm32-unknown-unknown/release/_client_bootstrap_web.wasm

# Stage 1 for server build, used to compile server program
FROM stage-deps AS stage-server-build1

WORKDIR /home
RUN cargo build --offline --package _server_bootstrap_docker --release

# Stage 2 for server build, used to integrate the build result of client and generate the final image
FROM rust:latest AS stage-server-build2

COPY --from=stage-client-build2 /home/target/wasm32-html/app.js /home/target/cache/static-resources/app.js
COPY --from=stage-client-build2 /home/target/wasm32-html/app_bg.wasm /home/target/cache/static-resources/app.wasm
COPY --from=stage-server-build1 /home/target/release/_server_bootstrap_docker /home/a
ENV ROOT_DIR=/home/res
WORKDIR /home
ENTRYPOINT [ "./a" ]
EXPOSE 8787
