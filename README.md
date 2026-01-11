# Pressure-Field Coordination for Multi-Agent LLM Systems

This repository contains the code and experiments for the paper:

> **Emergent Coordination in Multi-Agent Systems via Pressure Fields and Temporal Decay**
> Roland R. Rodriguez, Jr.
> January 2026

📄 [Read the paper](paper/rodriguez-pressure-field-coordination-2026.pdf)

## Key Finding

Implicit coordination through shared pressure gradients matches explicit hierarchical control (38.2% vs 38.8% solve rate, p=0.94) while dramatically outperforming dialogue-based multi-agent coordination (8.6%). Temporal decay is essential—disabling it increases final pressure 49-fold.

## What's Here

```
crates/
├── survival-kernel/    # Core pressure-field coordination framework
└── latin-experiment/   # Latin Square experiments and baselines
paper/
└── main.typ           # Paper source (Typst)
```

## Quick Start

```bash
# Build
cargo build

# Run tests
cargo nextest run

# Run experiments (requires vLLM with Qwen2.5 models)
cargo run -p latin-experiment -- --help
```

## The Idea

Traditional multi-agent LLM frameworks use explicit orchestration: planners, managers, message-passing. We take a different approach inspired by stigmergy (ant colonies, immune systems):

1. **Shared artifact** — agents modify a common workspace
2. **Pressure gradients** — local quality signals guide action
3. **Temporal decay** — prevents premature convergence
4. **No communication** — coordination emerges from shared state

## Experiments

We evaluate on Latin Square constraint satisfaction (1,078 trials):

| Strategy | Solve Rate |
|----------|------------|
| Pressure-field | 38.2% |
| Hierarchical | 38.8% |
| Sequential | 23.3% |
| Random | 11.7% |
| Conversation (AutoGen-style) | 8.6% |

Pressure-field matches hierarchical with simpler architecture. Conversation-based dialogue performs worst—even below random.

## Requirements

- Rust 1.75+ (edition 2024)
- [vLLM](https://github.com/vllm-project/vllm) with Qwen2.5 models
- [Typst](https://typst.app/) (for paper compilation)

## License

MIT OR Apache-2.0
