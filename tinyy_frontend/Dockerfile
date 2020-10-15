##### Build Image #####
FROM node as builder

WORKDIR /usr/src/app

# Install dependencies
COPY package.json .
COPY yarn.lock .
RUN yarn

# Build app
COPY . .
RUN yarn build

##### Runtime Image #####
FROM nginx:latest

# Copy react code
COPY --from=builder /usr/src/app/build /usr/share/nginx/html

ENTRYPOINT ["nginx"]
CMD ["-g", "daemon off;"]
