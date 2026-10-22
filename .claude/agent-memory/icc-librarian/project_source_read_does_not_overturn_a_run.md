---
name: source-read-does-not-overturn-a-run
description: DL-082 — a pinned, file:line-cited correction to an lcms2 behavioural claim was WRONG, and NC-078 (measured, same pin, same pairing) falsified it; every visible provenance signal favoured the wrong answer
metadata:
  type: project
---

**When a source-read and an executed measurement disagree about
BEHAVIOUR, the run holds and the read yields — regardless of which one
carries the better-looking provenance. Before accepting a correction to
an lcms2 behavioural claim, grep the ledger for a row that MEASURED it,
not one that states it.**

**Why:** 2026-09-01, `DL-082`. `NC-319`'s missing lcms2 pin — the oldest
unpaid item in a shipped release — was discharged by a clone at tag
`lcms2.19.1` (`21c582a…`, matching `tools/difftest/lcms2.pin`). Getting
the pin **forced a re-read**, and the re-read came back with a
correction: `cmscnvrt.c:1132` sets `BPC[i]` **per profile**, so *"gated
on the destination being v4"* was allegedly wrong, and *"pair a v4 source
with a v2 destination and the sentence would have been wrong."*

**`NC-078` had measured that exact pairing, at that exact pin, twenty
days earlier, in both directions:** v4 source → v2 destination = **`0,0`
bit-identical** (no forcing); v2 source → v4 destination =
**`3,137 3×10⁻²`**. The correction was **refused** and no tracked wording
was edited.

**The mechanism of the error — it read one site of two.** §3.11.6 had
*already* recorded the per-profile setting (the correction's own line)
**and** the consumption site: `ComputeConversion(i, hProfiles, …, BPC[i], …)`
builds the leg **into** `hProfiles[i]`, so in a two-profile chain the
only flag ever read is the destination's. The dispatch re-found **half**
a mechanism and drew from it a conclusion the other half forbids.

★★★ **The part that makes this worth a memory: every visible provenance
signal favoured the wrong answer.** The original claim had no pin, no
citation, no disclosed method. The correction had all three plus the
credibility of correcting its own author, and **both `file:line`
citations were accurate**. A reader ranking the two on decorations picks
the wrong one confidently. **A pin buys re-checkability. It does not buy
rank.**

★★ **Two second-order lessons:**

- **Pinning is not bookkeeping — and it is still right.** It surfaced all
  of this. Had `NC-319` stayed unpinned the wrong correction would never
  have been drafted, *and neither would the check that caught it*. **An
  unpinned citation cannot be checked, so it cannot be found wrong** —
  that is the real cost of leaving one, higher than "hard to reproduce".
  A process that produces a wrong draft and catches it is working.
- **A correction is the worst carrier for an error.** It supplies its own
  scrutiny — arrives having visibly done the work, retires the
  alternative by name, tells the reader the checking is finished. Here it
  had already gone to the consumer **in place**, with the *correct*
  original named as withdrawn, so `pdfce` holds the wrong one **last**.
  This is the run `DL-080` was waiting for: there the early-sent figures
  reproduced and it was forgiven as *"an outcome, not a method"*; here
  the outcome went the other way.

**How to apply:** (1) A behavioural correction about an oracle is not
accepted on citations — grep `NUMERIC_CLAIMS.md` for an executed row
(`NC-078`, `NC-093`, `NC-095`, `NC-100`, `NC-101`, `NC-102` are the
forced-BPC ones; §10's invalidation table already indexes them together
under *"icccer ever forcing BPC, or lcms2 ceasing to"* — the index this
check needs, built for another purpose). (2) A dispatch's account of
**where its own wrong words live** is a claim like any other: this one
said "the channel reply and the dispatch" and the wording was in **three
tracked files** as well. (3) Refuse the correction, keep the wording,
and file the **scope clause** instead — the shorthand holds for a
**two-profile** chain; three has never been measured (`DL-021`).

Related: [[stale-status-decays-faster-than-a-number]] (the pin/run-id
family — carry the hash), [[provenance-is-a-different-axis-from-evidence-class]]
(better instrument, same `[REPORTED]` class),
[[wrong-clause-refusal-and-discarded-halves]] (a wrong clause is worse
than no clause; a correct citation on the wrong subject invites
acceptance), [[icccer-pass-status]].
