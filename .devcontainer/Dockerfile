FROM gr8ninja/rust
USER root
ENV NODE_VER=v22.12.0
RUN mkdir -p /usr/local/lib/nodejs &&\
    wget https://nodejs.org/dist/${NODE_VER}/node-${NODE_VER}-linux-x64.tar.xz &&\
    tar -xJvf node-${NODE_VER}-linux-x64.tar.xz -C /usr/local/lib/nodejs &&\
    rm -f node-*.tar.xz &&\
    mkdir -p /home/vscode/.config/.wrangler &&\
    chown -R vscode:vscode /home/vscode/.config &&\
    apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown &&\
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh &&\
    cargo install cargo-leptos worker-build leptosfmt
USER vscode
ENV PATH="/usr/local/lib/nodejs/node-${NODE_VER}-linux-x64/bin:${PATH}"
VOLUME [ "/home/vscode/.config/.wrangler" ]
