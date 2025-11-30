# Neo4j Industrial Graph Dashboard
**Analítica Avanzada de Activos Industriales con Rust + Neo4j + GenAI Multi-Proveedor**

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![Neo4j](https://img.shields.io/badge/Neo4j-Aura%20%2F%20Local-blue?logo=neo4j)
![Actix-Web](https://img.shields.io/badge/Backend-Actix_Web-green)
![AI Providers](https://img.shields.io/badge/AI-OpenAI%20|%20Groq%20|%20DeepSeek%20|%20Ollama-purple?logo=openai)
![License](https://img.shields.io/badge/License-MIT-lightgrey)

---

**Idiomas / Languages / Idiomes:**  
[🇪🇸 Español](#-español) | [🇬🇧 English](#-english) | [🏴 Català](#-català)

---

<a name="-español"></a>
# 🇪🇸 Español

## 📖 Descripción General
**Neo4j Industrial Graph Dashboard** es una plataforma web de alto rendimiento diseñada para la visualización, auditoría y análisis de activos industriales complejos. Su arquitectura combina la seguridad y velocidad de **Rust** en el backend con la potencia de **Neo4j** para modelar relaciones jerárquicas (Plantas → Equipos → Materiales).

La característica estrella es su **Asistente de IA Universal**, capaz de traducir preguntas en lenguaje natural a consultas Cypher complejas, soportando múltiples proveedores de LLM (Nube y Local).

### ✨ Características Principales
*   **Visualización de Grafos:** Renderizado interactivo de redes de activos con `Vis.js`.
*   **Gestión Dinámica:** Las consultas se cargan desde un archivo JSON sin recompilar el servidor.
*   **Multi-Proveedor de IA:** Soporte nativo para OpenAI, Groq, DeepSeek y modelos locales con Ollama.
*   **Proxy Seguro:** El backend actúa como pasarela para evitar problemas de CORS y proteger las claves API.

---

## 🤖 Configuración del Asistente de IA (Multi-Modelo)

La aplicación incluye un chat inteligente (botón flotante 🤖) que utiliza **Function Calling** para interrogar a la base de datos. Puedes elegir tu proveedor de IA preferido haciendo clic en el botón de configuración (⚙️) dentro del chat.

### Opciones de Configuración

#### 1. ⚡ Groq (Recomendado por Velocidad)
Ideal para respuestas casi instantáneas utilizando modelos Llama 3.
*   **Proveedor:** Selecciona `Groq`.
*   **API Key:** Obtén una gratuita en [console.groq.com](https://console.groq.com).
*   **Modelo:** Por defecto usa `llama-3.3-70b-versatile`.

#### 2. 🧠 OpenAI (Estándar)
La opción más robusta y fiable.
*   **Proveedor:** Selecciona `OpenAI`.
*   **API Key:** Tu clave `sk-...` de OpenAI.
*   **Modelo:** `gpt-4o` (por defecto).

#### 3. 💻 Ollama (Privacidad Local)
Para ejecutar modelos en tu propia máquina sin enviar datos a la nube.
*   **Proveedor:** Selecciona `Ollama`.
*   **Requisitos:** Tener [Ollama](https://ollama.com/) instalado.
*   **Configuración Técnica:**
    *   Ejecuta Ollama permitiendo orígenes: `OLLAMA_ORIGINS="*" ollama serve`
    *   **Si usas Docker:** La URL debe ser `http://host.docker.internal:11434/v1/chat/completions`.
    *   **Si ejecutas en local:** La URL es `http://localhost:11434/v1/chat/completions`.
*   **Modelo:** Asegúrate de tener el modelo descargado (ej. `ollama pull llama3.2`).

#### 4. 🚀 DeepSeek (Económico y Potente)
Excelente capacidad de razonamiento (V3) a una fracción del coste.
*   **Proveedor:** Selecciona `DeepSeek`.
*   **API Key:** Tu clave de `platform.deepseek.com`.

#### 5. 🔌 Anthropic (Claude) / Custom
Para usar Claude 3.5 Sonnet u otros proveedores compatibles con OpenAI.
*   **Proveedor:** Selecciona `Custom`.
*   **Método Recomendado:** Usar **OpenRouter** como intermediario para compatibilidad de API.
    *   **Base URL:** `https://openrouter.ai/api/v1/chat/completions`
    *   **Modelo:** `anthropic/claude-3.5-sonnet`

---

## 🧩 Gestión de Consultas (Queries)

No es necesario tocar código Rust para añadir nuevas analíticas. Todo reside en `queries.json`.

```json
{
  "id": "M01",
  "category": "Mantenimiento",
  "title": "Desglose BOM",
  "description": "Descripción que la IA usa para entender cuándo usar esta herramienta.",
  "cypher": "MATCH (n)-[:REL]->(m) RETURN ...",
  "needs_param": true,  // true = Requiere seleccionar un nodo antes
  "is_graph": true,     // true = Renderiza nodos; false = Renderiza tabla/gráfico
  "icon": "fa-share-nodes"
}
```
*Para añadir una consulta, simplemente edita este archivo y reinicia el servidor.*

---

## 🚀 Despliegue

### Docker (Recomendado)
```bash
# Construir la imagen
docker build -t neo4j_dashboard .

# Ejecutar (Asegúrate de tener el archivo .env configurado)
docker run -p 8080:8080 --env-file .env neo4j_dashboard
```

### Ejecución Local (Rust)
```bash
# Instalar dependencias
cargo build --release

# Ejecutar
./target/release/neo4j_dashboard
```

Variables de entorno requeridas en `.env`:
```env
NEO4J_URI="neo4j+s://<tu-instancia>.databases.neo4j.io"
NEO4J_USERNAME="neo4j"
NEO4J_PASSWORD="<tu-password>"
PORT=8080
```

---

<a name="-english"></a>
# 🇬🇧 English

## 📖 Overview
**Neo4j Industrial Graph Dashboard** is a high-performance web platform designed for visualization, auditing, and analysis of complex industrial assets. Its architecture combines the safety and speed of **Rust** on the backend with the power of **Neo4j** to model hierarchical relationships (Plants → Equipment → Materials).

The flagship feature is its **Universal AI Assistant**, capable of translating natural language questions into complex Cypher queries, supporting multiple LLM providers (Cloud & Local).

### ✨ Key Features
*   **Graph Visualization:** Interactive asset network rendering with `Vis.js`.
*   **Dynamic Management:** Queries are loaded from a JSON file without recompiling the server.
*   **Multi-Provider AI:** Native support for OpenAI, Groq, DeepSeek, and local models via Ollama.
*   **Secure Proxy:** The backend acts as a gateway to prevent CORS issues and protect API keys.

---

## 🤖 AI Assistant Configuration (Multi-Model)

The application includes a smart chat (floating button 🤖) that uses **Function Calling** to query the database. You can choose your preferred AI provider by clicking the settings button (⚙️) inside the chat.

### Configuration Options

#### 1. ⚡ Groq (Recommended for Speed)
Ideal for near-instant responses using Llama 3 models.
*   **Provider:** Select `Groq`.
*   **API Key:** Get a free one at [console.groq.com](https://console.groq.com).
*   **Model:** Defaults to `llama-3.3-70b-versatile`.

#### 2. 🧠 OpenAI (Standard)
The most robust and reliable option.
*   **Provider:** Select `OpenAI`.
*   **API Key:** Your OpenAI `sk-...` key.
*   **Model:** `gpt-4o` (default).

#### 3. 💻 Ollama (Local Privacy)
To run models on your own machine without sending data to the cloud.
*   **Provider:** Select `Ollama`.
*   **Requirements:** Have [Ollama](https://ollama.com/) installed.
*   **Technical Setup:**
    *   Run Ollama allowing origins: `OLLAMA_ORIGINS="*" ollama serve`
    *   **If using Docker:** URL must be `http://host.docker.internal:11434/v1/chat/completions`.
    *   **If running locally:** URL is `http://localhost:11434/v1/chat/completions`.
*   **Model:** Ensure you have pulled the model (e.g., `ollama pull llama3.2`).

#### 4. 🚀 DeepSeek (Cost-Effective & Powerful)
Excellent reasoning capabilities (V3) at a fraction of the cost.
*   **Provider:** Select `DeepSeek`.
*   **API Key:** Your key from `platform.deepseek.com`.

#### 5. 🔌 Anthropic (Claude) / Custom
To use Claude 3.5 Sonnet or other OpenAI-compatible providers.
*   **Provider:** Select `Custom`.
*   **Recommended Method:** Use **OpenRouter** as a middleware for API compatibility.
    *   **Base URL:** `https://openrouter.ai/api/v1/chat/completions`
    *   **Model:** `anthropic/claude-3.5-sonnet`

---

## 🧩 Query Management

No need to touch Rust code to add new analytics. Everything resides in `queries.json`.

```json
{
  "id": "M01",
  "category": "Maintenance",
  "title": "BOM Breakdown",
  "description": "Description the AI uses to understand when to use this tool.",
  "cypher": "MATCH (n)-[:REL]->(m) RETURN ...",
  "needs_param": true,  // true = Requires selecting a node first
  "is_graph": true,     // true = Renders nodes; false = Renders table/chart
  "icon": "fa-share-nodes"
}
```
*To add a query, simply edit this file and restart the server.*

---

## 🚀 Deployment

### Docker (Recommended)
```bash
# Build the image
docker build -t neo4j_dashboard .

# Run (Ensure you have the .env file configured)
docker run -p 8080:8080 --env-file .env neo4j_dashboard
```

### Local Execution (Rust)
```bash
# Install dependencies & Build
cargo build --release

# Run
./target/release/neo4j_dashboard
```

Required environment variables in `.env`:
```env
NEO4J_URI="neo4j+s://<your-instance>.databases.neo4j.io"
NEO4J_USERNAME="neo4j"
NEO4J_PASSWORD="<your-password>"
PORT=8080
```

---

<a name="-català"></a>
# 🏴 Català

## 📖 Descripció General
**Neo4j Industrial Graph Dashboard** és una plataforma web d'alt rendiment dissenyada per a la visualització, auditoria i anàlisi d'actius industrials complexos. La seva arquitectura combina la seguretat i velocitat de **Rust** al backend amb la potència de **Neo4j** per modelar relacions jeràrquiques (Plantes → Equips → Materials).

La característica estrella és el seu **Assistent d'IA Universal**, capaç de traduir preguntes en llenguatge natural a consultes Cypher complexes, suportant múltiples proveïdors de LLM (Núvol i Local).

### ✨ Característiques Principals
*   **Visualització de Grafs:** Renderització interactiva de xarxes d'actius amb `Vis.js`.
*   **Gestió Dinàmica:** Les consultes es carreguen des d'un fitxer JSON sense recompilar el servidor.
*   **Multi-Proveïdor d'IA:** Suport natiu per a OpenAI, Groq, DeepSeek i models locals amb Ollama.
*   **Proxy Segur:** El backend actua com a passarel·la per evitar problemes de CORS i protegir les claus API.

---

## 🤖 Configuració de l'Assistent d'IA (Multi-Model)

L'aplicació inclou un xat intel·ligent (botó flotant 🤖) que utilitza **Function Calling** per interrogar la base de dades. Pots triar el teu proveïdor d'IA preferit fent clic al botó de configuració (⚙️) dins del xat.

### Opcions de Configuració

#### 1. ⚡ Groq (Recomanat per Velocitat)
Ideal per a respostes gairebé instantànies utilitzant models Llama 3.
*   **Proveïdor:** Selecciona `Groq`.
*   **API Key:** Aconsegueix-ne una gratuïta a [console.groq.com](https://console.groq.com).
*   **Model:** Per defecte utilitza `llama-3.3-70b-versatile`.

#### 2. 🧠 OpenAI (Estàndard)
L'opció més robusta i fiable.
*   **Proveïdor:** Selecciona `OpenAI`.
*   **API Key:** La teva clau `sk-...` d'OpenAI.
*   **Model:** `gpt-4o` (per defecte).

#### 3. 💻 Ollama (Privadesa Local)
Per executar models a la teva pròpia màquina sense enviar dades al núvol.
*   **Proveïdor:** Selecciona `Ollama`.
*   **Requisits:** Tenir [Ollama](https://ollama.com/) instal·lat.
*   **Configuració Tècnica:**
    *   Executa Ollama permetent orígens: `OLLAMA_ORIGINS="*" ollama serve`
    *   **Si utilitzes Docker:** La URL ha de ser `http://host.docker.internal:11434/v1/chat/completions`.
    *   **Si executes en local:** La URL és `http://localhost:11434/v1/chat/completions`.
*   **Model:** Assegura't de tenir el model descarregat (ex. `ollama pull llama3.2`).

#### 4. 🚀 DeepSeek (Econòmic i Potent)
Excel·lent capacitat de raonament (V3) a una fracció del cost.
*   **Proveïdor:** Selecciona `DeepSeek`.
*   **API Key:** La teva clau de `platform.deepseek.com`.

#### 5. 🔌 Anthropic (Claude) / Custom
Per utilitzar Claude 3.5 Sonnet o altres proveïdors compatibles amb OpenAI.
*   **Proveïdor:** Selecciona `Custom`.
*   **Mètode Recomanat:** Utilitzar **OpenRouter** com a intermediari per a compatibilitat d'API.
    *   **Base URL:** `https://openrouter.ai/api/v1/chat/completions`
    *   **Model:** `anthropic/claude-3.5-sonnet`

---

## 🧩 Gestió de Consultes (Queries)

No cal tocar codi Rust per afegir noves analítiques. Tot resideix a `queries.json`.

```json
{
  "id": "M01",
  "category": "Manteniment",
  "title": "Desglossament BOM",
  "description": "Descripció que la IA utilitza per entendre quan fer servir aquesta eina.",
  "cypher": "MATCH (n)-[:REL]->(m) RETURN ...",
  "needs_param": true,  // true = Requereix seleccionar un node abans
  "is_graph": true,     // true = Renderitza nodes; false = Renderitza taula/gràfic
  "icon": "fa-share-nodes"
}
```
*Per afegir una consulta, simplement edita aquest fitxer i reinicia el servidor.*

---

## 🚀 Desplegament

### Docker (Recomanat)
```bash
# Construir la imatge
docker build -t neo4j_dashboard .

# Executar (Assegura't de tenir el fitxer .env configurat)
docker run -p 8080:8080 --env-file .env neo4j_dashboard
```

### Execució Local (Rust)
```bash
# Instal·lar dependències i compilar
cargo build --release

# Executar
./target/release/neo4j_dashboard
```

Variables d'entorn requerides a `.env`:
```env
NEO4J_URI="neo4j+s://<la-teva-instancia>.databases.neo4j.io"
NEO4J_USERNAME="neo4j"
NEO4J_PASSWORD="<el-teu-password>"
PORT=8080
```

---

### Author / Autor
**Angel A. Urbina**  
📧 [CV & Portfolio](https://angelurbinacv.netlify.app/)

### License
This project is licensed under the MIT License.
