FROM node:20

RUN apt update
RUN apt install -y curl git file jq
RUN npm i -g typescript ts-node

ENV FORC_VERSION=v0.60.0
RUN ARCH=$(uname -m) && \
    if [ "$ARCH" = "aarch64" ]; then \
        FORC_URL="https://github.com/FuelLabs/sway/releases/download/$FORC_VERSION/forc-binaries-linux_arm64.tar.gz"; \
    elif [ "$ARCH" = "x86_64" ]; then \
        FORC_URL="https://github.com/FuelLabs/sway/releases/download/$FORC_VERSION/forc-binaries-linux_amd64.tar.gz"; \
    else \
        echo "Unsupported architecture: $ARCH" && exit 1; \
    fi && \
    mkdir -p /tmp/forc-binaries && \
    curl -L $FORC_URL | tar -xz -C /tmp && \
    mv /tmp/forc-binaries/* /usr/local/bin/

WORKDIR /app
COPY . .

RUN yarn install --frozen-lockfile
RUN forc build --release

ENTRYPOINT [ "ts-node", "index.ts", "&&", "cargo", "run" ]