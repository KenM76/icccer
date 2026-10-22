---
name: refusal-at-one-layer-is-nonconformance-at-another
description: DL-072 (2026-08-25) — icccer's CLI refuses an unknown intent name by design; a library API consumed by a PDF reader MUST substitute and disclose. Both right, different layers; the decision is that neither may be changed by citing the other, and the dangerous commit is the tidying one.
metadata:
  type: project
---

**A behaviour that is correct at the CLI can be NON-CONFORMANT in the
library, and the two are not evidence about each other.**

`icccer transform --intent <name>` refuses an unrecognised name with exit
`2`, with a doc comment giving project rule 1 as the reason: *"a
substituted intent produces plausible wrong colour."* `pdfce` requires the
opposite of a library API, citing ISO 32000-2 **§8.6.5.8**, which permits
names outside the standard four and **obliges a reader to substitute a
default**. Filed as **`ARCHITECTURE.md` DL-072**.

| layer | who typed the name | cost of refusing | cost of substituting |
|---|---|---|---|
| CLI | a human, once | an error message | ★ plausible wrong colour, nobody told |
| library in a PDF reader | a *document*, at scale, unattended | ★★★ a page that will not render, which the standard forbids | nothing **if disclosed** |

**Why:** the two surfaces face opposite failure modes. Rule 1 (*a wrong
colour looks exactly like a right one*) argues for refusal only where
there is a human to tell.

**How to apply:**

- **The decision taken is narrow and is the useful part:** neither
  behaviour may be changed by citing the other. ★ **The commit to catch is
  a *tidying* one** — making the two surfaces "consistent" in one
  direction with one justification. Its diff reads as a simplification
  while it trades a conformance defect for a colour defect or the reverse.
  **If it changes both, it needs two reasons.**
- **Check existing mechanisms before proposing a third.** Both were read
  and neither generalises: `DestinationProvenance` is a payload-free
  two-variant enum (`DL-050`); `SpecDeviation` carries
  `declined: Vec<Signature>` and is about **icccer** declining a capability
  — the opposite shape, since here the *caller* handed us an unnameable
  thing. Candidate: **`IntentProvenance`** (requested name, intent used,
  substitution occurred). ★ **Verified 2026-08-25 that no such type
  exists** — grep before recommending it.
- **§8.6.5.8 is `[REPORTED]` by `pdfce` and NOT re-derived.** Re-derive
  from `PDF_Spec\_sources\` first — see
  [[icccer-wrong-clause-refusal-and-discarded-halves]] (`DL-057`), where an
  attractive wrong clause number was persuasive precisely because it was a
  citation.
- Related: [[icccer-disclosure-is-not-enforcement]] — disclose rather than
  perform silently is this project's standing answer to exactly this
  shape.
