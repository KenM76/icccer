---
name: iccce-fitted-table-cannot-express-a-clamp
description: DL-069/DL-070 (2026-08-25, PCS010) — two rival tables missing in OPPOSITE directions is the signature of a DISCONTINUITY, not bad fitting; and the reference was a screen capture, the weakest evidence class this ledger has ever admitted. ★ DL-070's boundary moved 2026-08-28: DL-074 put a ΔE WITHIN one capture outside it, then DL-075 ruled that BETWEEN TWO captures is INSIDE it — the rule attaches to the artifact — and left the two in tension.
metadata:
  type: project
---

**A fitted interpolation table cannot reproduce a normative clamp,
because a clamp is a DISCONTINUITY and an interpolant is defined by not
having one — and the same argument runs against iccce's own CLUT path,
one stage earlier.** `ARCHITECTURE.md` **DL-069**; rows `NC-301` …
`NC-305` in `NUMERIC_CLAIMS.md` §3.38; narrative
`SUITE_COMPATIBILITY.md` §4.7.

**Why:** `pdfce` reported (2026-08-25, `Pass 122.7`, a note marked
*"no reply owed"*) that its three interim `DeviceCMYK`→sRGB tables all
miss Acrobat on one suite patch **in opposite directions** — `naive`
blue 0 / green 143, `calibrated` blue 55 / green 165, Acrobat 0 / 156.
Read as a fitting problem that is baffling. Read as a **shape** problem
it is forced: the colour is **out of sRGB gamut on the blue axis**, so a
real CMM clamps each linear component to `[0,1]` **before** the inverse
TRC (ICC.1:2022 **Annex F.8–F.16**, `MatrixTrc::pcs_to_device`) and lands
on **exactly `0.000000`**. A 6×6×6×6 grid interpolates **smoothly** and
can sit on the boundary or above it — **not both**.

**★ The half that cuts at us.** iccce's LUT path interpolates the press
profile's `A2B0` CLUT (`clutPoints = 16`) and smooths across whatever
discontinuities that data holds, **exactly like the fitted table**. The
clamp survives only because it is applied **analytically, after every
interpolation**. ⇒ **a normative discontinuity must live in CODE, on the
far side of every interpolation.** Corollary: a pre-computed fast-path
table (rule 8 territory) **inherits the limitation** and must re-apply
the clamp after the lookup.

**★ The diagnostic worth reusing:** when two implementations miss a
target in **opposite directions** on one channel, suspect a **boundary**,
not a calibration error — then discriminate with a **sweep, not a point**.
Output that moves with its input is converged; output that is
**bit-identical across a range** is clamped. `NC-302` is seven points at
exactly `0.000000`.

**★★ DL-070, filed the same day: a screen capture is a RENDERING
cross-check and nothing stronger, and NO ΔE MAY EVER BE COMPUTED AGAINST
ONE.** The reference is a `PrintWindow` grab of a running Acrobat through
an **unrecorded** display transform and monitor profile, quantised to 8
bits. New evidence class **`rendering-cross-check-screen-capture`**,
which **ranks BELOW `implementation-cross-check`** (lcms2's path is
pinned; a capture's display leg is not). Two independent reasons for the
prohibition: it dresses a rendering comparison up as colorimetry; **and
at perceptual intent the value is clamped, so a ΔE in sRGB would be
measuring the clamp.** ★ Filed as a **prohibition rather than a caveat**
so that adding the number later means **reversing a decision**, not
filling a gap.

**★★★ DL-070's BOUNDARY MOVED 2026-08-28 — read this before quoting the
prohibition.** `DL-074` establishes that a ΔE taken **between two objects
of ONE capture** is **outside** it: the class DL-070 names has **iccce's
output on one side and capture pixels on the other**, whereas an
intra-render differential has **neither side iccce and both sides from the
same capture**, so the unknown display leg is common-mode and cancels.
★★★ **DECIDED LATER THE SAME DAY — `DL-075`: DL-070 DOES reach a ΔE between
TWO DIFFERENT captures.** The prohibition attaches to the **ARTIFACT**, not
to what sits opposite it (§1's class names *pixels read out of a capture* in
its **head clause**; the iccce side is only the *"Here:"* instance). Two
unknown legs, nothing cancels, reason 1 applies *harder*. **Nothing widened,
nothing exempted.** ★★ And it puts the sentence above under tension: the
same-day withdrawal withdrew the **intra-render** ΔE too, so `DL-074`'s
carve-out **paragraph** may be too strong even though its **class** is
untouched — `DL-075` §7 recommends reading reason 1 as being about the
**UNIT** and leaves the choice open. **Do not state any capture-derived
distance in ΔE; 8-bit counts only.**
[[iccce-prohibition-buys-nameability-not-prevention]] carries the ruling and
the bigger finding (the prohibition did **not** prevent the number — its
author re-derived it, disclosed it as his own limit, and computed it anyway).
★ And DL-070's **reason 2 survives intact** and becomes
`DL-074`'s one-sidedness: **a near-zero agreement may be the clamp
collapsing two different colours together.** Full entry:
[[iccce-intra-run-differential-outranks-cross-renderer]].

**How to apply:**
- Never let this be retold as an accuracy gain over `pdfce`'s tables —
  `CLAUDE.md` rule 10 item 4 stands, the swap is **lateral in evidence
  class**. The claim is **structural and narrow**.
- Never let *"CoatedFOGRA39/perceptual matched all three channels"*
  become *"Acrobat used FOGRA39"* — **three of four FOGRA sources are
  within 3 counts on green**, and it is one point among nine rows.
- `NC-305` is a **negative**: `A2B0` and `A2B2` share tag offset 296, so
  correct selection and a silent fallback are **indistinguishable** on
  that profile — candidate separation zero
  ([[iccce-agreement-can-be-the-symptom]], DL-033).
- The case **can never become a committed fixture**: suite (licensed),
  Heidelberg (the embedded profile), Acrobat (a capture of a commercial
  product), Microsoft (the Windows colour profiles). It lives as a
  **written number plus a harness that takes paths** — so **no test fails
  if the transform drifts.**
- Cross-project twin:
  `C:\personal_rag\pdf\lesson_20260825_fitted_cmyk_table_cannot_reproduce_a_gamut_clamp_because_a_clamp_is_a_discontinuity.md`.

Related: [[iccce-pass-status]],
[[iccce-stale-citation-worse-than-stale-number]] (the harness names
`MatrixTrc::device_to_rgb`, which does not exist),
[[iccce-compatibility-not-certification]],
[[iccce-ground-truth-cannot-exist]], [[iccce-verify-own-draft-too]].
