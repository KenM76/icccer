---
name: icccer-licensed-tool-split-outside-the-tree
description: DL-071 — a tool that must quote licensed material is split into a generic engine (tracked) plus a case file that lives OUTSIDE the repo, not gitignored inside it. And the deleted predecessor took the only auditor of its own numbers with it.
metadata:
  type: project
---

**When a tool must carry licensed third-party specifics to do its job,
SPLIT it: a generic engine containing no numbers (tracked, public) plus a
case file holding every licensed value — and the case file lives OUTSIDE
the repository, not in a gitignored path inside it.**

**Why:** operator decision, **2026-08-25**, in his own terms — *"split it
into a public generic tool plus local file."* `tools/suite/shading_cell.py`
had inlined a licensed suite's shading dictionary, indirect-object
numbers, endpoint colours and the embedded press profile's identity as
**constants**, which is the same entanglement that untracked
`docs/SUITE_COMPATIBILITY.md` on **2026-08-18**. ★ **The placement clause
is the load-bearing half: a gitignored path is one `git add -f` away from
the thing the decision was made to prevent.** An ignore rule protects
against inattention, not against a deliberate act, a tooling default, or
a future contributor who reads the entry as *"build noise."*

★ **It was filed as `DL-071` rather than folded into the 2026-08-18
decision because THAT DECISION IS NOT IN THE DECISION LOG AT ALL**
*(verified)* — it lives in a `.gitignore` comment, in `NEXT_SESSION.md`,
and in an untracked document. A pattern recorded only in a `.gitignore`
comment is one refactor from being lost.

**How to apply:**

- The test for what is case data is **not** *"is this secret"* but
  ***"did I read it out of somebody else's file?"*** Plumbing (how to
  inflate a stream, how to sample an annulus) may be public; a colour
  value, pixel box, stitch bound, object number or document name may not.
- ★★ **A second property fell out and is worth keeping: a tool that reads
  its comparison from DATA can be made to REFUSE an unlabelled claim.**
  `patch_compare.py` requires `reference.evidence_class` with no default,
  prints it and the provenance string above every table, and has **no ΔE
  and no flag for one** — so `DL-070`'s prohibition moved from prose into
  code. A hard-coded harness cannot do this: there is nothing to declare.
- ★★★ **The costs, because a decision with only upsides is
  under-examined:** the public half is unrunnable as shipped (mitigated by
  a worked example using only Windows-shipped profiles); a case file has
  **no history, no backup, no review**, so a ledger row is anchored to an
  artifact the project does not version; and **the old tool was DELETED,
  not archived** — so the claim that the new one reproduces the old
  numbers exactly is `[REPORTED]` and **now unfalsifiable**
  ([[icccer-negative-finding-removes-its-auditor]]'s shape: the act that
  settled the question removed the only thing that could re-open it).
  **If a re-run ever disagrees with `NC-301` … `NC-305`, that is not
  evidence about the transform.**
- ★★★ **Still open, and it is the operator's call, not an engineer's:
  `NUMERIC_CLAIMS.md` §3.38 is TRACKED and already carries all four
  categories the harness was deleted for inlining**, including the
  profile's quoted `cprt`. Recorded at §7.28 item 6; **do not resolve it
  by editing the ledger** (append-only, and it is a licence call).
- **Never** accept *"we'll just gitignore it"* for the next such tool —
  that is a reversal of this decision and needs a new one.

Related: [[icccer-gitignored-file-invisible-to-citation-sweep]],
[[icccer-pass-status]], [[icccer-fitted-table-cannot-express-a-clamp]],
[[icccer-free-to-disagree]], [[icccer-artifact-existence-is-not-obligation-status]].
