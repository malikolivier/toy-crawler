FROM rust:1.65@sha256:6d44ed87fe759752c89d1f68596f84a23493d3d3395ed843d3a1c104866e5d9e as builder

WORKDIR /root/toy-crawler

COPY . .

RUN cargo install --path .

ENTRYPOINT ["toy-crawler"]
