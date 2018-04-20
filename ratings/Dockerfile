# from https://blog.sedrik.se/posts/my-docker-setup-for-rust/
FROM ekidd/rust-musl-builder as builder

WORKDIR /home/rust/

# Avoid having to install/build all dependencies by copying
# the Cargo files and making a dummy src/main.rs
COPY Cargo.toml .
COPY Cargo.lock .
RUN echo "fn main() {}" > src/bin.rs
RUN echo "" > src/lib.rs
# RUN cargo test
RUN cargo build --release

# We need to touch our real main.rs file or else docker will use
# the cached one.
COPY . .
RUN sudo touch src/bin.rs src/lib.rs
RUN sudo chown -R  rust /home/rust

# RUN cargo test
RUN cargo build --release

# Size optimization & rename to main
RUN strip target/x86_64-unknown-linux-musl/release/ratings -o target/x86_64-unknown-linux-musl/release/main

# Start building the final image
FROM scratch
WORKDIR /home/rust/
COPY --from=builder /home/rust/target/x86_64-unknown-linux-musl/release/main .
ENTRYPOINT ["./main"]
