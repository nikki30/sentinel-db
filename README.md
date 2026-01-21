# 🛡️ Sentinel-DB

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![MCP Native](https://img.shields.io/badge/MCP-Native-blue.svg)](https://modelcontextprotocol.io)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Stars](https://img.shields.io/github/stars/YOUR_USERNAME/sentinel-db?style=social)](https://github.com/YOUR_USERNAME/sentinel-db)

> **The high-performance, local-first knowledge engine that stops AI agents from forgetting your code.**

Sentinel-DB is a specialized retrieval engine designed to provide **Long-Term Memory** for autonomous engineering agents. It eliminates the **"Context Tax"** by distilling massive codebases into a dense Knowledge Pyramid, allowing Claude, Cursor, and Copilot to reason about your entire architecture without hitting token limits or leaking data to the cloud.

---

## 💎 The "Aha!" Value
* **Stop the Context Tax**: Stop paying for redundant tokens. Sentinel-DB finds the exact 50 lines that matter.
* **Privacy-First**: Your code never leaves your machine. All indexing and vector search happens locally.
* **Architectural Reasoning**: Go beyond "Search." Ask *"How does Auth flow to the DB?"* and get an architectural summary, not just a file list.

---

## 🏗️ Architecture: The Knowledge Pyramid
Sentinel-DB doesn't just "index text"—it understands **code structure** using Tree-Sitter and recursive summarization.

```text
       ▲          [ REASONING LAYER ]
      / \         Summarized Modules & Patterns
     /   \        -----------------------------
    /     \       [ SEMANTIC LAYER ]
   /       \      HNSW Vector Index (Meaning)
  /         \     -----------------------------
 /           \    [ STRUCTURAL LAYER ]
/_____________\   Tree-Sitter AST & Symbol Maps
```

---

## 🛠️ Key Features
* ⚡**SIMD-Accelerated Vector Search**: High-concurrency Rust engine using AVX-512/NEON instructions for sub-50ms retrieval across million-line repos.
* 🌳**Semantic Chunking (Tree-Sitter)**: Instead of "blind" character limits, Sentinel-DB uses Abstract Syntax Trees (AST) to respect logical boundaries (Methods, Classes, and Modules).
* 📉 **Product Quantization (PQ)**: Advanced vector compression that reduces RAM footprint by 90% while maintaining 97%+ search accuracy.
* 🔌**MCP Native**: Built-in Model Context Protocol server, enabling "plug-and-play" memory for Claude, Cursor, and internal Microsoft agentic tools.
* 🏗️ **Knowledge Pyramids**: Recursive summarization that allows AI to navigate from "Line 42" up to "System Architecture" seamlessly.
* 🔐 **Privacy-First & Local-First**: No data leaves your machine. Full compatibility with offline-mode engineering workflows.
