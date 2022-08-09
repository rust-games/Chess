FROM archlinux:latest

RUN pacman -Sy cargo

RUN cargo run --release
