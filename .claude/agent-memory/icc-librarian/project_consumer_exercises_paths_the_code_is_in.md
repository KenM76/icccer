---
name: iccce-consumer-exercises-paths-the-code-is-in
description: DL-078 (2026-09-01) — pdfce's question found an API hole four Passes of our own testing had missed, because a successful `with_bpc()` can be a total no-op; our tests exercise the paths the code is FOR, a consumer exercises the paths the code is IN. Plus the rule that follows: when a call can succeed and do nothing, DISCLOSE, and check whether the ambiguity is the STANDARD'S before touching the code
metadata:
  type: project
---

**Our own tests are written by the people who know what the code is FOR,
so they exercise the paths the code is for. A consumer exercises the paths
the code is IN.** That blind spot is **structural** and no amount of
internal diligence closes it.

**Why:** `Chain::with_bpc()` had existed since Pass 5 with unit tests, a
`TOLERANCES.md` section, two registered approximations (`NA-009`,
`NA-010`) and rows in `NUMERIC_CLAIMS.md`. **Nothing here had ever noticed
that a successful call can be a complete no-op** — because every apparatus
built for BPC was built to measure *what the map does*, and the case in
question is the one where the map does nothing.

It surfaced on **2026-09-01** because **`pdfce` filed a request about its
own use of the flag**. The measurement that followed: `iccce transform
--bpc`, sRGB (v2 matrix/TRC) → U.S. Web Coated (SWOP) v2, 729 grid
operands, ink units — media-relative changed 727/729 (max `0.243951`);
**perceptual and saturation changed `0` of `729`, bit-identical**;
ICC-absolute refused by name. Filed as `NUMERIC_CLAIMS.md` §3.42,
`NC-308` … `NC-319`.

★★★ **This is the first time `CLAUDE.md` rule 10 item 1's claim —
*"a consumer's real usage is the best available check on this library's API
shape, and not asking is the failure mode, not a courtesy"* — has been
demonstrated with an OUTCOME rather than asserted as a rationale.**

★★ It is [[iccce-documented-is-not-tested]]'s shape (`DL-051`: a suite's
power is **per-defect**, not per-suite; the best-documented constant was
caught by **none** of six injections) **arriving from outside instead of
from an injection.** Both say the same thing about self-assessment: the
apparatus you build is shaped by the failure you imagined.

**How to apply:**

- ★★★ **When the request channel produces a question, treat it as a
  CANDIDATE DEFECT REPORT about the API, not merely as a support
  request.** The question *"what does `--bpc` actually do for my case?"*
  was the finding. Ask what the questioner had to know that the API does
  not tell them.
- ★★ **The general audit this licenses, and it is cheap: for every
  `Result`-returning entry point, ask what `Ok` CLAIMS.** If `Ok` can mean
  *"computed, and it came to nothing"* while a caller reads it as
  *"applied"*, the API is under-informative — `with_bpc()` is one instance
  and there is no reason to think it is the only one.
- ★ **A second, smaller instance in the same dispatch:** `pdfce` flagged a
  stale `README.md` status row **twice, across two files**, before it was
  corrected — [[iccce-stale-status-decays-faster-than-a-number]] instance
  #7. When an outside reader reports a staleness, sweep the **class** of
  claim, not the line.

---

### The half of `DL-078` that is a rule about STANDARDS, not about testing

★★★ **Before repairing a degenerate result, find out whose ambiguity it
is.** Here the answer changed the decision entirely. `ICC_Spec` **`A63`**:
**ISO/CD 18619 4.2.5 has four distinct exits that all return `(0,0,0)`, and
4.2.6 then erases the difference between them** — so no downstream
observation can recover which fired, by any means including re-running.

⇒ **The hole is in the PROCEDURE, and our API is the only place it can be
surfaced.** Three candidates, two refused with reasons:

| candidate | verdict |
|---|---|
| **repair** — substitute a black when the clamp goes degenerate | **NO.** That invents a value a **non-silent** clause does not specify. `NA-009` is a labelled subset re-implementation *inside a documented silence*; this would be a departure from printed text |
| **refuse** — error when the map is the identity | **NO.** A conforming outcome is not an error, and it would fail `--bpc` on an entire ordinary class of input (any matrix/TRC source) |
| **disclose** | ★ **CHOSEN** |

★★★ **This is `CLAUDE.md` rule 6 — *the parser reports; it does not repair*
— applied one layer up, to the CMM.** The rule was written about a
malformed tag: a silent correction hides the malformation from the only
layer that could disclose it. **A CMM that quietly substitutes a black
point hides a SPECIFICATION AMBIGUITY from the only layer that could
disclose it** — and here the caller is a *different project* with its own
conformance obligations.

**Two details worth reusing:**

- ★★ **`is_identity()` is bit-exact, not tolerant, and that is a
  decision.** The identity arises when the two black points are *equal*,
  making `a = (W−d)/(W−s)` an exact `1.0` because numerator and
  denominator are the same expression. **A tolerance there would report
  "identity" for a map that is merely WEAK — a different claim.** Two
  different claims must not share a word.
- ★ **Both new tests assert on the map's OUTPUT, not on the predicate.**
  The second is the **negative** case, without which `is_identity()`
  hard-coded to `true` passes.

★★ **`DL-037` still applies: the disclosure GATES NOTHING.** `--bpc`
behaves identically whether or not a caller reads it. **Do not quote
`DL-078` as evidence that a wrong BPC would be caught.**

### The three ways this finding gets over-quoted

1. ★★★ ***"`--bpc` does nothing at perceptual"* is FALSE as a general
   statement.** The zero is a property of **this source's black** (a
   matrix/TRC profile, already at `L* = 0`) meeting **this destination's
   clamped root**. Any LUT source whose black is not at `L* = 0` gives a
   non-identity map at the same intents.
2. ★★★ **The intent-separation figures are a SEPARATION claim, not an
   accuracy claim.** `NC-308` … `NC-311` have **no oracle** — a CMM wrong
   by 20 ΔE2000 at all four intents separates identically.
   [[iccce-agreement-can-be-the-symptom]] is the mirror hazard.
3. ★★ **The unit is INK, and no ΔE may be derived from any of it.** `0.464373`
   is not *"0.46 ΔE"*. [[iccce-fitted-table-cannot-express-a-clamp]] records
   the sibling rule for capture-derived units.

Related: [[iccce-pass-status]], [[iccce-disclosure-is-not-enforcement]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-injection-measures-one-magnitude]],
[[iccce-refusal-at-one-layer-is-nonconformance-at-another]].
