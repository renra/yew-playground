FROM rust

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

RUN apt-get -y update && \
  apt-get -y install dumb-init

ENTRYPOINT ["dumb-init", "--"]

ENV dir /usr/src/app
RUN mkdir -p ${dir}
WORKDIR ${dir}

COPY Cargo.toml ${dir}
COPY Cargo.lock ${dir}
COPY index.html ${dir}
COPY src ${dir}/src

RUN cargo build
RUN trunk build
