---
name: project-intent-selection-fixture-and-its-two-holes
description: The committed synthetic fixture that separates B2A0/B2A1/B2A2 (built 2026-09-01 on pdfce's ask) — a CONSTANT added to every CLUT node is the construction that makes a separation exact under any interpolation scheme; and the two holes injection found in my OWN assertions (a subset-of-pairs predicate, and a downstream CLIP that annihilates a differential).
metadata:
  type: project
---

**Built 2026-09-01** for `pdfce`'s
`request_srgb_to_cmyk_with_an_intent_and_why_saturation_is_load_bearing.md`
ask 4. Recipe `v2-cmyk-intent-separated-b2a` in
`tools/gen-profiles/src/recipes.rs` §(c4); test
`crates/iccce-cmm/tests/b2a_intent_selection_is_separated.rs` (6 tests, in
`cargo test --workspace`, which CI runs); rows/scope/injections
`docs/TOLERANCES.md` **§3.12**; narrative `tools/gen-profiles/README.md`
**§4.3**; `fixtures/synthetic/README.md` "third instrument".
Measured: **`K(sat) − K(rel) = 0,200 000 000 000 000`**, worst deviation
`1,665×10⁻¹⁶` over 125 probes.

**How to apply.**

- ★★★ **To make a separation EXACT rather than merely large, make it a
  CONSTANT added to every CLUT node.** Every interpolation scheme
  (multilinear, tetrahedral, prism, nearest) is an affine combination of the
  cell's corners with weights summing to one, so a constant present at all
  corners arrives undiminished. **The separation is then independent of the
  PCS point, the source profile, the probe and the geometry** — so the test
  asserts an *equality* (`±1×10⁻¹²`) instead of hoping for an inequality, and
  `NA-006`'s interpolation envelope is identically zero rather than allowed
  for. This generalises past intents to any "which block did it read?" question.
- ★★★ **Choose the offset to be an EXACT 16-bit code, or the rounding stops
  cancelling.** `65 535 = 3 × 5 × 17 × 257` ⇒ fifths are exact
  (`0,20 × 65 535 = 13 107`). Because `round(v + n) = round(v) + n` for integral
  `n`, the encoded difference is exactly 13 107 codes at all 729 nodes.
  **Injection I6 proved this is load-bearing, not decoration:** a floor of
  `0,001` (`× 65 535 = 65,535`, not an integer) broke the equality at the fifth
  decimal — `0,000 991 836` against `0,001`. Legacy PCSLAB scales `a*`/`b*` by
  exactly 256, so any *integer* CIELAB offset is exact there too.
- ★★★ **A DOWNSTREAM CLIP ANNIHILATES A DIFFERENTIAL.** `DL-077` says list what
  the arms SHARE; **also list what the pipeline DESTROYS after they diverge.**
  My `A2Bx` row put its label in the **PCS** (`b*` `+0/+8/+16`) and failed at 2
  of 81 probes: both arms clipped to the *same* cube corner at the RGB
  destination and a real 8-unit difference arrived as **exactly zero**.
  Clipping is idempotent and many-to-one; so is gamut mapping, a quantiser, any
  saturating encode. ★ **The rule was ALREADY WRITTEN in
  `fixtures/synthetic/README.md`** for the header-intent pair (*"every table
  entry is well inside the sRGB gamut, so the separation cannot be erased by
  clipping"*) and I still missed it. **Put the label on the DEVICE side** —
  this fixture's `K` spans `0 … 0,80` and never reaches a clip, which is why
  the graded rows are the destination ones. Exclusion is by a **checked**
  criterion (every channel of both arms pinned at `0,0`/`1,0`), never a chosen
  probe list.
- ★★★ **A distinctness predicate over a SUBSET of the pairs cannot see a
  collapse onto the pair it does not look at.** Injection I4 aliased saturation
  onto perceptual on the source side and the row stayed **GREEN**, because the
  two pairs it graded (`sat−rel`, `rel−per`) still separated. Grade **all
  three** pairs. Generalisation: with `n` arms there are `n(n−1)/2` collapses
  and grading `n−1` of them is not "enough".
- ★★★ **A SKIPPED test and a PASSING test print the same colour.** The claim
  this fixture replaces rested on
  `swop_perceptual_equals_saturation_shared_tag`, which reads a Windows system
  profile and **returns success when it is absent** — green on every CI runner,
  measured nowhere. §6's *"say the scope or say nothing"* needs the corollary:
  **a row whose apparatus can VANISH must state what happens when it does, and
  "returns green" is not an acceptable answer.** My replacement panics on a
  missing committed fixture; no `Option`, no early return.
- ★★ **Assert the SIGN, not the magnitude.** `|K_sat − K_rel| = 0,20` passes
  when `B2A1`/`B2A2` are swapped; the signed form fails (injection I2 measured
  `−0,200 000 000 000 000 12`). The magnitude says *a different* table was
  read; the sign says *the right* one.
- ★★ **A floor is NOT redundant with an equality.** The equality still passes
  if someone re-authors the fixture smaller and updates the constant in step —
  green suite, de-powered instrument. Injection **I6b** ran exactly that with
  exact-code offsets: equality **passed**, only the floor caught it at
  `9,994 659×10⁻³`. Floor reused from Pass K §F (`4×10⁻²`), not minted.
- ★★ **Hold everything but ONE variable equal, and assert the equality as a
  second row.** `C M Y` are byte-identical across the three `B2Ax`, so if any
  intent-conditional default (BPC, K-preservation, gamut mapping) ever appears
  it reddens *that* row instead of quietly becoming the `K` difference the
  headline reads as proof. Injection I7 confirmed the guard has power.
  ★ `Chain::new` today applies **neither** BPC nor K-preservation — both
  opt-in — which is what makes the bare constructor the right surface here.
- ★ **The zero-separation control is a TEST, not a paragraph.** The named rival
  (*"the CMM ignored the intent"*) is executed in the same run on
  `v2-cmyk-chromatic-neutral.icc` (only `B2A0`), giving `0,0`, so the candidate
  separation is a measured `0,20 − 0,0`. Both older CMYK fixtures stay — they
  are the control, not superseded — and the machine-local SWOP test stays too:
  it is the **aliased** arm where this is the **separated** arm.
- ★ **I typed "12 tags" in the recipe's `expect` and it is 9.** Caught by
  running `iccce inspect`, not by review. Same failure as
  [[project-stale-claim-strings-in-emitted-records]]; the generated
  `MANIFEST.md` would have published it.
- ★ Class is **`derived-expectation`** and the fixture is **structural**: a
  flat 20 % black floor is not what a saturation intent does, ICC.1 says
  nothing that would make it so, and `0,20` is not a fact about ink. The
  warning is repeated in four places on purpose — this is the corpus's only
  fixture that names an intent.
- **Not done:** no v4 `mAB `/`mBA ` fixture carries three of anything (gap
  closed for `mft2` only); no `link`/DeviceLink fixture; the compiled path
  (`CompiledTransform`) has no row here; `docs/NEXT_SESSION.md`'s statement of
  the gap is now stale and was **deliberately not edited** (contended by a
  concurrent session).

Related: [[project-passk-f-separating-fixture]],
[[project-prove-the-arm-by-injecting-the-defect]],
[[project-a-fixed-defect-can-blind-its-own-row]],
[[project-synthetic-fixture-corpus-and-gp001]],
[[project-header-rendering-intent-finding]].
