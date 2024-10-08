FROM docker.io/rust as builder

WORKDIR /home/rust/src

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev binaryen curl && apt-get clean

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo install wasm-bindgen-cli

COPY . .
RUN trunk build


FROM docker.io/svenstaro/miniserve:alpine
COPY --from=builder /home/rust/src/public public
USER 1000:1000
EXPOSE 443
ENTRYPOINT ["/app/miniserve", "public", "--index", "index.html", "--spa", "-i", "::", "-i", "0.0.0.0", "-p", "443", "-t", "Tyme"]
