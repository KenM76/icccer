---
name: gate-catches-only-enumerated-failures
description: 2026-08-25 — our wasm32 CI job cannot catch a threading regression (std::thread and rayon check cleanly for wasm32); the job's own long comment enumerates what it proves and omits this. Remedy is standing invariant 7, not a better gate. Plus: a byte-grep over PDFs measures zero and is believed.
metadata:
  type: project
---

**A gate is worth only the failure modes somebody thought to enumerate,
and its comment block is where a reader forms the wrong belief.**

`pdfce` found on 2026-08-21, and volunteered on 2026-08-25, that
**`std::thread` and `rayon` both `cargo check` cleanly for
`wasm32-unknown-unknown`** — the failure lands **at runtime in a web
fork**, where no compile gate can see it. `[REPORTED]`; measured on their
tree.

**Our `.github/workflows/ci.yml` job `wasm32-unknown-unknown (library
crates)` has the same hole.** Its comment block explains at length *why*
it exists and enumerates carefully what it does and does not prove —
*"this converts the observation into a gate"* — and **the threading hole
is not among them.** Filed at `ARCHITECTURE.md` **§3.1**, with a new
**standing invariant 7: no thread is spawned to build or evaluate a
transform.**

**Why the invariant and not a better gate:** a prohibition can be applied
by a reviewer reading a diff; a gate protects only what its author
enumerated.

**How to apply:**

- ★★★ **Do not round the evidence up.** They measured **`cargo check`**;
  our job runs **`cargo build`** *(verified)*. Whether `build` differs for
  this hazard **has not been measured by anybody here.** The honest form
  is *"not known to catch it, with positive evidence from a sibling about
  the adjacent command"* — **not** *"measured not to catch it."*
  Measuring it is one command and is recorded as owed.
- **Adding a dependency here can break a consumer's CI in their
  repository** (`DL-044`), so the gate matters — this finding narrows what
  it buys, it does not retire it.
- ★ **Sibling hazard from the same exchange, generalises beyond colour:**
  **a byte-grep for `/RI` over PDFs returns zero and is believed.** On
  `pdfce`'s print corpus the raw pass found `0` where the deep pass found
  `12` — every instance sits inside a Flate-compressed object. **Inflate
  first.** Both hazards are owed an escalation to `D:\dev\rag\rust\` and
  `C:\personal_rag\pdf\`; **not done as of 2026-08-25.**
- Related: [[iccce-documented-is-not-tested]] (`DL-051`) — the same shape,
  where a well-explained thing reads as protected; and
  [[iccce-unfailable-row-protected-only-by-siblings]] (`DL-066`) — a
  one-sided check is blind to half the defects.
