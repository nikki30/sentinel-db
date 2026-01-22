use anyhow;
use tree_sitter::{Parser, Node};
use walkdir::WalkDir;
use std::fs;
use fastembed::{TextEmbedding, EmbeddingModel, InitOptions};

fn main() -> anyhow::Result<()> {
    // 1. Initialize the Brain
    let model = TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::AllMiniLML6V2) // 👈 Removed the ", true"
            .with_show_download_progress(true)           // 👈 Add this if you want the progress bar
    )?;
    
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into())?;

    // 2. A place to store what we find
    let mut all_snippets = Vec::new();

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
            
            // 💎 Pass the snippet list to the crawler
            crawl_node(tree.root_node(), &code, &mut all_snippets);
        }
    }

    // 3. GENERATE ALL EMBEDDINGS AT ONCE
    if !all_snippets.is_empty() {
        println!("\n🧠 Generating embeddings for {} snippets...", all_snippets.len());
        let embeddings = model.embed(all_snippets.clone(), None)?;
        
        println!("✅ Success! First vector size: {}", embeddings[0].len());
    }

    Ok(())
}

// Update your crawl_node to take the &mut Vec<String>
fn crawl_node(node: Node, source: &str, snippets: &mut Vec<String>) {
    if node.kind() == "function_item" {
        let body = &source[node.start_byte()..node.end_byte()];
        snippets.push(body.to_string());
        println!("  🚀 Captured: {}", node.child_by_field_name("name").unwrap().utf8_text(source.as_bytes()).unwrap());
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        crawl_node(child, source, snippets);
    }
}