---
name: project-malformation-channel-overclaims
description: Interpretation-failure and conformance-failure were two claims sharing one channel; a v2 profile that violates nothing printed "malformations: 1" and that reached a graded row. RESOLVED — a separate violations: counter shipped and the graded row was re-derived to grade violations, not disclosures.
metadata:
  type: project
---

Measured on the four rendering-intent fixtures with the shipped
`iccce inspect`, 2026-08-18.

| fixture | `header.intent` line | `malformations:` |
|---|---|---|
| `v2-rendering-intent-high-bits` | `65537 (UNKNOWN)` | **0** |
| `v2-rendering-intent-low-half` | `4 (UNKNOWN)` | **1** |
| `v4-rendering-intent-low-half` | `4 (UNKNOWN)` | 1 |
| `rendering-intent-high-bits` (v4) | `65537 (UNKNOWN)` | 1 |

## 1. ★★ Two different questions, one channel

*"Can iccce interpret this value?"* and *"does this file break a rule?"* are
different questions with different answers, and ICC.1 answers only the second.

* Row 1 answers them **separately and correctly** — nothing is interpretable,
  nothing is violated. That is the CLI's `intent_name` match doing the right
  thing, and it does it by **accident**: it is a second, edition-blind copy of
  the 0..3 knowledge sitting ~90 lines from the gate that was just made
  edition-aware.
* Row 2 **conflates** them: `Malformation`'s own doc comment says *"A rule
  violation the file carries"*, and ICC.1:2001-04 6.1.11 makes this file carry
  none (`A56`). The emitted *words* are careful — *"define only 0..=3 and do not
  forbid others"* — the *channel* is not, and `malformations: N` is a printed,
  machine-readable count.

**How to apply:** before adding a variant to `Malformation`, ask whether the
condition is a rule violation. If it is not, the variant is a *disclosure* and
the count it increments is being read as something it is not.

## 2. There is precedent, which is why this is a doc defect and not a new one

`TrailingBytes` is already documented as **normal** for a profile embedded in a
PDF or TIFF. So the channel has carried non-violations since Pass 2, and the
enum's doc comment is now falsified by **two** of its own variants. That
precedent is also why `v2-rendering-intent-low-half` is filed `Malformed`
rather than needing a fourth fixture category: the category has meant "exactly
one named report is required" since Pass 2, never "the file is illegal".

## 3. ★ The CLI's UNKNOWN has TWO independent justifications, and the weaker
one is the one that gets quoted

`icc-engineer`'s reason: v2 never states the field's *type* (Table 9's cell
reads *"see below"*), so reading the low half as the intent is itself an
inference, and printing `media-relative` would assert that inference in emitted
output. True, but it rests on the type question.

The second reason does not: **the v2 high half is vendor space**, so a profile
with `0x0001` up there may be signalling something iccce has not read. Printing
`media-relative` would claim the field was understood when half of it was
discarded unexamined. The raw `65537` is printed alongside, so `UNKNOWN` costs a
reader nothing — `65537 & 0xFFFF` is theirs to compute.

## 4. The gap that would lose this silently

**No test covers the CLI's `header.intent` line.** A later "tidy-up" that masks
to `h.rendering_intent & 0xFFFF` would print `media-relative` for row 1 and the
disclosure would vanish with a green suite. One test in
`crates/iccce-cli/tests/` closes it; not landed 2026-08-18 because the operator
scoped the task to "tell me which you think it is", and asserting an unnamed
choice would have decided it.

## 5. ★★★ It reached a GRADED ROW, 2026-08-21 — `passh/B` was RED and no profile was defective

> **SUPERSEDED 2026-08-28 — see §6. The row is green and the channel is split.**
> Kept verbatim because the reasoning that produced the fix is the record.

