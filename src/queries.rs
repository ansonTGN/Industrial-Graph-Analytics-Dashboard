use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct QueryDefinition {
    pub id: &'static str,
    pub category: &'static str,     // Rol: Mantenimiento, Compras, etc.
    pub title: &'static str,        // Título corto para el menú
    pub description: &'static str,  // Descripción larga (tooltip/header)
    pub cypher: &'static str,       // Código Cypher
    pub needs_param: bool,          // ¿Requiere seleccionar un nodo (ID)?
    pub is_graph: bool,             // ¿Debe renderizarse con Vis.js?
    pub icon: &'static str,         // Icono de FontAwesome
}

pub fn get_all_queries() -> Vec<QueryDefinition> {
    vec![
        // =========================================================
        // 1. MANTENIMIENTO Y CONFIABILIDAD (12 Consultas)
        // =========================================================
        QueryDefinition {
            id: "M01", category: "Mantenimiento", title: "Desglose BOM (Gráfico)",
            description: "Visualización gráfica de la estructura de materiales de un equipo.",
            cypher: r#"MATCH (root {id: $p})-[r:CONTIENE]->(m:Material) 
                       RETURN root.id as A_ID, root.name as A_LABEL, labels(root)[0] as A_TYPE, m.id as B_ID, m.name as B_LABEL, labels(m)[0] as B_TYPE, m.quantity as CANTIDAD
                       UNION ALL
                       MATCH (root {id: $p})-[:CONTIENE]->(sub)-[r:CONTIENE]->(m:Material)
                       RETURN sub.id as A_ID, sub.name as A_LABEL, labels(sub)[0] as A_TYPE, m.id as B_ID, m.name as B_LABEL, labels(m)[0] as B_TYPE, m.quantity as CANTIDAD"#,
            needs_param: true, is_graph: true, icon: "fa-share-nodes"
        },
        QueryDefinition {
            id: "M02", category: "Mantenimiento", title: "Intercambiabilidad",
            description: "Equipos distintos que utilizan el mismo repuesto (Pooling de repuestos).",
            cypher: r#"MATCH (m:Material {id: $p})
                       MATCH (e1:Equipo)-[:CONTIENE]->(m)<-[:CONTIENE]-(e2:Equipo)
                       WHERE e1.id <> e2.id
                       RETURN e1.id as A_ID, e1.name as A_LABEL, "Equipo" as A_TYPE, m.id as B_ID, m.name as B_LABEL, "Repuesto" as B_TYPE, "COMPARTIDO" as RELACION LIMIT 100"#,
            needs_param: true, is_graph: true, icon: "fa-random"
        },
        QueryDefinition {
            id: "M03", category: "Mantenimiento", title: "Dónde se usa (Listado)",
            description: "Listado tabular de todos los equipos que instalan este material.",
            cypher: r#"MATCH (m:Material {id: $p}) MATCH (e:Equipo)-[:CONTIENE*]->(m) 
                       OPTIONAL MATCH (ut:UbicacionTecnica)-[:CONTIENE*]->(e)
                       RETURN e.id as EQUIPO, e.name as DESC_EQUIPO, ut.id as UBICACION, m.quantity as CANTIDAD"#,
            needs_param: true, is_graph: false, icon: "fa-map-location-dot"
        },
        QueryDefinition {
            id: "M04", category: "Mantenimiento", title: "Impacto Obsolescencia",
            description: "Visualiza el riesgo de propagación de materiales obsoletos en la planta.",
            cypher: r#"MATCH (m:Material) WHERE m.status_flag = 'X'
                       MATCH (e:Equipo)-[:CONTIENE]->(m)
                       RETURN e.id as A_ID, e.name as A_LABEL, "Equipo" as A_TYPE, m.id as B_ID, m.name as B_LABEL, "Obsoleto" as B_TYPE, "RIESGO" as RELACION LIMIT 200"#,
            needs_param: false, is_graph: true, icon: "fa-triangle-exclamation"
        },
        QueryDefinition {
            id: "M05", category: "Mantenimiento", title: "Maestro Materiales Planta",
            description: "Listado completo de materiales necesarios para una parada de planta (por ubicación).",
            cypher: r#"MATCH (u:UbicacionTecnica {id: $p})-[:CONTIENE*]->(e:Equipo)-[:CONTIENE]->(m:Material)
                       RETURN u.id as PLANTA, e.id as EQUIPO, m.id as MATERIAL, m.name as DESCRIPCION, sum(toInteger(m.quantity)) as TOTAL
                       ORDER BY EQUIPO LIMIT 1000"#,
            needs_param: true, is_graph: false, icon: "fa-clipboard-list"
        },
        QueryDefinition {
            id: "M06", category: "Mantenimiento", title: "Equipos sin Repuestos",
            description: "Equipos críticos que no tienen BOM definida en el sistema.",
            cypher: r#"MATCH (e:Equipo) WHERE NOT (e)-[:CONTIENE]->(:Material)
                       RETURN e.id as ID, e.name as EQUIPO, labels(e)[0] as TIPO LIMIT 100"#,
            needs_param: false, is_graph: false, icon: "fa-battery-empty"
        },
        QueryDefinition {
            id: "M07", category: "Mantenimiento", title: "Top 20 Equipos Críticos",
            description: "Equipos con mayor diversidad de componentes (Complejidad de mantenimiento).",
            cypher: r#"MATCH (e:Equipo)-[:CONTIENE]->(m:Material)
                       RETURN e.id as EQUIPO, e.name as NOMBRE, count(DISTINCT m) as DIVERSIDAD_PARTES
                       ORDER BY DIVERSIDAD_PARTES DESC LIMIT 20"#,
            needs_param: false, is_graph: false, icon: "fa-trophy"
        },
        QueryDefinition {
            id: "M08", category: "Mantenimiento", title: "Bombas y Motores",
            description: "Filtrado específico de equipos rotativos y sus componentes.",
            cypher: r#"MATCH (e:Equipo) WHERE e.name CONTAINS 'BOMBA' OR e.name CONTAINS 'MOTOR'
                       MATCH (e)-[:CONTIENE]->(m:Material)
                       RETURN e.id as TAG, e.name as DESCRIPCION, count(m) as NUM_REPUESTOS LIMIT 50"#,
            needs_param: false, is_graph: false, icon: "fa-fan"
        },
        QueryDefinition {
            id: "M09", category: "Mantenimiento", title: "Materiales Rotables",
            description: "Identificación de componentes reparables (Flag R o por nombre).",
            cypher: r#"MATCH (m:Material) WHERE m.name CONTAINS 'ROTOR' OR m.name CONTAINS 'ESTATOR' OR m.type = 'ROTABLE'
                       RETURN m.id as CODIGO, m.name as DESCRIPCION, m.price as PRECIO_EST LIMIT 50"#,
            needs_param: false, is_graph: false, icon: "fa-rotate"
        },
        QueryDefinition {
            id: "M10", category: "Mantenimiento", title: "Kits de Mantenimiento",
            description: "Agrupación de sellos, juntas y rodamientos por equipo.",
            cypher: r#"MATCH (e:Equipo {id: $p})-[:CONTIENE]->(m:Material)
                       WHERE m.name CONTAINS 'SELLO' OR m.name CONTAINS 'JUNTA' OR m.name CONTAINS 'RODAMIENTO'
                       RETURN e.name as EQUIPO, m.id as MATERIAL, m.name as COMPONENTE, m.quantity as CANT"#,
            needs_param: true, is_graph: false, icon: "fa-toolbox"
        },
        QueryDefinition {
            id: "M11", category: "Mantenimiento", title: "Entorno Operativo 360",
            description: "Qué rodea al equipo seleccionado (Padres, hijos y materiales).",
            cypher: r#"MATCH (c {id: $p})-[r]-(n)
                       RETURN c.id as A_ID, c.name as A_LABEL, labels(c)[0] as A_TYPE, n.id as B_ID, n.name as B_LABEL, labels(n)[0] as B_TYPE, type(r) as RELACION"#,
            needs_param: true, is_graph: true, icon: "fa-arrows-to-circle"
        },
        QueryDefinition {
            id: "M12", category: "Mantenimiento", title: "Análisis de Fallo Potencial",
            description: "Si falla este material, ¿cuántos equipos se detienen?",
            cypher: r#"MATCH (m:Material {id: $p})<-[:CONTIENE]-(e:Equipo)
                       RETURN m.name as CAUSA_RAIZ, count(e) as EQUIPOS_AFECTADOS, collect(e.id) as LISTA_TAGS"#,
            needs_param: true, is_graph: false, icon: "fa-bomb"
        },

        // =========================================================
        // 2. CADENA DE SUMINISTRO (10 Consultas)
        // =========================================================
        QueryDefinition {
            id: "S01", category: "Compras y Almacén", title: "Alta Rotación (Top 20)",
            description: "Materiales presentes en más equipos (Alta demanda teórica).",
            cypher: r#"MATCH (m:Material)<-[:CONTIENE]-(e:Equipo)
                       RETURN m.id as CODIGO, m.name as DESCRIPCION, count(DISTINCT e) as EQUIPOS_USO
                       ORDER BY EQUIPOS_USO DESC LIMIT 20"#,
            needs_param: false, is_graph: false, icon: "fa-fire"
        },
        QueryDefinition {
            id: "S02", category: "Compras y Almacén", title: "Cluster de Piezas",
            description: "Visualizar qué equipos comparten un repuesto específico.",
            cypher: r#"MATCH (m:Material {id: $p})<-[r:CONTIENE]-(e:Equipo)
                       RETURN m.id as A_ID, m.name as A_LABEL, "Material" as A_TYPE, e.id as B_ID, e.name as B_LABEL, "Equipo" as B_TYPE, r.quantity as CANTIDAD"#,
            needs_param: true, is_graph: true, icon: "fa-circle-nodes"
        },
        QueryDefinition {
            id: "S03", category: "Compras y Almacén", title: "Riesgo Stock Único",
            description: "Slow Movers: Materiales que solo se usan en 1 equipo en toda la planta.",
            cypher: r#"MATCH (m:Material)<-[:CONTIENE]-(e:Equipo)
                       WITH m, count(e) as uso WHERE uso = 1
                       MATCH (e)-[:CONTIENE]->(m)
                       RETURN m.id as MATERIAL, m.name as DESCRIPCION, e.name as UNICO_USUARIO LIMIT 50"#,
            needs_param: false, is_graph: false, icon: "fa-snowflake"
        },
        QueryDefinition {
            id: "S04", category: "Compras y Almacén", title: "Demanda Global Agregada",
            description: "Suma total de unidades instaladas de un material en toda la compañía.",
            cypher: r#"MATCH (m:Material {id: $p})<-[r:CONTIENE]-(e:Equipo)
                       RETURN m.id as CODIGO, m.name as DESCRIPCION, sum(toInteger(r.quantity)) as TOTAL_INSTALADO, count(e) as NUM_EQUIPOS"#,
            needs_param: true, is_graph: false, icon: "fa-calculator"
        },
        QueryDefinition {
            id: "S05", category: "Compras y Almacén", title: "Estandarización Multi-Planta",
            description: "Materiales que se usan en más de una planta (Oportunidad compra corporativa).",
            cypher: r#"MATCH (ut:UbicacionTecnica)-[:CONTIENE*]->(m:Material)
                       WITH m, collect(DISTINCT ut.id) as plantas WHERE size(plantas) > 1
                       RETURN m.id as MATERIAL, m.name as DESC, size(plantas) as NUM_PLANTAS, plantas as LISTA LIMIT 20"#,
            needs_param: false, is_graph: false, icon: "fa-building"
        },
        QueryDefinition {
            id: "S06", category: "Compras y Almacén", title: "Posibles Duplicados",
            description: "Materiales con el mismo nombre descriptivo pero distinto ID.",
            cypher: r#"MATCH (m:Material) WITH m.name as n, collect(m.id) as ids, count(*) as c WHERE c > 1
                       RETURN n as NOMBRE_DUPLICADO, c as VECES, ids as CODIGOS ORDER BY c DESC LIMIT 20"#,
            needs_param: false, is_graph: false, icon: "fa-clone"
        },
        QueryDefinition {
            id: "S07", category: "Compras y Almacén", title: "Inventario Fantasma",
            description: "Materiales creados en el sistema pero no asignados a ningún equipo.",
            cypher: r#"MATCH (m:Material) WHERE NOT ()-[:CONTIENE]->(m)
                       RETURN m.id as CODIGO, m.name as DESCRIPCION, "SIN ASIGNACION" as ESTADO LIMIT 50"#,
            needs_param: false, is_graph: false, icon: "fa-ghost"
        },
        QueryDefinition {
            id: "S08", category: "Compras y Almacén", title: "Valuación Repuestos (Simulado)",
            description: "Listado de materiales asumiendo 'quantity' como factor de valor.",
            cypher: r#"MATCH (m:Material) WHERE toInteger(m.quantity) > 10
                       RETURN m.id, m.name, m.quantity as STOCK_TEORICO ORDER BY toInteger(m.quantity) DESC LIMIT 20"#,
            needs_param: false, is_graph: false, icon: "fa-coins"
        },
        QueryDefinition {
            id: "S09", category: "Compras y Almacén", title: "Vendor Similarities",
            description: "Búsqueda de materiales con proveedores similares (por texto en descripción).",
            cypher: r#"MATCH (m:Material) WHERE m.name CONTAINS 'SKF' OR m.name CONTAINS 'FAG'
                       RETURN m.id, m.name as RODAMIENTO, "PROVEEDOR_COMUN" as TIPO LIMIT 50"#,
            needs_param: false, is_graph: false, icon: "fa-truck"
        },
        QueryDefinition {
            id: "S10", category: "Compras y Almacén", title: "Cobertura de Stock",
            description: "Relación entre equipos instalados y cantidad de repuesto.",
            cypher: r#"MATCH (e:Equipo)-[r:CONTIENE]->(m:Material)
                       RETURN m.name as MATERIAL, sum(toInteger(r.quantity)) as REQ_TOTAL, count(e) as PUNTOS_USO LIMIT 20"#,
            needs_param: false, is_graph: false, icon: "fa-umbrella"
        },

        // =========================================================
        // 3. INGENIERÍA Y PROCESOS (10 Consultas)
        // =========================================================
        QueryDefinition {
            id: "E01", category: "Ingeniería", title: "Árbol Jerárquico Completo",
            description: "Visualización de la ruta desde la planta hasta el activo seleccionado.",
            cypher: r#"MATCH (target {id: $p})
                       MATCH path = (root:UbicacionTecnica)-[:CONTIENE*]->(target)
                       UNWIND relationships(path) as r
                       WITH startNode(r) as a, endNode(r) as b, r
                       RETURN a.id as A_ID, a.name as A_LABEL, labels(a)[0] as A_TYPE, b.id as B_ID, b.name as B_LABEL, labels(b)[0] as B_TYPE, type(r) as RELACION"#,
            needs_param: true, is_graph: true, icon: "fa-sitemap"
        },
        QueryDefinition {
            id: "E02", category: "Ingeniería", title: "Densidad de Activos",
            description: "Conteo de activos por ubicación técnica.",
            cypher: r#"MATCH (u:UbicacionTecnica)-[:CONTIENE*]->(e:Equipo)
                       RETURN u.name as PLANTA, count(DISTINCT e) as TOTAL_EQUIPOS ORDER BY TOTAL_EQUIPOS DESC LIMIT 20"#,
            needs_param: false, is_graph: false, icon: "fa-city"
        },
        QueryDefinition {
            id: "E03", category: "Ingeniería", title: "Niveles de Profundidad",
            description: "Detectar cadenas de dependencia inusualmente largas.",
            cypher: r#"MATCH p=(u:UbicacionTecnica)-[:CONTIENE*]->(m:Material)
                       RETURN length(p) as NIVEL, u.name as RAIZ, m.name as HOJA LIMIT 20"#,
            needs_param: false, is_graph: false, icon: "fa-layer-group"
        },
        QueryDefinition {
            id: "E04", category: "Ingeniería", title: "Equipos Huérfanos",
            description: "Equipos que no están conectados a ninguna ubicación técnica (Error Datos).",
            cypher: r#"MATCH (e:Equipo) WHERE NOT (:UbicacionTecnica)-[:CONTIENE*]->(e)
                       RETURN e.id as ID, e.name as NOMBRE, "SIN UBICACION" as ERROR LIMIT 50"#,
            needs_param: false, is_graph: false, icon: "fa-link-slash"
        },
        QueryDefinition {
            id: "E05", category: "Ingeniería", title: "Sistemas Aislados",
            description: "Islas de datos desconectadas del grafo principal.",
            cypher: r#"MATCH (n) WHERE NOT (n)--() 
                       RETURN n.id, n.name, labels(n)[0] as TIPO LIMIT 50"#,
            needs_param: false, is_graph: false, icon: "fa-island-tropical"
        },
        QueryDefinition {
            id: "E06", category: "Ingeniería", title: "Validación de Tags",
            description: "Equipos cuyo ID no sigue el estándar (ej. longitud < 3).",
            cypher: r#"MATCH (e:Equipo) WHERE size(e.id) < 3
                       RETURN e.id as TAG_INVALIDO, e.name as NOMBRE LIMIT 50"#,
            needs_param: false, is_graph: false, icon: "fa-check-double"
        },
        QueryDefinition {
            id: "E07", category: "Ingeniería", title: "Dependencias Circulares",
            description: "Detección de errores lógicos donde A contiene a B y B contiene a A.",
            cypher: r#"MATCH path = (n)-[:CONTIENE*]->(n)
                       RETURN n.id as NODO_CIRCULAR, length(path) as LONGITUD_BUCLE LIMIT 10"#,
            needs_param: false, is_graph: true, icon: "fa-spin fa-circle-notch"
        },
        QueryDefinition {
            id: "E08", category: "Ingeniería", title: "Activos Sin Nombre",
            description: "Nodos que tienen ID pero les falta la propiedad 'name'.",
            cypher: r#"MATCH (n) WHERE n.name IS NULL OR n.name = ''
                       RETURN n.id as ID, labels(n)[0] as TIPO LIMIT 50"#,
            needs_param: false, is_graph: false, icon: "fa-tag"
        },
        QueryDefinition {
            id: "E09", category: "Ingeniería", title: "Capacidad de Planta",
            description: "Conteo total de nodos por tipo en una ubicación raíz.",
            cypher: r#"MATCH (root:UbicacionTecnica {id: $p})-[:CONTIENE*]->(n)
                       RETURN labels(n)[0] as TIPO_ACTIVO, count(n) as CANTIDAD"#,
            needs_param: true, is_graph: false, icon: "fa-chart-pie"
        },
        QueryDefinition {
            id: "E10", category: "Ingeniería", title: "Trazabilidad Bottom-Up",
            description: "Desde un tornillo hasta la planta (Camino inverso).",
            cypher: r#"MATCH (m:Material {id: $p})
                       MATCH path = (root:UbicacionTecnica)-[:CONTIENE*]->(m)
                       RETURN [n in nodes(path) | n.name] as RUTA_COMPLETA"#,
            needs_param: true, is_graph: false, icon: "fa-arrow-up-from-bracket"
        },

        // =========================================================
        // 4. GESTIÓN Y KPI (10 Consultas)
        // =========================================================
        QueryDefinition {
            id: "G01", category: "Gestión", title: "Resumen Global Activos",
            description: "Conteo total de equipos y materiales en base de datos.",
            cypher: r#"MATCH (n) RETURN labels(n)[0] as CATEGORIA, count(n) as TOTAL"#,
            needs_param: false, is_graph: false, icon: "fa-globe"
        },
        QueryDefinition {
            id: "G02", category: "Gestión", title: "Índice de Obsolescencia",
            description: "Porcentaje de materiales marcados como obsoletos.",
            cypher: r#"MATCH (m:Material) 
                       WITH count(m) as total, sum(CASE WHEN m.status_flag='X' THEN 1 ELSE 0 END) as obs
                       RETURN total as TOTAL_MATERIALES, obs as OBSOLETOS, (toFloat(obs)/total)*100 as PORCENTAJE_RIESGO"#,
            needs_param: false, is_graph: false, icon: "fa-percent"
        },
        QueryDefinition {
            id: "G03", category: "Gestión", title: "Completitud de Datos",
            description: "Nodos que tienen todas las propiedades clave rellenas.",
            cypher: r#"MATCH (e:Equipo) 
                       RETURN count(e) as TOTAL, sum(CASE WHEN e.name IS NOT NULL THEN 1 ELSE 0 END) as CON_NOMBRE"#,
            needs_param: false, is_graph: false, icon: "fa-database"
        },
        QueryDefinition {
            id: "G04", category: "Gestión", title: "Top Plantas Complejas",
            description: "Ubicaciones con mayor número de niveles jerárquicos.",
            cypher: r#"MATCH p=(u:UbicacionTecnica)-[:CONTIENE*]->(m:Material)
                       RETURN u.name as PLANTA, avg(length(p)) as PROFUNDIDAD_PROMEDIO ORDER BY PROFUNDIDAD_PROMEDIO DESC LIMIT 5"#,
            needs_param: false, is_graph: false, icon: "fa-layer-group"
        },
        QueryDefinition {
            id: "G05", category: "Gestión", title: "Ratio Repuestos/Equipo",
            description: "Promedio de repuestos por equipo.",
            cypher: r#"MATCH (e:Equipo)
                       OPTIONAL MATCH (e)-[:CONTIENE]->(m:Material)
                       WITH e, count(m) as conteo
                       RETURN avg(conteo) as PROMEDIO_PIEZAS_POR_EQUIPO"#,
            needs_param: false, is_graph: false, icon: "fa-divide"
        },
        QueryDefinition {
            id: "G06", category: "Gestión", title: "Equipos Críticos (Hotspots)",
            description: "Equipos que aparecen en múltiples rutas (Centralidad).",
            cypher: r#"MATCH (e:Equipo) 
                       OPTIONAL MATCH (e)-[:CONTIENE]->(m)
                       RETURN e.name, count(m) as CONEXIONES ORDER BY CONEXIONES DESC LIMIT 10"#,
            needs_param: false, is_graph: false, icon: "fa-bullseye"
        },
        QueryDefinition {
            id: "G07", category: "Gestión", title: "KPI Estandarización",
            description: "Cuántos materiales únicos vs total de relaciones.",
            cypher: r#"MATCH (m:Material) WITH count(DISTINCT m) as unicos MATCH ()-[r:CONTIENE]->(:Material) RETURN unicos as MATERIALES_UNICOS, count(r) as INSTALACIONES_TOTALES"#,
            needs_param: false, is_graph: false, icon: "fa-ruler-combined"
        },
        QueryDefinition {
            id: "G08", category: "Gestión", title: "Auditoría de Relaciones",
            description: "Verificar tipos de relaciones existentes en el grafo.",
            cypher: r#"MATCH ()-[r]->() RETURN type(r) as TIPO_RELACION, count(r) as CANTIDAD"#,
            needs_param: false, is_graph: false, icon: "fa-magnifying-glass"
        },
        QueryDefinition {
            id: "G09", category: "Gestión", title: "Nodos Recientes",
            description: "Últimos nodos creados (si existiera timestamp, simulado por ID).",
            cypher: r#"MATCH (n) RETURN n.id, n.name, labels(n)[0] as TIPO ORDER BY n.id DESC LIMIT 10"#,
            needs_param: false, is_graph: false, icon: "fa-clock"
        },
        QueryDefinition {
            id: "G10", category: "Gestión", title: "Salud del Grafo",
            description: "Métricas generales de conectividad.",
            cypher: r#"MATCH (n) WITH count(n) as nodos MATCH ()-[r]->() RETURN nodos as NODOS, count(r) as ARISTAS, toFloat(count(r))/nodos as DENSIDAD"#,
            needs_param: false, is_graph: false, icon: "fa-heart-pulse"
        },
    ]
}