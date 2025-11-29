# 🏭 Neo4j Industrial Graph Analytics

![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)
![Neo4j](https://img.shields.io/badge/Neo4j-Database-blue?logo=neo4j)
![Actix-Web](https://img.shields.io/badge/Framework-Actix_Web-green)
![License](https://img.shields.io/badge/License-MIT-lightgrey)

<!-- GALERÍA DE IMÁGENES -->
<div align="center">
  <img src="https://github.com/ansonTGN/Industrial-Graph-Analytics-Dashboard/blob/main/IMG/AAP-00.png?raw=true" width="48%" alt="Vista Grafo Interactivo" style="border-radius: 8px; margin-right: 5px;">
  <img src="https://github.com/ansonTGN/Industrial-Graph-Analytics-Dashboard/blob/main/IMG/AAP-01.png?raw=true" width="48%" alt="Vista Tabular de Datos" style="border-radius: 8px; margin-left: 5px;">
  <br>
  <p><em>Dashboard Views: Interactive Graph Explorer & Data Grid</em></p>
</div>

---

**[Español](#es) | [English](#en) | [Català](#ca)**

---

<a name="es"></a>
## 🇪🇸 Español

### 📖 Sobre el Proyecto
**Neo4j Industrial Analytics** es un panel de control web de alto rendimiento desarrollado en **Rust** (usando Actix-Web) diseñado para la visualización, análisis y auditoría de activos industriales complejos. La aplicación permite explorar relaciones jerárquicas entre Plantas, Equipos y Materiales almacenados en una base de datos de grafos Neo4j.

El sistema está optimizado para entornos de ingeniería, ofreciendo renderizado de grafos en tiempo real, análisis de listas de materiales (BOM) y detección de patrones logísticos.

### ✨ Características Principales
*   **🚀 Backend en Rust:** Latencia ultrabaja y gestión segura de concurrencia.
*   **🕸 Visualización Interactiva:** Motor gráfico basado en `Vis.js`.
*   **📊 Analítica Modular:** Sistema de consultas dinámico cargado desde `queries.json`.
*   **📈 Gráficos Automáticos:** Histogramas estadísticos con `Chart.js`.
*   **🔍 Búsqueda Avanzada:** Autocompletado para Ubicaciones, Equipos y Repuestos.

### ⚙️ Gestión de Consultas (JSON)
El núcleo analítico reside en el archivo `queries.json`. Puedes añadir o modificar consultas sin recompilar el código Rust (solo requiere reiniciar la aplicación).

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