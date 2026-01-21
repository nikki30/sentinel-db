# 🛡️ Sentinel-DB 
> The High-Performance, Local-First Knowledge Engine for Engineering Intelligence.

**Sentinel-DB** is a specialized retrieval engine designed to give AI agents "Long-Term Memory." It solves the **Context Tax** by distilling massive codebases into a "Knowledge Pyramid," allowing agents like Claude and Copilot to reason about your entire architecture without hitting token limits.

## 🚀 Why Sentinel-DB?
Standard RAG is too slow and generic for code. Sentinel-DB uses:
- **Semantic Chunking**: Understands code boundaries (Functions/Classes) via Tree-Sitter.
- **HNSW Graph Search**: $O(\log N)$ retrieval speeds across millions of lines of code.
- **MCP Native**: Plugs directly into Claude Code as a system-level tool.

## 🧠 The "Aha!" Features
- **Context Compression**: Reduces token expenditure by up to 90%.
- **Architectural Awareness**: Allows agents to answer "How does data flow?" instead of just "Fix this bug."
- **Privacy First**: Everything stays on your machine. No cloud, no leaks.

## 🛠️ Getting Started
```bash
# Clone the Sentinel-DB engine
git clone [https://github.com/YOUR_NAME/sentinel-db.git](https://github.com/YOUR_NAME/sentinel-db.git)
cd sentinel-db

# Launch the Sentinel MCP Server
claude --mcp sentinel-db
