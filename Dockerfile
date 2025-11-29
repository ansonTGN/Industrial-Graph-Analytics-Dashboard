# ---------------------------------------------------
# 1. Etapa de Construcción (Builder)
# ---------------------------------------------------
# CAMBIO IMPORTANTE: Usamos 'rust:1-slim-bookworm' para obtener la última versión estable (ej. 1.80+)
FROM rust:1-slim-bookworm as builder

# Instalar dependencias del sistema necesarias para compilar (OpenSSL para neo4rs)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Compilar en modo release (optimizado)
RUN cargo build --release

# ---------------------------------------------------
# 2. Etapa de Ejecución (Runner - Imagen final ligera)
# ---------------------------------------------------
FROM debian:bookworm-slim

# Instalar certificados SSL y OpenSSL (necesario para conectar a Neo4j Cloud)
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar el binario compilado
# Asegúrate que "neo4j_dashboard" coincide con el name en tu Cargo.toml
COPY --from=builder /app/target/release/neo4j_dashboard ./server

# Copiar archivos estáticos y configuración
COPY --from=builder /app/queries.json ./queries.json
COPY --from=builder /app/templates ./templates

# Render usa la variable PORT
ENV PORT=8080 
EXPOSE 8080

CMD ["./server"]