FROM rustlang/rust:nightly as builder

RUN USER=root cargo new --bin tinyy_backend

WORKDIR /tinyy_backend
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY . .

RUN rm ./target/release/deps/tinyy_backend*
RUN cargo build --release


FROM debian:buster-slim

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /tinyy_backend/target/release/tinyy_backend ${APP}/tinyy_backend

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./tinyy_backend"]
