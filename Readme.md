# 🐺 Dhib

**A Rust-based security auditing engine for infrastructure.**

> Security auditing from the system up.

Dhib is an open-source security auditing project written in **Rust**.
It is designed to inspect systems, collect security evidence, evaluate
security controls, and produce actionable findings.

The project is currently in early development and starts with **Linux
security auditing**, with plans to expand into containers, Kubernetes,
cloud infrastructure, and runtime security.

---

## ✨ Vision

Dhib aims to provide a modular security auditing engine that can answer:

> **What is running? What is configured? Is it secure? What evidence proves it?**

The long-term goal is to build a platform that connects low-level system
inspection with security controls, risk assessment, observability, and
remediation.

```text
Target System
      │
      ▼
┌───────────────┐
│    Collect    │
│    Evidence   │
└───────┬───────┘
        │
        ▼
┌───────────────┐
│ Rules Engine  │
└───────┬───────┘
        │
        ▼
   PASS / FAIL
        │
        ▼
┌───────────────┐
│   Findings    │
│   + Risk      │
└───────┬───────┘
        │
        ├──────► Terminal
        │
        └──────► JSON