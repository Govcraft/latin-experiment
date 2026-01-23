# Pressure-Field Coordination for Multi-Agent LLM Systems

This repository contains the code and experiments for the paper:

> **Emergent Coordination in Multi-Agent Systems via Pressure Fields and Temporal Decay**
> Roland R. Rodriguez, Jr.
> January 2026

## Key Finding

Pressure-field coordination substantially outperforms all baselines on meeting room scheduling: **48.5%** solve rate vs conversation-based dialogue (12.6%), hierarchical control (1.5%), and sequential/random baselines (<1%). Temporal decay is essential—disabling it reduces solve rate by 10 percentage points.

## What's Here

```
crates/
├── survival-kernel/      # Core pressure-field coordination framework
└── schedule-experiment/  # Meeting room scheduling experiments and baselines
paper/
└── rodriguez-pressure-field-coordination-2026.tex  # Paper source (LaTeX)
results/
├── *.json                # Experiment result data
├── generate_figures.R    # Figure generation scripts
└── analyze_*.R           # Analysis scripts
```

## Quick Start

```bash
# Build
cargo build

# Run tests
cargo nextest run

# Run experiments (requires Ollama with Qwen2.5 models)
OLLAMA_HOST=http://localhost:11434 cargo run -p schedule-experiment -- --help

# Quick test run (3 ticks, reduced trials)
QUICK=1 OLLAMA_HOST=http://localhost:11434 cargo run -p schedule-experiment

# Compile paper
cd paper && make final
```

## The Idea

Traditional multi-agent LLM frameworks use explicit orchestration: planners, managers, message-passing. We take a different approach inspired by stigmergy (ant colonies, immune systems):

1. **Shared artifact** — agents modify a common workspace
2. **Local pressure** — quality signals guide greedy action
3. **Temporal decay** — prevents premature convergence
4. **No communication** — coordination emerges from shared state

## Experiments

We evaluate on meeting room scheduling (270 trials across easy/medium/hard problems):

| Strategy | Solve Rate |
|----------|------------|
| Pressure-field | **48.5%** |
| Conversation (AutoGen-style) | 12.6% |
| Hierarchical | 1.5% |
| Sequential | 0.4% |
| Random | 0.4% |

Pressure-field achieves nearly 4× the solve rate of the next-best baseline. Effect size is large (Cohen's h = 1.07 vs conversation).

## Requirements

- Rust 1.75+ (edition 2024)
- [Ollama](https://ollama.ai/) with Qwen2.5 models (0.5b, 1.5b, 3b)
- LaTeX distribution (for paper compilation)

## License

MIT OR Apache-2.0
