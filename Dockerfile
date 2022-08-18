FROM archlinux:latest

RUN pacman -Syu --noconfirm cargo
COPY . .

RUN cargo run --release
