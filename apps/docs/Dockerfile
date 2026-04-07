FROM node:20-bullseye

RUN npm install -g honkit

WORKDIR /book
EXPOSE 4000

CMD ["honkit", "serve", "-p", "4000", "-a", "0.0.0.0"]
