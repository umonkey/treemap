FROM ghcr.io/getzola/zola:v0.22.1 AS builder
WORKDIR /app
COPY . .
RUN zola build

FROM docker.io/library/nginx:alpine
COPY --from=builder /app/public /usr/share/nginx/html
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
