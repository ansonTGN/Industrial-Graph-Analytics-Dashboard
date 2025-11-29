use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct QueryDefinition {
    // Usamos String en lugar de &'static str porque los datos se cargarán en tiempo de ejecución
    pub id: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub cypher: String,
    pub needs_param: bool,
    pub is_graph: bool,
    pub icon: String,
}

pub fn load_queries_from_file() -> Vec<QueryDefinition> {
    let file = match File::open("queries.json") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("❌ Error crítico: No se pudo abrir 'queries.json': {}", e);
            return Vec::new();
        }
    };
    
    let reader = BufReader::new(file);
    let queries: Vec<QueryDefinition> = match serde_json::from_reader(reader) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("❌ Error parseando 'queries.json': {}", e);
            return Vec::new();
        }
    };
    
    println!("✅ Cargadas {} consultas desde queries.json", queries.len());
    queries
}