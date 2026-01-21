use tree_sitter::{Parser, Node};
use walkdir::WalkDir;
use std::fs;

fn main() -> anyhow::Result<()> {
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_rust::LANGUAGE.into())?;

    println!("🔎 Sentinel Repo Scan Initiated...");

    // 1. Walk through every file in your folder
    for entry in WalkDir::new(".")
        .into_iter()
        .filter_entry(|e| !e.file_name().to_str().map(|s| s == "target" || s == ".git").unwrap_or(false))
        .filter_map(|e| e.ok()) 
    {
        let path = entry.path();
        
        // 2. Only analyze Rust source files
        if path.extension().map_or(false, |ext| ext == "rs") {
            let code = fs::read_to_string(path)?;
            let tree = parser.parse(&code, None).unwrap();
            
            println!("\n📄 File: {}", path.display());
            crawl_node(tree.root_node(), &code);
        }
    }

    Ok(())
}

fn crawl_node(node: Node, source: &str) {
    if node.kind() == "function_item" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = &source[name_node.start_byte()..name_node.end_byte()];
            println!("  🚀 [Line {}] Function: {}()", node.start_position().row + 1, name);
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        crawl_node(child, source);
    }
}