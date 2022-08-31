FROM rust:latest
WORKDIR /usr/src/app

COPY . .

RUN cd /usr/src/app

RUN sh -c "$(curl -sSfL https://release.solana.com/v1.10.32/install)"

RUN sh -c "$(rustup component add rls rust-analysis rust-src)"
RUN sh -c "$(cargo install cargo-edit)"

RUN sh -c "$(export PATH='/root/.local/share/solana/install/active_release/bin:$PATH':$PATH)"