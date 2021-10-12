
FROM rust
WORKDIR /usr/src/rost
COPY . .
# Switch to rust nightly
RUN rustup override set nightly
# Add rust-src to be able to compile rust's core lib
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
# Install bootimage tool
RUN cargo install bootimage
# Build rost
RUN cargo build
# Execute rost with the bootimage tool
CMD ["bootimage", "runner"]