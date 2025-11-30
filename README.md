# Neo4j Industrial Graph Dashboard
**Analítica Avanzada de Activos Industriales con Rust + Neo4j + IA**

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)
![Neo4j](https://img.shields.io/badge/Neo4j-Aura%20%2F%20Local-blue?logo=neo4j)
![Actix-Web](https://img.shields.io/badge/Backend-Actix_Web-green)
![OpenAI](https://img.shields.io/badge/AI-OpenAI%20Integration-purple?logo=openai)
![License](https://img.shields.io/badge/License-MIT-lightgrey)

---

**Idiomas / Languages / Idiomes:**  
[🇪🇸 Español](#-español) | [🇬🇧 English](#-english) | [🏴 Català](#-català)

---

<a name="-español"></a>
# 🇪🇸 Español

## 📖 Descripción General
**Neo4j Industrial Graph Dashboard** es una plataforma web de alto rendimiento para la visualización, auditoría y análisis de activos industriales complejos. Utiliza **Rust** para el backend, garantizando velocidad y seguridad, y **Neo4j** como base de datos de grafos para modelar relaciones jerárquicas (Plantas → Equipos → Materiales).

La característica más potente es su **Asistente de IA Integrado**, que permite interrogar a la base de datos utilizando lenguaje natural, transformando preguntas complejas en ejecuciones de datos precisas.

---

## 🧩 Gestión de Consultas (Queries)

El núcleo de la aplicación es dinámico. No es necesario modificar el código fuente en Rust para añadir nuevas analíticas; todo se gestiona desde el archivo `queries.json`.

Este archivo alimenta tanto el **Menú Lateral** de la interfaz como las **Herramientas (Tools)** disponibles para la Inteligencia Artificial.

### Estructura de una Consulta
Cada objeto en `queries.json` debe seguir este esquema:

```json
{
  "id": "M01",                  // Identificador único (usado por la API y la IA)
  "category": "Mantenimiento",  // Agrupación en el menú visual
  "title": "Desglose BOM",      // Nombre visible para el usuario
  "description": "Visualiza...",// Descripción para el usuario y contexto para la IA
  "cypher": "MATCH ...",        // La consulta Cypher a ejecutar en Neo4j
  "needs_param": true,          // true: Requiere un ID de nodo ($p). false: Consulta global.
  "is_graph": true,             // true: Renderiza nodos/aristas. false: Renderiza Tabla/Gráfico.
  "icon": "fa-share-nodes"      // Clase de icono FontAwesome 6
}
```

### ➕ Cómo añadir una nueva consulta
1. Abra el archivo `queries.json` en la raíz del proyecto.
2. Agregue un nuevo objeto al array JSON.
3. **Importante:** Si la consulta requiere un parámetro (ej. buscar hijos de un equipo específico), use `$p` en el código Cypher y establezca `"needs_param": true`.
4. Guarde el archivo.
5. **Reinicie la aplicación**. El sistema cargará la nueva consulta, la añadirá al menú y la registrará automáticamente como una nueva habilidad para el Asistente de IA.

### ➖ Cómo eliminar o modificar
* **Eliminar:** Simplemente borre el bloque JSON correspondiente y reinicie.
* **Modificar:** Edite el campo `cypher` o `description` y reinicie. Los cambios se reflejarán instantáneamente en la capacidad de razonamiento de la IA.

---

## 🤖 Uso de la Inteligencia Artificial

La aplicación incluye un chat inteligente (botón flotante 🤖) capaz de razonar sobre los datos industriales.

### ¿Cómo funciona?
1. **Trae tu propia clave (BYOK):** Al abrir el chat, se solicitará una API Key de OpenAI (`sk-...`). Esta clave se guarda **localmente en su navegador** (LocalStorage) y nunca se almacena en el servidor.
2. **Definición de Herramientas:** El backend convierte automáticamente las consultas de `queries.json` en "Tools" de OpenAI.
3. **Razonamiento:** Cuando usted pregunta *"¿Qué equipos tienen riesgo de obsolescencia?"*:
    * La IA analiza la pregunta.
    * Busca en su lista de herramientas y selecciona `M04` (Impacto Obsolescencia).
    * Solicita al servidor ejecutar esa consulta.
    * El servidor devuelve los datos JSON.
    * La IA interpreta los datos y le responde: *"He encontrado 5 equipos en riesgo, incluyendo la Bomba P-201..."*.

### Notas de Seguridad
* Las peticiones a OpenAI pasan a través de un **Proxy en el servidor Rust** (`/api/openai_proxy`) para evitar errores de CORS y proteger la comunicación.
* La IA solo tiene acceso de lectura a los datos que exponen las consultas definidas en `queries.json`.

---

## 🚀 Despliegue en Render / Docker

El proyecto está optimizado para contenedores.

```bash
# Construir y ejecutar localmente
docker build -t neo4j_dashboard .
docker run -p 8080:8080 --env-file .env neo4j_dashboard
```

Variables de entorno requeridas (`.env`):
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
**Neo4j Industrial Graph Dashboard** is a high-performance web platform for visualization, auditing, and analysis of complex industrial assets. It uses **Rust** for the backend, ensuring speed and safety, and **Neo4j** as a graph database to model hierarchical relationships (Plants → Equipment → Materials).

Its most powerful feature is the **Integrated AI Assistant**, which allows users to query the database using natural language, transforming complex questions into precise data executions.

---

## 🧩 Query Management

The core of the application is dynamic. There is no need to modify the Rust source code to add new analytics; everything is managed via the `queries.json` file.

This file powers both the **Sidebar Menu** and the **Tools** available to the Artificial Intelligence.

### Query Structure
Each object in `queries.json` must follow this schema:

```json
{
  "id": "M01",                  // Unique ID (used by API and AI)
  "category": "Maintenance",    // Grouping in the visual menu
  "title": "BOM Breakdown",     // Visible name for the user
  "description": "Visualizes...",// Description for user & context for AI
  "cypher": "MATCH ...",        // The Cypher query to execute in Neo4j
  "needs_param": true,          // true: Requires a node ID ($p). false: Global query.
  "is_graph": true,             // true: Renders nodes/edges. false: Renders Table/Chart.
  "icon": "fa-share-nodes"      // FontAwesome 6 icon class
}
```

### ➕ How to add a new query
1. Open the `queries.json` file in the project root.
2. Add a new object to the JSON array.
3. **Important:** If the query requires a parameter (e.g., finding children of a specific equipment), use `$p` in the Cypher code and set `"needs_param": true`.
4. Save the file.
5. **Restart the application**. The system will load the new query, add it to the menu, and automatically register it as a new skill for the AI Assistant.

### ➖ How to remove or modify
* **Remove:** Simply delete the corresponding JSON block and restart.
* **Modify:** Edit the `cypher` or `description` field and restart. Changes will instantly reflect in the AI's reasoning capabilities.

---

## 🤖 AI Usage

The application includes a smart chat (floating button 🤖) capable of reasoning over industrial data.

### How does it work?
1. **Bring Your Own Key (BYOK):** Upon opening the chat, you will be asked for an OpenAI API Key (`sk-...`). This key is stored **locally in your browser** (LocalStorage) and is never stored on the server.
2. **Tool Definition:** The backend automatically converts queries from `queries.json` into OpenAI "Tools".
3. **Reasoning:** When you ask *"Which equipment is at risk of obsolescence?"*:
    * The AI analyzes the question.
    * It searches its tool list and selects `M04` (Obsolescence Impact).
    * It requests the server to execute that query.
    * The server returns raw JSON data.
    * The AI interprets the data and answers: *"I found 5 equipment items at risk, including Pump P-201..."*.

### Security Notes
* Requests to OpenAI pass through a **Rust Server Proxy** (`/api/openai_proxy`) to prevent CORS errors and secure communication.
* The AI only has read access to data exposed by the queries defined in `queries.json`.

---

## 🚀 Deployment on Render / Docker

The project is container-optimized.

```bash
# Build and run locally
docker build -t neo4j_dashboard .
docker run -p 8080:8080 --env-file .env neo4j_dashboard
```

Required environment variables (`.env`):
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
**Neo4j Industrial Graph Dashboard** és una plataforma web d'alt rendiment per a la visualització, auditoria i anàlisi d'actius industrials complexos. Utilitza **Rust** per al backend, garantint velocitat i seguretat, i **Neo4j** com a base de dades de grafs per modelar relacions jeràrquiques (Plantes → Equips → Materials).

La característica més potent és el seu **Assistent d'IA Integrat**, que permet interrogar la base de dades utilitzant llenguatge natural, transformant preguntes complexes en execucions de dades precises.

---

## 🧩 Gestió de Consultes (Queries)

El nucli de l'aplicació és dinàmic. No cal modificar el codi font en Rust per afegir noves analítiques; tot es gestiona des del fitxer `queries.json`.

Aquest fitxer alimenta tant el **Menú Lateral** de la interfície com les **Eines (Tools)** disponibles per a la Intel·ligència Artificial.

### Estructura d'una Consulta
Cada objecte a `queries.json` ha de seguir aquest esquema:

```json
{
  "id": "M01",                  // Identificador únic (usat per l'API i la IA)
  "category": "Manteniment",    // Agrupació al menú visual
  "title": "Desglossament BOM", // Nom visible per a l'usuari
  "description": "Visualitza...",// Descripció per l'usuari i context per a la IA
  "cypher": "MATCH ...",        // La consulta Cypher a executar a Neo4j
  "needs_param": true,          // true: Requereix un ID de node ($p). false: Consulta global.
  "is_graph": true,             // true: Renderitza nodes/arestes. false: Renderitza Taula/Gràfic.
  "icon": "fa-share-nodes"      // Classe d'icona FontAwesome 6
}
```

### ➕ Com afegir una nova consulta
1. Obriu el fitxer `queries.json` a l'arrel del projecte.
2. Afegiu un nou objecte a l'array JSON.
3. **Important:** Si la consulta requereix un paràmetre (ex. buscar fills d'un equip específic), utilitzeu `$p` al codi Cypher i establiu `"needs_param": true`.
4. Deseu el fitxer.
5. **Reinicieu l'aplicació**. El sistema carregarà la nova consulta, l'afegirà al menú i la registrarà automàticament com una nova habilitat per a l'Assistent d'IA.

### ➖ Com eliminar o modificar
* **Eliminar:** Simplement esborreu el bloc JSON corresponent i reinicieu.
* **Modificar:** Editeu el camp `cypher` o `description` i reinicieu. Els canvis es reflectiran instantàniament en la capacitat de raonament de la IA.

---

## 🤖 Ús de la Intel·ligència Artificial

L'aplicació inclou un xat intel·ligent (botó flotant 🤖) capaç de raonar sobre les dades industrials.

### Com funciona?
1. **Porta la teva pròpia clau (BYOK):** En obrir el xat, se sol·licitarà una API Key d'OpenAI (`sk-...`). Aquesta clau es desa **localment al vostre navegador** (LocalStorage) i mai s'emmagatzema al servidor.
2. **Definició d'Eines:** El backend converteix automàticament les consultes de `queries.json` en "Tools" d'OpenAI.
3. **Raonament:** Quan pregunteu *"Quins equips tenen risc d'obsolescència?"*:
    * La IA analitza la pregunta.
    * Cerca a la seva llista d'eines i selecciona `M04` (Impacte Obsolescència).
    * Sol·licita al servidor executar aquesta consulta.
    * El servidor retorna les dades JSON.
    * La IA interpreta les dades i respon: *"He trobat 5 equips en risc, incloent-hi la Bomba P-201..."*.

### Notes de Seguretat
* Les peticions a OpenAI passen a través d'un **Proxy al servidor Rust** (`/api/openai_proxy`) per evitar errors de CORS i protegir la comunicació.
* La IA només té accés de lectura a les dades que exposen les consultes definides a `queries.json`.

---

## 🚀 Desplegament a Render / Docker

El projecte està optimitzat per a contenidors.

```bash
# Construir i executar localment
docker build -t neo4j_dashboard .
docker run -p 8080:8080 --env-file .env neo4j_dashboard
```

Variables d'entorn requerides (`.env`):
```env
NEO4J_URI="neo4j+s://<la-teva-instancia>.databases.neo4j.io"
NEO4J_USERNAME="neo4j"
NEO4J_PASSWORD="<el-teu-password>"
PORT=8080
```

---

### Author / Autor
**Angel A. Urbina**

### License
This project is licensed under the MIT License.