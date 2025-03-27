FROM rustlang/rust:nightly-bullseye

RUN apt-get update && apt-get install -y curl unzip pkg-config libssl-dev wget fish --fix-missing

RUN ARCH=$(uname -m) && \
    if [ "$ARCH" = "x86_64" ]; then \
    curl -L "http://ftp.debian.org/debian/pool/main/o/openssl/libssl1.1_1.1.1w-0+deb11u1_amd64.deb" -o libssl1.1_1.1.1w-0+deb11u1.deb; \
    elif [ "$ARCH" = "aarch64" ]; then \
    curl -L "http://ftp.debian.org/debian/pool/main/o/openssl/libssl1.1_1.1.1w-0+deb11u1_arm64.deb" -o libssl1.1_1.1.1w-0+deb11u1.deb; \
    else \
    echo "Unsupported architecture: $ARCH" && exit 1; \
    fi && \
    dpkg -i libssl1.1_1.1.1w-0+deb11u1.deb && \
    rm libssl1.1_1.1.1w-0+deb11u1.deb

RUN apt --fix-broken -y install

RUN ARCH=$(uname -m) && \
    if [ "$ARCH" = "x86_64" ]; then \
    curl -L "https://github.com/duckdb/duckdb/releases/latest/download/libduckdb-linux-amd64.zip" -o libduckdb.zip; \
    elif [ "$ARCH" = "aarch64" ]; then \
    curl -L "https://github.com/duckdb/duckdb/releases/latest/download/libduckdb-linux-aarch64.zip" -o libduckdb.zip; \
    else \
    echo "Unsupported architecture: $ARCH" && exit 1; \
    fi && \
    unzip libduckdb.zip -d /usr/lib/ && \
    rm libduckdb.zip

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz --no-check-certificate
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

RUN apt-get install -y --no-install-recommends clang

RUN cargo binstall cargo-leptos -y

RUN rustup target add wasm32-unknown-unknown

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /app
WORKDIR /app
COPY . .

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="target/site"
EXPOSE 8080

CMD ["/usr/bin/fish"]