At tip `0a88ad6` the difftest suite is **`pass=372 fail=1`** in the main tree
(and `371/1` in a worktree, which lacks `vendor/` and skips one `passl` row).
The failure is
`passh/B/acceptance/no-malformation-is-disclosed-on-any-accepted-file`,
observed **`5.000000e0`** against a required `0`, on
`ITU-RBT709ReferenceDisplay.icc`, `PSOsc-b_paper_v3_FOGRA54.icc`,
`PSOuncoated_v3_FOGRA52.icc`, `SC_paper_eci.icc` and `sRGB2014.icc`.

**All five are v2** (`2.0.0` or `2.4.0`) carrying a 16-byte MD5 at bytes
84..99 — the position v4 later named `profileID` — with 100..127 all zero.
`0a88ad6` correctly stopped reading that as a `profileID` on v2 and widened
`HeaderReservedNonZero` to name `84..128`; `Malformation::violation_status`
correctly returns **`NotAViolation`** for v2 (ICC.1:2001-04 Table 9 states
nothing; ICC.1:2022 7.2.19 states a `shall`). **Nothing in the parser is
wrong and no published profile is defective.**

★★ **The `malformations: N` line does not consult `violation_status`.** So a
correct, edition-aware disclosure is counted as a conformance failure by a
row whose expectation is *"a profile published as conformant contains nothing
for a conformant parser to disclose"*. This is §1's conflation reaching a
**graded row** for the first time, and the remedy is in the **channel**, not
in `passh/B`'s bound — widening that bound would be `CLAUDE.md` rule 5 in
reverse. Candidates: a second counter (`violations: N` beside
`malformations: N`), or `passh/B` grading violations rather than disclosures.
**Not this role's to choose**; reported to the lead 2026-08-21.

★★★ **And `passh/B`'s expectation was already falsified in writing.** The CLI's
own comment above `println!("malformations: {}", profile.malformations.len())`
has said since `DL-063` (2026-08-19) that the count *"can be non-zero for a
fully conforming file"* and that *"a consumer reading N as a conformance
verdict will condemn one"* — which is exactly what `passh/B` does. **The row is
measuring the wrong quantity, not measuring the right one too tightly.** A doc
comment retracted a claim and the graded row that rested on it was not
re-derived: [[a-fixed-defect-goes-stale-in-someone-elses-doc]] with the
direction reversed — here the DOC was corrected and the TEST went stale.

Related: [[project-the-four-cell-gate-and-its-injections]],
[[project-header-rendering-intent-finding]].


## 6. ★★★ RESOLVED, and the remedy was the one §5 named — re-verified 2026-08-28 at tip `3af2d87`

The channel was split. `iccce inspect` now prints **`violations: N` beside
`malformations: N`**, and `passh/B` was **re-derived rather than re-bounded**:

| row | grading | observed |
|---|---|---|
| `passh/B/acceptance/no-VIOLATION-is-disclosed-on-any-accepted-file` | tolerance **0** | **0** |
| `passh/B/acceptance/lawful-disclosures-on-accepted-files` | **ungraded (`inf`)** | **5** |

So the five disclosures are still counted and still visible — they were moved
out of the *conformance* row and into an ungraded observation, which is what
§5 asked for. **The bound was not widened.** Whole suite `pass=374 fail=0
skip=9 error=0`.

★ The same disclosure fires on **real producer profiles**, not only on
fixtures: `HeaderReservedNonZero` is what a **v2.4** file carrying a valid ICC
profile-ID MD5 at bytes 84..99 (with 100..127 zero) looks like to an
edition-aware parser. I verified the MD5s recompute correctly under the ICC
rule (flags, header rendering intent and the ID field zeroed) — **the files are
not defective and the parser is not wrong.** This is now the ordinary case, not
a curiosity, so any future doc sentence of the form *"N of N profiles, zero
malformations"* is a **dated observation that will go stale the next time the
disclosure set widens** — write the count with its date and its tip, or write
`violations` instead.

**How to apply:** grade `violations`. Report `malformations` as an ungraded
observation with its date. Never write a bare "zero malformations" claim about
a corpus without the tip it was measured at.


Related: [[the-artworks-design-contract-as-evidence]].
