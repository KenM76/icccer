---
name: pass5d-source-black-vs-cmsdetectblackpoint
description: Pass 5d (2026-11-13) — lcms2 forces BPC INSIDE its own black estimator on v4; uses 400% ink at v2 perceptual; an "increment" disagreement can be interpolation re-seen through a steeper TRC, not the quantity under test.
metadata:
  type: project
---

Pass 5d cross-checked icccer's source-side ISO/CD 18619 4.2.3 estimator (`5c66587`) against lcms2's `cmsDetectBlackPoint` CALLED DIRECTLY via `tools/difftest/probes/lcms2_black_point.c` (build: `probes/build-probe.ps1`). Record: difftest README §29, TOLERANCES §3.5.10, NUMERIC_CLAIMS §3.43 (NC-320..329).

Findings that are not obvious:
- **lcms2 forces BPC inside its OWN estimator's round trip** on a v4 profile (cmscnvrt.c L1132): Lab0 -> PERCEPTUAL_BLACK -> B2A0. On the v4 mAB fixture that's the whole 3.137 dE76 at relative. icccer follows literal 4.2.3. Same mechanism as NC-078.
- **lcms2 at v2 perceptual/saturation uses 400% ink through A2B (DarkerColorant)**, not 4.2.3's B2A0(Lab0). SWOP: L* 0 vs icccer 0.804. icccer is the spec-following one.
- SWOP relative: estimator agrees; the 0.017 L* is entirely A2B1 interpolation (lcms2's A2B1 of icccer's LocalBlack reproduces lcms2's black exactly — u16 input quantisation makes it exact).
- End to end the BPC "increment" disagreement (3.4e-3) was 7x what the black difference allows (4.5e-4). A first-order model (BPC-off interpolation disagreement x slope ratio of sRGB encode at the on vs off operating point) explained 85%. **Before blaming the quantity under test for a differential's growth, model how the OTHER known disagreement is transported by the change.**
- Full-grid increment was clamp-bounded (equal to off_max at an out-of-gamut point); restrict increment metrics to the unclamped population.

**How to apply:** for any lcms2 black-point comparison, name the branch (the probe prints its reading) and check for forced BPC in the round trip before calling a difference an estimator disagreement. Related: [[pass5c-estimator-branch-finding]], [[lcms2-findings-legacy-lab-and-forced-bpc]].
