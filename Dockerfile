# ---------------------------------------------------
# 1. Etapa de Construcción (Builder)
# ---------------------------------------------------
FROM rust:1-slim-bookworm as builder

# Instalar dependencias del sistema necesarias
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Compilar en modo release
# Cargo leerá "default-run" del toml
RUN cargo build --release

# ---------------------------------------------------
# 2. Etapa de Ejecución (Runner)
# ---------------------------------------------------
FROM debian:bookworm-slim

# Instalar certificados SSL para Neo4j Cloud
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar el binario
COPY --from=builder /app/target/release/neo4j_dashboard ./server

# Copiar archivos estáticos
COPY --from=builder /app/queries.json ./queries.json
COPY --from=builder /app/templates ./templates

# Configuración puerto Render
ENV PORT=8080 
EXPOSE 8080

CMD ["./server"]