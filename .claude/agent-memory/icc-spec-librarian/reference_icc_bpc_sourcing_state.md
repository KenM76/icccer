---
name: icc-bpc-sourcing-state
description: BPC is SOURCED (Adobe 2006 + ISO/CD 18619:2013 in `_sources/`). ★★ 2026-09-01: at PERCEPTUAL/SATURATION the mid-range short-circuit is UNREACHABLE and FOUR exits of 4.2.5 all return (0,0,0) — so a bit-identical BPC no-op proves NOTHING (A63). `Saturation` occurs ONCE in the whole ISO document ⇒ the perceptual-or-saturation pairing is the DOCUMENTS' shape. BOTH lcms2 version gates switch off for a v2 destination ⇒ M2 does NOT apply there (M7). The v4 fixed-black short-circuit is the UNSOURCED half of a version-gated estimator, not the v2 path (A64). Earlier: forcing verdict NO; 4.2.5.4 returns InitialLab not outRamp[first]; root not vertex
metadata:
  type: reference
---

**Entry point: `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__ref__bpc.md`** (synthesis
+ verdicts, §15–§19 are the current material) →
**`icc\icc__s__bpc_algorithm.md`** (the two documents transcribed side by side).
**Do not re-do the sourcing sweep. Do not re-open §7.1/§7.2/§7.3 — they are
closed.**

## ★★★ 12th pass, 2026-09-01 — PERCEPTUAL / SATURATION on a v2 destination

**Dispatch: a `Chain::with_bpc()` that succeeds and changes nothing.**
Entry points: **`icc__ref__bpc.md` §21** (verdicts + the discriminating run's
pre-registered predictions at §21.7) and **`icc__s__bpc_algorithm.md` §9**
(clause trace, verbatim, exhaustive counts). New: **`A63`**, **`A64`**, **`M7`**,
**`D11b`**. **Do not re-derive any of the five below.**

1. **★★★ `(0,0,0)` IS FOUR STATEMENTS WEARING ONE VALUE (`A63`).** 4.2.5 exits to
   `(0,0,0)` at **4.2.5.3** (invalid ramp), **4.2.5.5** (`<3` shadow points),
   **4.2.5.5** (`d ≤ 0`), and **4.2.5.5's fitted root clamped by `max(0,…)`** —
   the last a *computed result*. **4.2.6 then erases the difference**: against a
   source black of `L*=0` all four make the map the identity. ⇒ **"BPC was not
   needed" and "BPC could not be computed" are the same observable event**, so a
   bit-identical no-op is **not evidence the procedure ran**. Test power, not
   magnitude (`C8`). The instrument must be inside the estimator.
2. **The mid-range short-circuit is UNREACHABLE at perceptual/saturation.**
   4.2.5.4's opening sentence and 4.2.5.1 both gate it on `RelativeColorimetric`.
   `InitialLab = (0,0,0)` there is an **input to the ramp**, never the answer —
   so the 11th pass's `InitialLab`-vs-`outRamp[first]` question **does not arise**
   at these intents. **4.2.5.2.1 VERBATIM: *"If RenderingIntent is not
   RelativeColorimetric, then InitialLab shall be set to (0,0,0), else…"* — and
   EVERYTHING after that (CMYK `LocalBlack`, `D()`, chroma zeroing, `L*>50` clip)
   is inside the `else`, i.e. relative-only.**
