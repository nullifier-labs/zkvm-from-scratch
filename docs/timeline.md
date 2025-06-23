# zkVM Project Timeline

> **Assumption** – small, focused team of 3–4 engineers working full-time.  Durations are optimistic but achievable with prior zk experience.

| Phase | Duration | Start | End | Dependencies |
| --- | --- | --- | --- | --- |
| 0 – Research & Setup | 2 weeks | T0 | T0 + 2w | – |
| 1 – RISC-V Execution Model | 4 weeks | T0 + 2w | T0 + 6w | 0 |
| 2 – Trace & Constraint Design | 6 weeks | T0 + 6w | T0 + 12w | 1 |
| 3 – Prover Implementation | 8 weeks | T0 + 12w | T0 + 20w | 2 |
| 4 – Verifier Implementation | 4 weeks | T0 + 18w | T0 + 22w | 3 (parallel from week 18) |
| 5 – Recursion & Aggregation | 4 weeks | T0 + 22w | T0 + 26w | 3,4 |
| 6 – Integration & API | 3 weeks | T0 + 26w | T0 + 29w | 3,4,5 |
| 7 – Audit & Optimisation | 4 weeks | T0 + 29w | T0 + 33w | 6 |
| 8 – Documentation & Release | 3 weeks | T0 + 33w | T0 + 36w | 6,7 |

## Gantt-style View (text)

```
T0 --------------------------------------------------------------------------- T0 + 36 weeks
Phase 0  |==|
Phase 1      |====|
Phase 2            |======|
Phase 3                    |========|
Phase 4                              |====|
Phase 5                                    |====|
Phase 6                                          |===|
Phase 7                                               |====|
Phase 8                                                    |===|
```

## Milestones

1. **M1 – Minimal Interpreter (RV32I) passes test-suite** (end of Phase 1)
2. **M2 – First end-to-end proof of simple program** (end of Phase 3)
3. **M3 – On-chain verifier deployed on testnet** (mid-Phase 4)
4. **M4 – Aggregated proof ≤1 MB for 10k-step program** (end of Phase 5)
5. **M5 – v1.0.0 Public Release** (end of Phase 8)

## Buffer & Contingency

Allocate ~15 % calendar slack (≈5 weeks) for unforeseen complexity or personnel changes. 