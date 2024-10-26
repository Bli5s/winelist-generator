FROM rust:latest as builder
WORKDIR /opt/build
COPY . .
RUN cargo install --path .

FROM ubuntu:latest
WORKDIR /opt/winelist-generator
RUN mkdir static
RUN apt-get update && apt-get install -y texlive-full pandoc && rm -rf /var/lib/apt/lists/*
COPY --from=builder /opt/build/target/release/winelistgen .
EXPOSE 8000
CMD ["winelistgen"]
