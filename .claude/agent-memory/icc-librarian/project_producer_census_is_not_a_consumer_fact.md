---
name: producer-census-is-not-a-consumer-fact
description: NC-307 + the new producer-corpus-census class (2026-08-25) — a count of what PRODUCERS wrote says nothing about any READER, demonstrated by the measurer itself; and rule 11 was applied AGAINST the dispatch, because a rule is written about the MATERIAL, not the MEASURER.
metadata:
  type: project
---

**Two separate findings from the same filing, and both are about scope.**

## 1. A count of what producers WROTE is not a fact about any reader

`NC-307` (`NUMERIC_CLAIMS.md` §3.39) — `pdfce`'s census of its own
`fixtures/` corpus: **4 239 files, 350 deep-scanned, 0 of 350 name more
than one rendering intent.** New evidence class
**`producer-corpus-census`**, tested against folding it into
`apparatus-census` and rejected: that class's discipline is *"every census
row carries the command"*, and a producer census has a second ambiguity a
command cannot resolve — **which files**.

★★★ **The demonstration is perfect and worth quoting: the project that
took the census honours none of the intents it counted.** `/RI` is an
explicit no-op there and `b"RI"` occurs **zero times** under their
`crates/` *(counted 2026-08-25)*.

**Why it was worth a weak row anyway:** the alternative was not a better
number, it was **a guess**. `ARCHITECTURE.md` costed a `4 × 2` cache at
`~13` s / `~290` MiB on an assumption about caller behaviour that this
project could never measure — it has no PDF corpus and by §4 must never
learn what a PDF is. **A design parameter that can only come from the
consumer has to be asked for** (`CLAUDE.md` rule 10 item 1: requests flow
both ways; not asking is the failure mode).

**How to apply:** the census **bounds a cache**; it does **not** retire
ISO 32000-2 §11.7.5.3's per-paint requirement, because no page tree was
built. *A document that names one intent says nothing about whether a page
switches intent mid-stream.* Also carry the counting unit: object
references are a **lower** bound on distinct profiles and an **upper**
bound on cache entries. Both bounds travel or neither does.

## 2. ★★★ A rule is written about the MATERIAL, not about the MEASURER

The dispatch asked for the licensed corpus's census as ledger rows,
reasoning *"they are pdfce's own census of their own copy."* **Declined.**
`CLAUDE.md` **rule 11** forbids adding **a measurement** over that suite to
a tracked file, and `NUMERIC_CLAIMS.md` is tracked and public. Whose hand
held the instrument does not change what was measured.

**Why:** a rule with a clear subject gets narrowed by a plausible
exemption offered in good faith by someone who is not the operator.
`DL-038`/`DL-040`: **exemptions are declared, never acquired.**

**How to apply:**

- Read rule 11 **verbatim** before applying it — the dispatch paraphrased
  it more narrowly than it is written, and it names *"not a
  measurement"* explicitly.
- The split used: non-suite population → `NUMERIC_CLAIMS.md` `NC-307`;
  licensed population → **`docs/SUITE_COMPATIBILITY.md` §10** (untracked,
  which rule 11 itself names as where that work continues). A tracked
  document may **point** at an untracked one by section.
- ★ **State the cost, don't gloss it.** *"How many output-intent profiles
  per document?"* now has **no tracked answer** — `NUMERIC_CLAIMS.md`
  §7.29 item 2. Silence would have implied the measurement was never
  taken.
- Related: [[iccce-gitignored-file-invisible-to-citation-sweep]] — §10
  lives in the file a default sweep cannot see. Search it **by name**.
