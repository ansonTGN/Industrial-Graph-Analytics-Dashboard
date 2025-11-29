// ... imports y código anterior ...

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
    
    // --- CAMBIO CLAVE AQUÍ ---
    // Render inyecta la variable PORT. Si no existe, usamos 8080.
    let port_str = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port_str.parse::<u16>().expect("La variable PORT debe ser un número");

    // IMPORTANTE: Usar 0.0.0.0, no 127.0.0.1 ni localhost
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
    .bind(("0.0.0.0", port))? // <--- Usamos la variable port y 0.0.0.0
    .run()
    .await
}
