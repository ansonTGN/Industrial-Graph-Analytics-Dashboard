# Industrial Graph Analytics Dashboard

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)
![Neo4j](https://img.shields.io/badge/Neo4j-Database-blue?logo=neo4j)
![Actix-Web](https://img.shields.io/badge/Framework-Actix--Web-green)
![Status](https://img.shields.io/badge/Status-Beta-yellow)

**[English](#english) | [Español](#español)**

---

<a name="english"></a>
## 🇬🇧 English

### About the Project
**Neo4j Industrial Analytics** is a high-performance web dashboard built with **Rust** and **Actix-Web**. It is designed to visualize, analyze, and audit industrial assets (Petrochemical Systems) stored in a Neo4j Graph Database.

The application bridges the gap between raw graph data and engineering decision-making, offering specialized views for Maintenance, Supply Chain, and Engineering teams.

### Key Features
*   **🚀 High Performance:** Backend built in Rust for low latency and high concurrency.
*   **🕸 Interactive Graph Visualization:** Real-time rendering of nodes and relationships using `Vis.js`.
*   **📊 Integrated Analytics:** 40+ pre-defined Cypher queries categorized by operational role.
*   **📈 Data Visualization:** Automatic generation of bar charts for numerical data.
*   **💾 Export Capabilities:** Download graph snapshots (PNG) and tabular data (CSV).
*   **🔍 Advanced Search:** Autocomplete search for specific Materials, Equipment, or Technical Locations.

### Use Cases
The dashboard is pre-configured with queries for:
1.  **Maintenance & Reliability:** BOM analysis, spare parts pooling, and obsolescence impact.
2.  **Supply Chain:** High rotation items, "slow movers," and stock coverage.
3.  **Engineering:** Hierarchical trees, circular dependency detection, and isolate systems identification.
4.  **Management KPIs:** Global asset summaries and data quality auditing.

### Tech Stack
*   **Backend:** Rust, Actix-Web 4.4, Neo4rs (Bolt Driver).
*   **Frontend:** HTML5 (Tera Templates), Bootstrap 5.3, FontAwesome 6.
*   **Visualization:** Vis.js (Network), Chart.js (Statistics).

### Data Model Requirements
To use this dashboard effectively, your Neo4j database should follow this schema:
*   **Nodes Labels:** `UbicacionTecnica` (Plant/Area), `Equipo` (Equipment), `Material` (Spare Part).
*   **Relationships:** `[:CONTIENE]` (CONTAINS).
*   **Properties:** `id`, `name`, `quantity`, `price`, `status_flag`.

### Installation & Setup

1.  **Clone the repository**
    ```bash
    git clone https://github.com/your-username/neo4j_dashboard.git
    cd neo4j_dashboard
    ```

2.  **Environment Configuration**
    Create a `.env` file in the root directory (or modify the existing one):
    ```env
    NEO4J_URI="bolt://localhost:7687"
    NEO4J_USERNAME="neo4j"
    NEO4J_PASSWORD="your_password"
    ```

3.  **Run the Application**
    ```bash
    cargo run
    ```

4.  **Access**
    Open your browser and navigate to: `http://localhost:8081`

---

<a name="español"></a>
## 🇪🇸 Español

### Sobre el Proyecto
**Neo4j Industrial Analytics** es un panel de control web de alto rendimiento construido con **Rust** y **Actix-Web**. Está diseñado para visualizar, analizar y auditar activos industriales (Sistemas Petroquímicos) almacenados en una base de datos orientada a grafos Neo4j.

La aplicación conecta los datos crudos del grafo con la toma de decisiones de ingeniería, ofreciendo vistas especializadas para equipos de Mantenimiento, Compras e Ingeniería.

### Características Principales
*   **🚀 Alto Rendimiento:** Backend en Rust que garantiza baja latencia y alta concurrencia.
*   **🕸 Visualización Interactiva:** Renderizado en tiempo real de nodos y relaciones usando `Vis.js`.
*   **📊 Analítica Integrada:** Más de 40 consultas Cypher predefinidas y categorizadas por rol operativo.
*   **📈 Gráficos Estadísticos:** Generación automática de gráficos de barras para datos numéricos.
*   **💾 Exportación:** Descarga de capturas del grafo (PNG) y datos tabulares (CSV).
*   **🔍 Búsqueda Avanzada:** Buscador con autocompletado para Materiales, Equipos o Ubicaciones Técnicas.

### Casos de Uso
El dashboard incluye consultas preconfiguradas para:
1.  **Mantenimiento y Confiabilidad:** Análisis de listas de materiales (BOM), intercambiabilidad de repuestos e impacto de obsolescencia.
2.  **Cadena de Suministro:** Artículos de alta rotación, inventario inmovilizado y cobertura de stock.
3.  **Ingeniería:** Árboles jerárquicos, detección de dependencias circulares y sistemas aislados.
4.  **KPIs de Gestión:** Resúmenes globales de activos y auditoría de calidad de datos.

### Stack Tecnológico
*   **Backend:** Rust, Actix-Web 4.4, Neo4rs (Driver Bolt).
*   **Frontend:** HTML5 (Plantillas Tera), Bootstrap 5.3, FontAwesome 6.
*   **Visualización:** Vis.js (Redes), Chart.js (Estadísticas).

### Requisitos del Modelo de Datos
Para utilizar este dashboard, su base de datos Neo4j debe seguir este esquema:
*   **Etiquetas de Nodos:** `UbicacionTecnica`, `Equipo`, `Material`.
*   **Relaciones:** `[:CONTIENE]`.
*   **Propiedades clave:** `id`, `name`, `quantity`, `price`, `status_flag`.

### Instalación y Ejecución

1.  **Clonar el repositorio**
    ```bash
    git clone https://github.com/tu-usuario/neo4j_dashboard.git
    cd neo4j_dashboard
    ```

2.  **Configuración de Entorno**
    Crea un archivo `.env` en la raíz (o modifica el existente):
    ```env
    NEO4J_URI="bolt://localhost:7687"
    NEO4J_USERNAME="neo4j"
    NEO4J_PASSWORD="tu_password"
    ```

3.  **Ejecutar la Aplicación**
    ```bash
    cargo run
    ```

4.  **Acceso**
    Abre tu navegador y ve a: `http://localhost:8081`

---

## Author / Autor
**Angel A. Urbina**
*Engineer & Developer*

## License
This project is licensed under the MIT License - see the LICENSE file for details.