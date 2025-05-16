FROM rust:1.87.0 as builder

COPY docker/config.toml /usr/local/cargo/config.toml
COPY . .

ENV SQLX_OFFLINE true
RUN cargo install --path .

FROM rust:1.87.0

COPY --from=builder /usr/local/cargo/bin/skarmory /opt/skarmory
COPY config /opt/config

WORKDIR /opt
ENV PATH "$PATH:/opt"
EXPOSE 3000
CMD ["skarmory"]
