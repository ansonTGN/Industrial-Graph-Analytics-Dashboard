use actix_web::{web, App, HttpServer, HttpResponse, Responder, http::header};
use neo4rs::*;
use serde::{Deserialize, Serialize};
use tera::{Tera, Context};
use tokio::sync::Mutex;
use std::collections::{HashMap, BTreeMap};
use dotenv::dotenv;
use std::env; // <--- Necesario para leer el PORT

// Importamos el módulo de consultas
mod queries;
use queries::{load_queries_from_file, QueryDefinition};

// ==========================================
// 1. ESTRUCTURAS
// ==========================================

struct AppState {
    graph: Mutex<Option<Graph>>,
    db_host: Mutex<String>, 
    tera: Tera,
    queries: Vec<QueryDefinition>,
}

#[derive(Deserialize)]
struct LoginParams {
    uri: String,
    user: String,
    pass: String,
}

#[derive(Deserialize)]
struct QueryParams {
    query_id: String,
    param: Option<String>,
}

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
}

#[derive(Serialize)]
struct SearchResult {
    id: String,
    title: String,
    label: String,
}

#[derive(Serialize)]
struct QueryResult {
    columns: Vec<String>,
    rows: Vec<HashMap<String, String>>,
    query_title: String, 
    timestamp: String,
    is_graph: bool,
}

// ==========================================
// 2. HELPERS
// ==========================================

async fn get_node_name(graph: &Graph, id: &str) -> String {
    let q_str = "MATCH (n) WHERE n.id = $id RETURN n.id + ' - ' + n.name as label LIMIT 1";
    let q = query(q_str).param("id", id);
    if let Ok(mut stream) = graph.execute(q).await {
        if let Ok(Some(row)) = stream.next().await {
            return row.get::<String>("label").unwrap_or(id.to_string());
        }
    }
    id.to_string()
}

// ==========================================
// 3. HANDLERS
// ==========================================

async fn index(data: web::Data<AppState>) -> impl Responder {
    println!("📥 Petición recibida en Login"); 
    let mut ctx = Context::new();
    // Leemos variables de entorno, con fallback para local
    ctx.insert("env_uri", &env::var("NEO4J_URI").unwrap_or_else(|_| "bolt://localhost:7687".to_string()));
    ctx.insert("env_user", &env::var("NEO4J_USERNAME").unwrap_or_else(|_| "neo4j".to_string()));
    ctx.insert("env_pass", &env::var("NEO4J_PASSWORD").unwrap_or_default());
    
    match data.tera.render("login.html", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => {
            println!("❌ Error renderizando login.html: {}", e);
            HttpResponse::InternalServerError().body(format!("Error en plantilla: {}", e))
        }
    }
}

async fn connect_db(data: web::Data<AppState>, form: web::Form<LoginParams>) -> impl Responder {
    println!("🔌 Conectando a: {}", form.uri);
    let config = ConfigBuilder::default()
        .uri(form.uri.trim())
        .user(&form.user)
        .password(&form.pass)
        .max_connections(10)
        .build()
        .unwrap();

    match Graph::connect(config).await {
        Ok(graph) => {
            let mut g = data.graph.lock().await;
            *g = Some(graph);
            let mut host_store = data.db_host.lock().await;
            *host_store = form.uri.clone();
            println!("✅ Conexión establecida");
            HttpResponse::SeeOther().append_header((header::LOCATION, "/dashboard")).finish()
        },
        Err(e) => {
            println!("❌ Fallo de conexión: {}", e);
            let mut ctx = Context::new();
            ctx.insert("error", &format!("Error de conexión: {}", e));
            ctx.insert("env_uri", &form.uri);
            ctx.insert("env_user", &form.user);
            ctx.insert("env_pass", &form.pass);
            HttpResponse::Ok().body(data.tera.render("login.html", &ctx).unwrap())
        }
    }
}

