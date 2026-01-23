use anyhow;
use tree_sitter::{Parser, Node};
use walkdir::WalkDir;
use std::fs;
use std::env;
use fastembed::{TextEmbedding, EmbeddingModel, InitOptions};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct KnowledgePoint {
    file_path: String,
    function_name: String,
    code_body: String,
    embedding: Vec<f32>,
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage:\n  cargo run -p sentinel-mcp -- index\n  cargo run -p sentinel-mcp -- search \"your query\"");
        return Ok(());
    }

    // 1. Initialize the Brain
    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true)
    )?;

    match args[1].as_str() {
        "index" => run_indexing(&model)?,
        "search" => {
            if args.len() < 3 {
                println!("Please provide a search query.");
                return Ok(());
            }
            let query = &args[2];
            let map_data = fs::read_to_string("sentinel_map.json")?;
            let map: Vec<KnowledgePoint> = serde_json::from_str(&map_data)?;
            search(query, &map, &model)?;
        }
        _ => println!("Unknown command. Use 'index' or 'search'."),
    }

    Ok(())
}

fn run_indexing(model: &TextEmbedding) -> anyhow::Result<()> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into())?;
    let mut all_snippets = Vec::new();
    let mut metadata = Vec::new(); // Store names and paths

    println!("🔎 Sentinel Repo Scan Initiated...");

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| !e.file_name().to_str().map(|s| s == "target" || s == ".git").unwrap_or(false))
        .filter_map(|e| e.ok()) 
    {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "rs") {
            let code = fs::read_to_string(path)?;
            let tree = parser.parse(&code, None).unwrap();
            
            // Modified crawl_node to return details
            crawl_and_collect(tree.root_node(), &code, path.to_str().unwrap(), &mut all_snippets, &mut metadata);
        }
    }

    if !all_snippets.is_empty() {
        println!("\n🧠 Generating embeddings for {} snippets...", all_snippets.len());
        let embeddings = model.embed(all_snippets.clone(), None)?;
        
        let final_map: Vec<KnowledgePoint> = metadata.into_iter().zip(embeddings).map(|(meta, emb)| {
            KnowledgePoint {
                file_path: meta.0,
                function_name: meta.1,
                code_body: meta.2,
                embedding: emb,
            }
        }).collect();

        let json = serde_json::to_string_pretty(&final_map)?;
        fs::write("sentinel_map.json", json)?;
        println!("✅ Success! Knowledge map saved to sentinel_map.json");
    }
    Ok(())
}

fn crawl_and_collect(node: Node, source: &str, path: &str, snippets: &mut Vec<String>, metadata: &mut Vec<(String, String, String)>) {
    if node.kind() == "function_item" {
        let name_node = node.child_by_field_name("name").unwrap();
        let name = name_node.utf8_text(source.as_bytes()).unwrap().to_string();
        let body = source[node.start_byte()..node.end_byte()].to_string();
        
        snippets.push(body.clone());
        metadata.push((path.to_string(), name.clone(), body)); 
        println!("  🚀 Captured: {}()", name);
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        crawl_and_collect(child, source, path, snippets, metadata);
    }
}

fn search(query: &str, map: &[KnowledgePoint], model: &TextEmbedding) -> anyhow::Result<()> {
    let query_vector = model.embed(vec![query.to_string()], None)?[0].clone();

    let mut results: Vec<(&KnowledgePoint, f32)> = map.iter()
        .map(|point| {
            let score = point.embedding.iter().zip(&query_vector).map(|(a, b)| a * b).sum();
            (point, score)
        })
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\n🎯 Top Matches for: '{}'", query);

    for (point, score) in results.iter().take(3) {
        if *score > 0.35 { // Only show confident matches
            println!("[Score: {:.4}] Function: {}()", score, point.function_name);
            println!("   Location: {}", point.file_path);
            println!("---");
        }
    }
   
    Ok(())
}