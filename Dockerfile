# Build the backend
FROM rust:1.60 as backend-builder

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


# Build the frontend
FROM node:16.14.2-alpine as frontend-builder

ADD ./webapp/package.json /app/
ADD ./webapp/yarn.lock /app/
WORKDIR /app
RUN yarn

ADD ./webapp /app
RUN yarn build


#FROM debian:buster-slim
FROM nginx:1.21.6
ARG APP=/usr/src/app

#RUN apt-get update \
#    && apt-get install -y ca-certificates tzdata \
#    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000
EXPOSE 9000

#ENV TZ=Etc/UTC \
#    APP_USER=appuser

#RUN groupadd $APP_USER \
#    && useradd -g $APP_USER $APP_USER \
#    && mkdir -p ${APP}

RUN mkdir -p ${APP}
COPY --from=backend-builder /server/target/release/server ${APP}/server
RUN chown -R $APP_USER:$APP_USER ${APP}

COPY --from=frontend-builder /app/dist /usr/share/nginx/html
COPY ./proxy/nginx.conf /etc/nginx/conf.d/default.conf
RUN chown -R $APP_USER:$APP_USER /etc/nginx/conf.d/default.conf
RUN chown -R $APP_USER:$APP_USER /var/cache/nginx

#USER $APP_USER

# Copy database file
WORKDIR ${APP}
COPY ./db/processed ./db/processed

ADD ./proxy/start.sh ./
CMD ["./start.sh"]