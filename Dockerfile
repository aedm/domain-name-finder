FROM rust:1.60 as builder

# Update image
RUN apt-get update \
    && apt-get install -y cmake

# Install and build dependencies
RUN USER=root cargo new --bin server
WORKDIR ./server
COPY server/Cargo.toml ./Cargo.toml
COPY server/Cargo.lock ./Cargo.lock
RUN cargo build --release
RUN rm -rf ./src
RUN rm ./target/release/deps/server*

# Copy and build sources
ADD server/src ./src
RUN cargo build --release


#FROM debian:buster-slim
FROM nginx:latest
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 9000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /server/target/release/server ${APP}/server

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER

# Copy database file
COPY ./db ${APP}/db
WORKDIR ${APP}

CMD ["./server"]