async fn search_nodes(data: web::Data<AppState>, info: web::Query<SearchParams>) -> impl Responder {
    let g_guard = data.graph.lock().await;
    let graph = match g_guard.as_ref() {
        Some(g) => g,
        None => return HttpResponse::Unauthorized().json("No DB connection"),
    };

    let search_term = info.q.clone().unwrap_or_default();
    let (cypher_q, param_q) = if search_term.trim().is_empty() {
        (r#"MATCH (n) WHERE n:Material OR n:Equipo OR n:UbicacionTecnica
            RETURN n.id as id, n.name as name, labels(n)[0] as label LIMIT 20"#, "".to_string())
    } else {
        (r#"MATCH (n) WHERE (n:Material OR n:Equipo OR n:UbicacionTecnica) AND 
            (toLower(n.id) CONTAINS toLower($q) OR toLower(n.name) CONTAINS toLower($q))
            RETURN n.id as id, n.name as name, labels(n)[0] as label LIMIT 20"#, search_term)
    };

    let q = query(cypher_q).param("q", param_q);
    let mut results = Vec::new();

    if let Ok(mut stream) = graph.execute(q).await {
        while let Ok(Some(row)) = stream.next().await {
            results.push(SearchResult {
                id: row.get("id").unwrap_or_default(),
                title: format!("{} - {}", row.get::<String>("id").unwrap_or_default(), row.get::<String>("name").unwrap_or_default()),
                label: row.get("label").unwrap_or_default(),
            });
        }
    }
    HttpResponse::Ok().json(results)
}

async fn dashboard(data: web::Data<AppState>) -> impl Responder {
    let g = data.graph.lock().await;
    if g.is_none() {
        return HttpResponse::SeeOther().append_header((header::LOCATION, "/")).finish();
    }
    
    let mut ctx = Context::new();
    let host_info = data.db_host.lock().await;
    ctx.insert("db_host", &*host_info);

    ctx.insert("current_query", "");
    ctx.insert("current_param", "");
    ctx.insert("current_param_label", "");

    let mut grouped: BTreeMap<String, Vec<QueryDefinition>> = BTreeMap::new();
    for q in &data.queries {
        grouped.entry(q.category.to_string()).or_insert(Vec::new()).push(q.clone());
    }
    ctx.insert("categorized_queries", &grouped);
    
    match data.tera.render("dashboard.html", &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => {
            println!("❌ Error dashboard: {}", e);
            HttpResponse::InternalServerError().body(format!("Error: {}", e))
        }
    }
}

async fn execute_query(data: web::Data<AppState>, form: web::Form<QueryParams>) -> impl Responder {
    let g_guard = data.graph.lock().await;
    let graph = match g_guard.as_ref() {
        Some(g) => g,
        None => return HttpResponse::Unauthorized().body("DB desconectada"),
    };

    let query_def = match data.queries.iter().find(|q| q.id == form.query_id) {
        Some(q) => q.clone(), 
        None => return HttpResponse::BadRequest().body("Consulta no encontrada"),
    };

    let mut current_param_label = String::new();
    let current_param_val = form.param.as_deref().unwrap_or("").to_string();
    let mut query_obj = query(&query_def.cypher);

    let mut ctx = Context::new();
    let host_info = data.db_host.lock().await;
    ctx.insert("db_host", &*host_info);
    
    let mut grouped: BTreeMap<String, Vec<QueryDefinition>> = BTreeMap::new();
    for q in &data.queries {
        grouped.entry(q.category.to_string()).or_insert(Vec::new()).push(q.clone());
    }
    ctx.insert("categorized_queries", &grouped);
    ctx.insert("current_query", &form.query_id);
    
    if query_def.needs_param {
        if !current_param_val.trim().is_empty() {
            query_obj = query_obj.param("p", current_param_val.clone());
            current_param_label = get_node_name(graph, &current_param_val).await;
        } else {
            ctx.insert("error", "⚠️ Esta consulta requiere seleccionar un objeto en el filtro.");
            ctx.insert("current_param", ""); 
            return HttpResponse::Ok().body(data.tera.render("dashboard.html", &ctx).unwrap());
        }
    }

    match graph.execute(query_obj).await {
        Ok(mut stream) => {
            let mut rows_vec = Vec::new();
            let mut columns: Vec<String> = Vec::new();

            while let Ok(Some(row)) = stream.next().await {
                let map: HashMap<String, serde_json::Value> = row.to().unwrap_or_default();
                if columns.is_empty() {
                    let mut keys: Vec<String> = map.keys().cloned().collect();
                    keys.sort_by(|a, b| {
                        let priority_a = if a.contains("ID") { 0 } else if a.contains("LABEL") || a.contains("NOMBRE") { 1 } else { 2 };
                        let priority_b = if b.contains("ID") { 0 } else if b.contains("LABEL") || b.contains("NOMBRE") { 1 } else { 2 };
                        if priority_a != priority_b { priority_a.cmp(&priority_b) } else { a.cmp(b) }
                    });
                    columns = keys;
                }
                
                let mut row_display = HashMap::new();
                for key in &columns {
                    let val_str = match map.get(key) {
                        Some(val) => match val {
                            serde_json::Value::String(s) => s.clone(),
                            serde_json::Value::Number(n) => n.to_string(),
                            serde_json::Value::Array(_) => format!("Lista [{}]", val.as_array().unwrap().len()),
                            serde_json::Value::Bool(b) => b.to_string(),
                            serde_json::Value::Null => "-".to_string(),
                            _ => val.to_string(),
                        },
                        None => "-".to_string(),
                    };
                    row_display.insert(key.clone(), val_str);
                }
                rows_vec.push(row_display);
            }

            let result_data = QueryResult {
                columns,
                rows: rows_vec,
                query_title: query_def.title.to_string(),
                timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                is_graph: query_def.is_graph
            };

            ctx.insert("results", &result_data);
            ctx.insert("current_param", &current_param_val);
            ctx.insert("current_param_label", &current_param_label);
            
            HttpResponse::Ok().body(data.tera.render("dashboard.html", &ctx).unwrap())
        },
        Err(e) => {
            ctx.insert("error", &format!("Error ejecutando Cypher: {}", e));
            HttpResponse::Ok().body(data.tera.render("dashboard.html", &ctx).unwrap())
        }
    }
}

// ==========================================
// 4. MAIN (Aquí está el cambio del Puerto)
// ==========================================

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 
    let tera = Tera::new("templates/**/*").expect("Error cargando templates");
    let loaded_queries = load_queries_from_file();

    let app_state = web::Data::new(AppState {
        graph: Mutex::new(None),
        db_host: Mutex::new(String::new()),
        tera,
        queries: loaded_queries,
    });
    
    // --- LÓGICA DE PUERTO PARA RENDER ---
    // Si la variable PORT existe (Nube), úsala. Si no (Local), usa 8081.
    // Importante: Debe ser bind a "0.0.0.0"
    let port_str = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let port = port_str.parse::<u16>().expect("PORT debe ser un número válido");

    println!("🚀 SERVIDOR INICIADO EN: 0.0.0.0:{}", port);
    
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/connect", web::post().to(connect_db))
            .route("/dashboard", web::get().to(dashboard))
            .route("/query", web::post().to(execute_query))
            .route("/api/search", web::get().to(search_nodes))
    })
    .bind(("0.0.0.0", port))? 
    .run()
    .await
}