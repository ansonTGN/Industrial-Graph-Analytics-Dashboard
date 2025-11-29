# 🏭 Neo4j Industrial Graph Analytics

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)
![Neo4j](https://img.shields.io/badge/Neo4j-Database-blue?logo=neo4j)
![Actix-Web](https://img.shields.io/badge/Framework-Actix_Web-green)
![License](https://img.shields.io/badge/License-MIT-lightgrey)

**[Español](#es) | [English](#en) | [Català](#ca)**

---

<a name="es"></a>
## 🇪🇸 Español

### 📖 Sobre el Proyecto
**Neo4j Industrial Analytics** es un panel de control web de alto rendimiento desarrollado en **Rust** (usando Actix-Web) diseñado para la visualización, análisis y auditoría de activos industriales complejos. La aplicación permite explorar relaciones jerárquicas entre Plantas, Equipos y Materiales almacenados en una base de datos de grafos Neo4j.

El sistema está optimizado para entornos de ingeniería, ofreciendo renderizado de grafos en tiempo real, análisis de listas de materiales (BOM) y detección de patrones logísticos.

### ✨ Características Principales
*   **🚀 Backend en Rust:** Latencia ultrabaixa y gestión segura de concurrencia.
*   **🕸 Visualización Interactiva:** Motor gráfico basado en `Vis.js`.
*   **📊 Analítica Modular:** Sistema de consultas dinámico cargado desde `queries.json`.
*   **📈 Gráficos Automáticos:** Histogramas estadísticos con `Chart.js`.
*   **🔍 Búsqueda Avanzada:** Autocompletado para Ubicaciones, Equipos y Repuestos.

### ⚙️ Gestión de Consultas (JSON)
El núcleo analítico reside en el archivo `queries.json`. Puedes añadir o modificar consultas sin recompiar el código Rust (solo requiere reiniciar la aplicación).

#### Estructura del Objeto JSON
```json
{
  "id": "C01",                  // Identificador único
  "category": "Mantenimiento",  // Agrupación en el menú lateral
  "title": "Título Visible",    // Nombre en la interfaz
  "description": "Tooltip...",  // Descripción al pasar el mouse
  "cypher": "MATCH ...",        // Código Cypher (ver abajo)
  "needs_param": true,          // true = Requiere seleccionar un nodo previo
  "is_graph": true,             // true = Renderiza Grafo, false = Tabla/Chart
  "icon": "fa-bolt"             // Clase de icono FontAwesome 6
}
```

#### Reglas para Cypher
1.  **Parámetro de Entrada:** Si `needs_param` es `true`, debes usar **`$p`** en tu consulta Cypher para referenciar el ID del nodo seleccionado.
    *   *Ejemplo:* `MATCH (n {id: $p})...`
2.  **Visualización de Grafos (`is_graph: true`):** Para que el visualizador de redes funcione, la consulta **DEBE** devolver exactamente estas columnas con estos alias:
    *   `A_ID`, `A_LABEL`, `A_TYPE` (Nodo Origen)
    *   `B_ID`, `B_LABEL`, `B_TYPE` (Nodo Destino)
    *   `RELACION` (Etiqueta de la arista)
3.  **Visualización de Tablas/Gráficos:** Devuelve cualquier columna. Si una columna es numérica (ej. `TOTAL`, `CANTIDAD`), se generará automáticamente un gráfico de barras.

### 🚀 Instalación y Uso
1.  **Clonar y Configurar:**
    ```bash
    git clone https://github.com/tu-usuario/neo4j-industrial-analytics.git
    cd neo4j-industrial-analytics
    # Crear archivo .env
    echo 'NEO4J_URI="bolt://localhost:7687"\nNEO4J_USERNAME="neo4j"\nNEO4J_PASSWORD="pass"' > .env
    ```
2.  **Ejecutar:**
    ```bash
    cargo run --release
    ```
3.  **Acceso:** Navegar a `http://localhost:8081`.

---

<a name="en"></a>
## 🇬🇧 English

### 📖 About the Project
**Neo4j Industrial Analytics** is a high-performance web dashboard built in **Rust** (Actix-Web) designed for visualizing, analyzing, and auditing complex industrial assets. The application allows users to explore hierarchical relationships between Plants, Equipment, and Materials stored in a Neo4j Graph Database.

The system is optimized for engineering environments, offering real-time graph rendering, Bill of Materials (BOM) analysis, and supply chain pattern detection.

### ✨ Key Features
*   **🚀 Rust Backend:** Ultra-low latency and safe concurrency management.
*   **🕸 Interactive Visualization:** Graph engine based on `Vis.js`.
*   **📊 Modular Analytics:** Dynamic query system loaded from `queries.json`.
*   **📈 Automatic Charting:** Statistical histograms via `Chart.js`.
*   **🔍 Advanced Search:** Autocomplete for Plants, Equipment, and Spares.

### ⚙️ Custom Query Management (JSON)
The analytical core is driven by `queries.json`. You can add or modify queries without recompiling the Rust code (application restart required).

#### JSON Object Structure
```json
{
  "id": "C01",                  // Unique ID
  "category": "Maintenance",    // Sidebar group name
  "title": "Visible Title",     // Interface name
  "description": "Tooltip...",  // Mouseover description
  "cypher": "MATCH ...",        // Cypher code (see below)
  "needs_param": true,          // true = Requires a target node selection
  "is_graph": true,             // true = Graph View, false = Table/Chart
  "icon": "fa-bolt"             // FontAwesome 6 icon class
}
```

#### Cypher Rules
1.  **Input Parameter:** If `needs_param` is `true`, you must use **`$p`** in your Cypher query to reference the selected node's ID.
    *   *Example:* `MATCH (n {id: $p})...`
2.  **Graph Visualization (`is_graph: true`):** For the network visualizer to work, the query **MUST** return exactly these columns with these aliases:
    *   `A_ID`, `A_LABEL`, `A_TYPE` (Source Node)
    *   `B_ID`, `B_LABEL`, `B_TYPE` (Target Node)
    *   `RELACION` (Edge Label)
3.  **Table/Chart Visualization:** Return any columns. If a column is numeric (e.g., `TOTAL`, `QUANTITY`), a bar chart will be automatically generated.

### 🚀 Installation & Usage
1.  **Clone & Setup:**
    ```bash
    git clone https://github.com/your-username/neo4j-industrial-analytics.git
    cd neo4j-industrial-analytics
    # Create .env file
    echo 'NEO4J_URI="bolt://localhost:7687"\nNEO4J_USERNAME="neo4j"\nNEO4J_PASSWORD="pass"' > .env
    ```
2.  **Run:**
    ```bash
    cargo run --release
    ```
3.  **Access:** Open browser at `http://localhost:8081`.

---

<a name="ca"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 Català

### 📖 Sobre el Projecte
**Neo4j Industrial Analytics** és un tauler de control web d'alt rendiment desenvolupat en **Rust** (fent servir Actix-Web), dissenyat per a la visualització, anàlisi i auditoria d'actius industrials complexos. L'aplicació permet explorar relacions jeràrquiques entre Plantes, Equips i Materials emmagatzemats en una base de dades de grafs Neo4j.

El sistema està optimitzat per a entorns d'enginyeria, oferint renderitzat de grafs en temps real, anàlisi de llistes de materials (BOM) i detecció de patrons logístics.

### ✨ Característiques Principals
*   **🚀 Backend en Rust:** Latència ultrabaixa i gestió segura de concurrència.
*   **🕸 Visualització Interactiva:** Motor gràfic basat en `Vis.js`.
*   **📊 Analítica Modular:** Sistema de consultes dinàmic carregat des de `queries.json`.
*   **📈 Gràfics Automàtics:** Histogrames estadístics amb `Chart.js`.
*   **🔍 Cerca Avançada:** Autocompletat per a Plantes, Equips i Recanvis.

### ⚙️ Gestió de Consultes (JSON)
El nucli analític resideix al fitxer `queries.json`. Pots afegir o modificar consultes sense recompilar el codi Rust (només cal reiniciar l'aplicació).

#### Estructura de l'Objecte JSON
```json
{
  "id": "C01",                  // Identificador únic
  "category": "Manteniment",    // Agrupació al menú lateral
  "title": "Títol Visible",     // Nom a la interfície
  "description": "Tooltip...",  // Descripció en passar el ratolí
  "cypher": "MATCH ...",        // Codi Cypher (veure a sota)
  "needs_param": true,          // true = Requereix seleccionar un node previ
  "is_graph": true,             // true = Renderitza Graf, false = Taula/Chart
  "icon": "fa-bolt"             // Classe d'icona FontAwesome 6
}
```

#### Regles per a Cypher
1.  **Paràmetre d'Entrada:** Si `needs_param` és `true`, has d'utilitzar **`$p`** a la teva consulta Cypher per referenciar l'ID del node seleccionat.
    *   *Exemple:* `MATCH (n {id: $p})...`
2.  **Visualització de Grafs (`is_graph: true`):** Perquè el visualitzador de xarxes funcioni, la consulta **HA DE** retornar exactament aquestes columnes amb aquests àlies:
    *   `A_ID`, `A_LABEL`, `A_TYPE` (Node Origen)
    *   `B_ID`, `B_LABEL`, `B_TYPE` (Node Destí)
    *   `RELACION` (Etiqueta de l'aresta)
3.  **Visualització de Taules/Gràfics:** Retorna qualsevol columna. Si una columna és numèrica (ex. `TOTAL`, `QUANTITAT`), es generarà automàticament un gràfic de barres.

### 🚀 Instal·lació i Ús
1.  **Clonar i Configurar:**
    ```bash
    git clone https://github.com/el-teu-usuari/neo4j-industrial-analytics.git
    cd neo4j-industrial-analytics
    # Crear fitxer .env
    echo 'NEO4J_URI="bolt://localhost:7687"\nNEO4J_USERNAME="neo4j"\nNEO4J_PASSWORD="pass"' > .env
    ```
2.  **Executar:**
    ```bash
    cargo run --release
    ```
3.  **Accés:** Navega a `http://localhost:8081`.

---

### Author / Autor
**Angel A. Urbina**

### License
This project is licensed under the MIT License - see the LICENSE file for details.