---
name: pdfce-integration-and-the-pending-publish-decision
description: pdfce/pdfcer integrates icccer from crates.io pins; publishing is standing-authorised by Ken since 2026-11-13
metadata:
  type: project
---

**As of 2026-09-01 `pdfce` has stopped asking whether `icccer` can do the job
and started asking how to depend on it.** It diagnosed a real rendering
defect (a print-conformance patch failing 15 of 16 cells), attributed it to
its own round-trip sRGB→CMYK transform, and named `icccer` as the
replacement. It offered three dependency shapes — pinned git `rev`,
crates.io release, or vendoring — and stated its own bias for the pinned
`rev`.

**Why:** `pdfce` will not grow a CMM (its `ARCHITECTURE.md` decision 064
puts conversion on icccer's side of the boundary permanently), so this is the
first real consumer arriving at the seam rather than a hypothetical one.

**How to apply:**

- The recommendation given back was **pinned `rev`**, and the reason is
  worth keeping: consumer questions keep changing this library's API surface
  (`Chain::bpc_disclosure` exists *because* pdfce asked a question on
  2026-09-01), and a `rev` makes the next change cheap.
- **RESOLVED 2026-11-13: Ken — "Always publish."** crates.io publishing is
  standing-authorised (0.4.2 published 2026-11-07, 0.4.3 on 2026-11-13; pdfcer
  pins `"=0.4.x"`). The bullet below is history.
- (historical) **A crates.io release is an OPEN operator decision.** It was put in
  front of Ken on 2026-09-01 and he has not ruled. The manifests are
  release-shaped (`0.2.0`, `version` on path deps) so it is a decision, not
  a work item. **Do not treat readiness as permission** — rule 9 stands, and
  no go-ahead has been given. Check with Ken before assuming this resolved.
- pdfce's hard gates (wasm32 CI, no copyleft, no network client in
  `pdfce-core`/`pdfce-render`) constrain anything added here. See
  [[suite-compatibility]] and `CLAUDE.md` rule 10 item 2.

Related: [[run-the-conformance-demonstration-not-just-the-diagnosis]] — the
2026-09-01 exchange is the pattern working: every one of pdfce's four asks
was answered with a measurement rather than a code reading.

**Update 2026-10-22:** the consumer now signs itself **`pdfcer`** and pins
`a4d9003b`. It asked for crates.io *readiness*, not publication, because
`pdfcer-render` can't publish while it has git deps. Readiness is done
(`DL-083`, `v0.3.1`). **The publish question is still Ken's and still open.**
