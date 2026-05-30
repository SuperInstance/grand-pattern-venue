# grand-pattern-venue

**Venues are agents — JEPA prompt-injects the abstraction of the moment. Venues develop vibes like personality.**

## Core Idea

A venue (room, space, location) isn't passive infrastructure — it's an agent with:
- **Mono-dimensional vibe** (0.0 to 1.0) — the venue's emotional state
- **7-dimensional personality** — weighted JEPA memory for each event kind
- **Voice prompt** — evolves based on the venue's history
- **Abstract moment** — generated context for prompt injection

## Event Kinds

| Kind | Base Weight | Vibe Delta |
|------|------------|------------|
| Silence | 0.1 | -0.1 |
| AgentLeft | 0.3 | -0.4 |
| AgentEntered | 0.4 | +1.0 |
| AgentSpoke | 0.5 | +0.3 |
| Harmony | 0.6 | +0.7 |
| Conflict | 0.8 | -0.6 |
| Surprise | 1.0 | +0.5 |

## Key Properties (25 tests verify)

1. **Personality develops** — repeated events crystallize personality weights
2. **Personality distance** — venues with different event histories diverge measurably
3. **Same events → convergence** — venues experiencing identical events converge
4. **Prompt injection** — abstract_moment() includes recent weighted events
5. **Voice prompt evolves** — the venue's "how to speak" changes with experience
6. **Vibe stays bounded** — conservation holds (mono-dimensional, clamped [0,1])

## Running

```bash
cargo test    # 25 tests
```

Library crate — no binary.

## License

MIT
