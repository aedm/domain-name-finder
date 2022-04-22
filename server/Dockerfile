FROM rust:1.60 as builder

RUN apt-get update \
    && apt-get install -y cmake

RUN USER=root cargo new --bin server
WORKDIR ./server
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD src ./src

RUN rm ./target/release/deps/server*
RUN cargo build --release


#FROM debian:buster-slim
FROM nginx:latest
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /server/target/release/server ${APP}/server

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

RUN mkdir ${APP}/db
WORKDIR ${APP}/db

COPY com.zone.20220416-060720.txt.gz ${APP}/db/

CMD ["../server"]