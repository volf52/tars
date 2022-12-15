FROM volf52/cargo-chef:0.1.48 as chef
FROM volf52/upx-minimal:4.0.1 as upx-src
FROM volf52/rust-musl-builder:1.65.0-slim-mold as img

FROM img as base

WORKDIR /app

COPY --from=chef /bin/cargo-chef /home/volfy/.cargo/bin/cargo-chef

# Recipe

FROM base as recipe

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

# Build

FROM base as build

COPY --from=recipe /app/recipe.json .

RUN cargo chef cook --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY --from=upx-src /bin/upx ./upx

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl && ./upx --best --lzma -o ./tars ./target/x86_64-unknown-linux-musl/release/tars

FROM scratch

COPY --from=build /app/tars /bin/tars

CMD ["/bin/tars", "--help"]
