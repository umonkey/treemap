# Please note that this file is only used on dev machines to run the environment.
# In production, we use a static website hosting service.

FROM docker.io/library/node:20-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci
COPY . .
EXPOSE 5173
CMD ["npm", "run", "dev", "--", "--host"]
