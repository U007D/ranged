FROM ubuntu:20.10
RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y python3 curl git libssl-dev pkg-config clang ninja-build cmake
WORKDIR /opt
RUN git clone https://github.com/rust-lang/rust.git rust/master
WORKDIR ./rust/master
RUN git worktree add ../old
WORKDIR ../old
ARG old_rev
RUN git checkout $old_rev
RUN cat config.toml.example | sed 's/^#assertions = false$/assertions = true/' | sed 's/^#incremental = false/incremental = true/' > config.toml
RUN python3 ./x.py build
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup.sh && chmod +x /tmp/rustup.sh && /tmp/rustup.sh -y
ENV PATH /root/.cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
ENV RUSTC /opt/rust/old/build/x86_64-unknown-linux-gnu/stage1/bin/rustc
WORKDIR /tmp
