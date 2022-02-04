FROM node:16-alpine as build

ADD ./package.json /app/
ADD ./yarn.lock /app/
WORKDIR /app
RUN yarn

ADD . /app
RUN yarn build

FROM nginx:1.21-alpine

COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist /usr/share/nginx/html
