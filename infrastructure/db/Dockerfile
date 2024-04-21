FROM node:21-alpine

WORKDIR /APP

COPY ./db.json .
RUN npm install -g json-server
# RUN json-server --watch db.json
EXPOSE 3000
CMD [ "json-server", "--watch", "db.json" ]