3. **★ EXHAUSTIVE, two engines: `Saturation` occurs ONCE in the whole of
   ISO/CD 18619** (4.1's allow-list) and **`Perceptual` three times, never as a
   condition** (twice as a fixed forward intent inside `T(…)`). `RelativeColorimetric`
   ×6. **The control flow is binary.** Adobe pairs them positively twice
   (`"Else if (UserIntent is Perceptual or Saturation)"`). ⇒ **a two-variant
   estimation-intent type is the DOCUMENTS' shape and needs no justification;
   the arm that does is ICC-absolute, which 4.1 excludes outright.**
4. **★★ `M2`/`DL-013` DO NOT APPLY TO A v2 DESTINATION (`M7`).** Both lcms2
   `cmsGetEncodedICCversion(...) >= 0x4000000` gates switch off — the fixed
   `cmsPERCEPTUAL_BLACK` short-circuit (`cmssamp.c` L433) **and** forced BPC
   (`cmscnvrt.c` L1127-1134). lcms2 then runs the same 4.2.5 procedure. **Any
   unqualified "lcms2 forces BPC at perceptual" is false of the commonest real
   case.** Also `M7`: **both detectors return a `cmsBool` that `ComputeConversion`
   DISCARDS** — the same conflation as `A63`, by a second route.
5. **★★ THE SOURCING RUNS OPPOSITE TO THE APPEARANCE (`A64`, `D11b`).** In
   `if (v4 && perceptual) → fixed PRM black; else → 4.2.5`, the **v2 path is the
   SOURCED half** (`shall` end to end) and the **v4 short-circuit is the
   IMPROVISED half** — no BPC document contains any version-keyed rule (`A45`).
   ICC.1:2001-04 defines **no perceptual black at all**; 6.3.4.3's *"frequently
   use zero"* is descriptive and hedged. ⇒ **`(0,0,0)` at a v2 perceptual
   destination is defensible as an OUTCOME and NOT SOURCED as an ASSUMPTION.**
   Behavioural consequence (`D11b`): **on v4 BPC can never be a no-op; on v2 it
   usually is.** lcms2 gates on perceptual **or saturation**, iccce on perceptual
   **only** — unmeasured divergence at a v4 saturation destination.

**Already held, do not re-measure:** `outRamp[0]` of the *perceptual* ramp on
`USWebCoatedSWOP.icc` is **`L* 16,571 474`** (= Pass 5c's `bt_perc`, = lcms2's
`BlackPointUsingPerceptualBlack` before its clip) ⇒ the ramp is **valid**, exit A
does not fire in a correct implementation there.

## ★★★ 11th pass, 2026-08-12 (later) — the short-circuit return value

**`icc-conformance` asked one clause question and it found a shipped iccce bug.**
At **4.2.5.4**'s mid-range straightness short-circuit (no curve fitted),
**ISO/CD 18619 returns `InitialLab`, unchanged — stated TWICE in `shall`
language (4.2.5.1 Overview and 4.2.5.4's last paragraph) and NEVER in terms of
`inRamp`/`outRamp`.** `outRamp[first]` occurs in the whole of 4.2.5 only as
`MinL` and as the left side of the 4.2.5.3 validity test — **it is not a
candidate for `DestinationBlackPoint` in any branch.** **lcms2 returns
`InitialLab` and CONFORMS; iccce returned `outRamp[first]`.** Measured on
`USWebCoatedSWOP` at rel.col.: **`8,166 8×10⁻² ΔE76`, 100 % `L*`** — small, and
**100 % of the divergence on that fixture.** **Filed as a BUG, not an ambiguity
row** (the two candidates are not two readings — one has no textual support).

**Three consequences that generalise:**
1. **On a real ink profile the quadratic fit NEVER RUNS** — both implementations
   short-circuit. Everything about the shadow window, `n < 4` and vertex-vs-root
   is **dead code on `USWebCoatedSWOP`**. Any prediction that named it is
   untested, not confirmed.
2. **§17.3's chroma prediction is INERT on CMYK** (4.2.5.2.1 zeroes `a*`/`b*`,
   and lcms2's `BlackPointUsingPerceptualBlack` does the same). It needs a
   **Gray/RGB LUT** destination. The scope line was right; a paraphrase of it in
   `D:\Dev\iccce\docs\TOLERANCES.md` was not — **the sentence "precisely lcms2's
   method-4 (quadratic-fit) territory", attributed there to §17.3, does not
   exist in this corpus** (grepped three tokens, zero hits).
3. **New ISO defect (`§4.6`): 4.2.5.4's opening sentence sends an *invalid*
   `outRamp` to curve fitting, contradicting 4.2.5.3 and 4.2.5.1**, which both
   `shall` it to `(0,0,0)`. Corpus adopts `(0,0,0)`, 2 clauses to 1. Also: the
   source misspells `InitalLab` in 4.2.5.1 (both engines agree — it is theirs).
4. **The short-circuit is the ONLY branch of 4.2.5 that can return a CHROMATIC
   black** (4.2.5.3 → `(0,0,0)`, 4.2.5.5 → `(z,0,0)`), because 4.2.5.2.1 zeroes
   chroma **only for CMYK**. Do not "fix" that to neutral.
5. **`§5.4` step 4's "Not verifiable here" is RETRACTED** — lcms2's
   `shall`-voice comment IS 4.2.5.4's final paragraph, near word-for-word.

**Figure 1 (p.12, "DestinationBlackPoint computation logic sequence") has NO
extractable text** — image only, `pdfminer.six` returns the caption alone. It
cannot corroborate the prose; do not go looking again.

## The four things that changed on 2026-08-12 (10th pass, earlier)

1. **★ THE FORCING VERDICT — NO, and it does not change a shipped policy.**
   lcms2 forces `BPC = TRUE` at v4 perceptual/saturation *"following Adobe's
   document"*. **Adobe's document does not say that.** Of the three clauses in
   that source comment, **two are faithful** (devicelink excluded, absolute
   excluded) and the third is absent. Positively: Adobe **6.1** makes BPC
   user-deselectable, **6.2** says for perceptual *"BPC should not be
   necessary … available, however … to be used with malformed profiles"*,
   **6.3** declares profile version irrelevant, and ISO/CD 18619's **Annex A**
   calls BPC *"an optional feature that the user can enable or disable"*.
   **Exhaustive search of both documents: `ersion` 25 + 18 hits all read, zero
   `V4`/`force` — no version-keyed rule exists in either.**
   **⟹ iccce's `DL-022` (never force) is CORROBORATED; nothing reverses.**
   New register row **`A45`, RESOLVED-as-DELEGATED.** `M2` itself still stands
   — it was measured; only its **attribution** is retracted.
2. **★ `A42` UNVERIFIED → PARTLY RESOLVED, and lcms2 is a HYBRID.** ISO/CD 18619
   clause **4.2** specifies the whole estimation procedure in `shall` language,
   and **every threshold this corpus called "unattributed" is in it verbatim**
   (`0.2`, `≥ 4`, `[0.1,0.5)`, `[0.03,0.25)`, the `L* ≤ 50` clamps, 256 ramp
   samples, `±50` chroma clamp, `1.0E-10`, `max(0,min(50,·))`, `n < 3`).
   **Three constants are still lcms2's own and unattributed anywhere:
   `L* > 95 → 0`, `IsEmptyLayer`'s `0.002` (`M6`), and `n < 4` in the fitter
   (which contradicts ISO's `3` AND lcms2's own caller).**
   **lcms2 follows ISO for the destination curve fit and ADOBE for the darkest
   colour** — it has no `D()` vertex search.
   **Not RESOLVED** only because the held document is a **committee draft**.
   **Binding citation form: "ISO/CD 18619:2013 clause 4.2.x", NEVER
   "ISO 18619".**
3. **★ A LOUD prediction owed to `icc-conformance` — SOURCE-READ, not measured.**
   ISO 4.2.6 says the black points' `a`/`b` *"are ignored"*; **lcms2 retains
   chroma at three points and propagates it through a per-channel map.**
   DERIVED two ways: **at input black the divergence equals exactly
   `√(a*² + b*²)` of the detected destination black — 2 to 6 ΔE76** for `b*`
   between −2 and −6 — decaying to zero at white. **Scope: relative
   colorimetric, LUT destination, chromatic black.** If iccce implements ISO
   faithfully this WILL show up in a difftest and it is **not** an error.
4. **★ Vertex vs root, settled.** Adobe §7.2 Step 4 v takes the **VERTEX**
   (`−u/2t`) and openly calls it *"an approximation"*; ISO 4.2.5.5 takes the
   **ROOT** with a `|a| < 1.0E-10` linear branch and a `[0,50]` clamp; **lcms2
   does the root**, under a stale comment saying "vertex". **Implement the root**
   — the vertex is unbounded as the fit straightens, which is the common case.

## The tier answer (it gated NUMERIC_CLAIMS NC-084)

**`icc__ref__bpc.md` §2 (the scaling map) — YES, promotable.** Stated four ways
(ISO/CD 18619 4.2.6 · Adobe §7.3 · ICC.1 6.3.4.3 · lcms2) with **zero
disagreement**, proved symbolically (`sympy` → exactly `[0,0,0]`) and numerically
(`decimal` prec 50 → `1,0×10⁻⁴⁹`). **Cite ISO/CD 18619 4.2.6** — the only one
written as a runtime rule between two profiles.
**§3 (the `A41` perceptual black triple) — NO.** Neither BPC document mentions a
fixed perceptual black at all, so `A41` is untouched and anything resting on that
triple stays `derived-expectation`. **Two halves of one file, two tiers — saying
"the BPC file is primary now" would be false for half of it.**

## Document identity and licences — check before quoting

- **`_sources\BlackPointCompensation.pdf` is ISO/CD 18619, NOT WP40.** WP40 is
  its Bibliography `[1]` and is **superseded** — do not put it back on a wanted
  list. Lineage: Adobe 2006 → WP40 2010 → ISO/CD 18619 2013 → ISO 18619:2015.
- **`AdobeBPC.pdf` has TWO OPPOSITE grants: text reproduction PROHIBITED, but
  the ALGORITHM is patent-free and Adobe explicitly invites implementation.**
  Do not conflate them. Implementing is fine; carrying the prose is not.
- **`PDF20_AN001-BPC.pdf` is CC-BY-4.0** — the only freely quotable one.
  **PDF 2.0 binds `UseBlackPtComp` to ISO 18619:2015 by name**, and its default
  value `Default` means *"up to the PDF processor"* — a third corroboration of
  `A45`.
- Per-document provenance and terms: `D:\Dev\iccce\docs\LEGAL.md` **§2.5**.

## Extraction

**For the two BPC PDFs `pypdf` is the STRONGER engine and poppler the weaker** —
the inverse of ICC.1:2022. `pdftotext -layout` turns every non-Latin-1 glyph into
`U+FFFD` (20 / 49), losing `≤`, `×` and the **`U+2013` en-dash minus signs**.
**Run both anyway: their losses are complementary** (pypdf drops the inline
equation image `y = tx² + ux + c`; poppler destroys the `DecodeL` table's
row↔formula pairing). See [[icc-pdf-symbol-font-sign-loss]].

## Still true from before

- **`A28`**: ICC.1 states the *map* at **6.3.4.3** as an **authoring** fix-up —
  **do not cite 6.3.4.3 *as* BPC** (trap T2, the `C1` failure mode). But "no
  standard defines BPC" is **false**: the ICC standardised it outside ICC.1.
- **`A41`**: ICC.1 Table 16 prints `0,003 357 / 0,003 479 / 0,002 869`; lcms2 and
  iccDEV are byte-identical to each other at `0.00336 / 0.0034731 / 0.00287`.
  **Use the implementations' triple.** `0,037 ΔE76 / 0,050 ΔE2000` — exactly zero
  at 16-bit PCS, and on a float path the same order as a whole scenario budget,
  so **sub-perceptible ≠ negligible**.
- **`M6`**: `IsEmptyLayer` drops the stage below an L1 `0,002` ⟹ no BPC once the
  two blacks are within ≈`0,41 L*`; the same guard drops the ICC-absolute white
  stage. **SOURCE-READ, never run.** Still unattributed after this ingest.
- **`bkpt`**: parse and report it; do not use it.
- **`D11`/`A43` did NOT close.** Neither BPC document mentions clause 6.3.4.3,
  the PRM black, or any fixed/version-keyed black — so **both** the iccDEV and
  lcms2 routes are outside both documents. A43 stays SILENT for a
  better-understood reason.

Related: [[lcms2-measured-behaviour-file]], [[icc-tos-automated-access-blocker]],
[[verify-a-documents-identity-from-its-title-page]],
[[label-the-predicate-not-just-the-payload]]
