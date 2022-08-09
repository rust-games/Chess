FROM archlinux:latest

RUN pacman -Syu --noconfirm cargo

RUN cargo run --release
