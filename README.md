# 🦀 Solana Chad MCP

![Demo](./screenshots/demo.png)

> **Use AI + MCP to monitor your SOL wallet, check market indicators. All by just chatting.**

## 🚀 What is this?

This project is a **Model Context Protocol (MCP) Server** that lets any LLM (like Claude, ChatGPT, etc.) interact with your **Solana wallet** and trade SOL on your behalf through simple natural language.

**Imagine saying:**

> "Check how many SOL are currently in the wallet and whether the price has increased. If it has increased by more than 5%, sell 0.25 SOL."

And it just **does it.**

## 🧠 Features

✅ `check_price` – Get real-time SOL price  
✅ `get_balance` – Check SOL balance of any wallet  
✅ Natural language interaction via any LLM that supports **MCP**

## 🏗️ Tech Stack

- 🦀 **Rust** – blazing fast MCP Server
- 🔗 **Solana RPC** – for blockchain interactions
- 💬 **Claude / ChatGPT** – to give AI commands

## 🔧 Usage

Add this into `claude_desktop_config.json`. (This example is for **Windows**)

```json
{
  "mcpServers": {
    "solana": {
      "command": "PATH-TO/sol-chad-mcp/target/release/examples/sol_chad_mcp.exe",
      "args": []
    }
  }
}
```

### 🧪 Example MCP Tool

```json
{
  "name": "get_balance",
  "description": "Get the balance of a Solana wallet",
  "parameters": {
    "address": "xxxxxxxxxxxxx"
  }
}
```

## ⚠️ Disclaimer

> This is for **educational & entertainment** purposes only.  
> Don’t let AI YOLO your life savings 💸
