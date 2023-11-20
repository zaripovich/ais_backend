FROM archlinux as builder
RUN pacman -Syu --noconfirm && pacman -S --noconfirm rustup libpqxx base-devel
COPY . /ais_backend
WORKDIR /ais_backend
RUN rustup default stable
RUN cargo build --release --bin ais_backend

FROM archlinux as runtime
RUN pacman -Syu --noconfirm && pacman -S --noconfirm rustup libpqxx base-devel
COPY --from=builder /ais_backend/target/release/ais_backend /usr/local/bin
WORKDIR /
CMD ./usr/local/bin/ais_backend