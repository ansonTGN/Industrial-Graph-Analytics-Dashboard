# ---------------------------------------------------
# 1. Etapa de Construcción (Builder)
# ---------------------------------------------------
FROM rust:1.75-slim-bookworm as builder

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
COPY --from=builder /app/target/release/neo4j_dashboard ./server

# CRUCIAL: Copiar los archivos que tu app lee en tiempo de ejecución
COPY --from=builder /app/queries.json ./queries.json
COPY --from=builder /app/templates ./templates
# Si tuvieras una carpeta static o assets, cópiala también:
# COPY --from=builder /app/static ./static

# Render asignará el puerto automáticamente en la variable PORT
ENV PORT=8080 
EXPOSE 8080

CMD ["./server"]