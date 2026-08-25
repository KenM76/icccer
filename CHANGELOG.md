# Changelog

All notable changes to `iccce` are recorded here.

This project's changelog is written to the same standard as its code: it
says **why**, not only what, and it records **what a change does not
claim** wherever that is the part a reader would otherwise assume. A
colour management module can be wrong in ways that look right, so a
changelog entry that describes a capability without its evidence class is
worse than no entry.

Format loosely follows [Keep a Changelog](https://keepachangelog.com/).
Versioning is [SemVer](https://semver.org/); while the major version is
`0`, **the public API may change in any release**.

---

## [0.2.0] — 2026-08-25

Thirty-seven commits since `v0.1.0`, plus the working tree of the release
day. The shape of the release is unusual and worth naming up front: **the
two largest additions are both *named policies inside specification
silences*, not conformance features** — K-only black preservation, and a
second reading of sRGB's transfer function — and the most valuable
paragraphs below are the ones recording what those policies **cost** and
which of this project's own earlier claims turned out to be false.

Three real defects were fixed. Every one of them produced **plausible
wrong output**: a fabricated identifier that looked like a real one, a
policy silently dropped on one of two evaluation surfaces, and a report
that was correct for one edition of ICC.1 and wrong for the other. None
of the three announced itself, which is rule 1 of this project restated
as a release note.

### Added

- **★★★ K-only black preservation — a named, opt-in policy with NO
  default.** `iccce transform --preserve-black <policy>`;
  `iccce_cmm::black_preserve::KMapping`. An input carrying black alone
  produces an output carrying black alone.

  **The mandatory argument is the design.** There is deliberately no bare
  `--preserve-black`, because **two published definitions of "preserve
  the black" disagree**, by up to `4.889900e-2` in device units on a
  cross-press pair. A default would be this library choosing one and
  reporting it under a name that means both.

  | policy | status |
  |---|---|
  | `k-only-equal-lightness` | **Implemented.** lcms2's construction — a **vendor** choice with no normative text behind it, and **also this project's own oracle**, which is stated because it means a differential test here measures agreement rather than truth |
  | `k-only-ratio` | ★★ **Refused by name, and the refusal is sourced.** Cholewo (2000), IS&T/SID CIC8 pp. 257–261 — genuinely published, and therefore the *stronger* provenance of the two |

  ★★★ **Why the published definition is the one iccce refuses**, since
  that reads backwards and the reasoning is the most characteristic
  decision in the release. Cholewo §2.4 determines `K_MIN`/`K_MAX` **per
  colour**, by inverting a fitted, continuous, differentiable printer
  model under constraints — four model inversions per colour plus a fifth
  optimisation for the answer. An ICC profile's LUT is not that object;
  six weights are named and three are unspecified, so two faithful
  implementations of the paper will not agree with each other; and the
  paper's own construction is an **offline device-link procedure**, not a
  runtime transform. **An implementation that hoists those limits out of
  the per-colour loop is not implementing the method.**

  ★ And the sharpest reason, which is about testing rather than about
  Cholewo: **on a same-press pair, Cholewo's Eq. (1) reduces exactly to
  `K_d = K_i`.** So *"copy K through"* is not the naive rival — it is
  Cholewo's own answer there — and **a wrong approximation of the method
  and a right one are indistinguishable by construction on the fixture
  this project exercises most.** Approximating it would have been
  indistinguishable from implementing it. `ChainError::KMappingNotAvailable`
  names the refusal instead.

  ICC.1 specifies **no black-preservation construct at all**, in either
  edition searched (`ICC_Spec` register `A51`, a closed negative). So no
  published value for what preservation *should* return can exist, and
  every number this release quotes for it is a self-comparison or a
  cross-check. **See "Measured" for what it costs.**

- **Both readings of sRGB's transfer function, with the default
  unchanged.** `iccce_cmm::builtin::SrgbTrc`, selectable through
  `Destination::BuiltinSrgb(SrgbTrc)`. `Destination::None` is **exactly**
  `BuiltinSrgb(ValueContinuous)` and is asserted bit-identical to it.

  The two readings are both in force. The **C⁰** reading (`α = 1.055`
  exactly, encoded breakpoint `0.04045`) is what ICC's own sRGB document,
  W3C CSS Color 4 and Khronos KDF print. The **C¹** reading
  (`α = 1.055 010 718 947 586 4`, breakpoint `0.039 293 370 676 847 5`)
  is what `Rec. ITU-T H.273 (V4) | ISO/IEC 23091-2` clause 8.2
  **requires** for `TransferCharacteristics = 13` — which is what AVIF,
  HEIF, AV1 and the ISOBMFF `nclx` box point at. **IEC 61966-2-1's
  normative text would settle which is sRGB; nobody on this project has
  read it** (paywalled, unobtained).

  ★★ **The option's guard has a LOWER bound as well as an upper one**,
  which is unusual here and is the point: the upper bound catches the
  option becoming perceptible, and the lower bound (`> 1×10⁻⁹`) catches
  it becoming **decorative** — a regression collapsing the two variants
  into one would separate by exactly zero and sail through a
  ceiling-only assertion. Both failures are silent and the second is the
  more likely.

- **`violations: N`, printed beside `malformations: N` by
  `iccce inspect`.** `malformations` counts **disclosures**; `violations`
  counts the subset that breaches a stated requirement. Both are printed
  unconditionally, including when they are `0` and when they are equal —
  a disclosure that appears only when it is interesting teaches a reader
  to read its absence as *"nothing to see"*.

  ★★★ **This is filed as an addition, but its occasion was a false
  accusation made by this project against five ICC-published profiles.**
  The distinction *"a count of disclosures is not a conformance verdict"*
  existed in three doc comments and **nowhere a program could read it**,
  so a graded conformance row consumed `.len()` and went red against
  `sRGB2014.icc`, `ITU-RBT709ReferenceDisplay.icc`,
  `PSOsc-b_paper_v3_FOGRA54.icc`, `PSOuncoated_v3_FOGRA52.icc` and
  `SC_paper_eci.icc` — each disclosing exactly one `HeaderReservedNonZero`
  because they are **v2** files carrying an MD5 where v4 later put
  `profileID`. The row offered two hypotheses, *"either iccce
  over-reports or a published ICC profile is defective"*, and **the truth
  was neither**: the engine was right, the files are conformant, and the
  row was counting the wrong quantity. The lesson is not about ICC
  profiles: **a corrected doc comment did not reach the code that needed
  it.** Nothing is repaired, hidden or reclassified — rule 6 holds, every
  malformation line is still emitted verbatim, and this adds a second
  count over the same unaltered set.

- **A ground-truth measurement for chromatic adaptation** — the error
  class this project's own plan names as the canonical from-memory
  mistake, and the first time anything here has been graded against a
  **published** adaptation matrix. iccce's Bradford-derived D65→D50
  matrix against the nine cells ICC prints at fifteen decimal places
  (Holm, *"How to interpret the sRGB color space … for ICC profiles"*,
  2015, §B.2). Nineteen rows, and **none of them can skip** — no oracle,
  no profile, no fixture, no environment variable.

  ★★ **Every bound was derived in exact arithmetic before the pass was
  first run**, and the derivation the pass was commissioned with was
  **incomplete**: it named the cone matrix, which the rows are *about*,
  and omitted the white point, which they merely *use*. The omitted term
  is `7.9×` the one that was named. Written down first, it would have
  failed at `7.4×` its own bound. See "Measured".

  ★ Two by-products worth a consumer's attention. **ICC's published
  matrix was computed with Bradford `M_A[0][0] = 0,8950`, while ICC.1:2022
  Annex E.3 Eq. (E.1) prints `0,8951`** — recovered by eigendecomposition
  and confirmed by exact reconstruction (`5,7×10⁻¹⁶` against
  `5,661×10⁻⁶`). iccce follows the printed specification. **There is no
  clause under which one of two ICC publications is the wrong one**, so
  that row is reported, never graded. And **sub-ULP does not mean
  identical bytes**: at `2,730` ULP, **6 of 9 cells encode to a different
  `s15Fixed16` word**, so if iccce ever writes a `chad` for its built-in
  sRGB it will not byte-match ICC's, and no clause requires it to.

- **`docs/FEATURES.md` — a consumer-facing capability list where every
  row carries its evidence class.** `README.md` §Status answers *"how far
  along is the plan?"*; this answers *"can I call this from my program
  tomorrow?"*, and it is written for the consumer named first in the
  README.

- **`tools/patch-compare/` — a generic, MIT tool for comparing
  `iccce transform` output against pixel regions of a reference render.**
  Worth a line for consumers because it is deliberately **empty of
  subject matter**: it contains no colour value, no pixel coordinate and
  no document name, and reads all of that from a **case file** supplied
  on the command line and kept outside this repository.

  Three properties are enforced by the program rather than documented in
  prose: a case file **must** declare `reference.evidence_class` (there
  is no default, because a defaulted evidence class is exactly how
  *"iccce matches X"* becomes a claim nobody chose to make); the declared
  class and provenance string are printed **above every table**; and
  **no ΔE is ever printed** — output is 8-bit channel counts only, with
  the standing reason in the module header.

### Fixed

- **★★★ A v2 profile has no `profileID`, and iccce was fabricating one.**
  A v2 file carrying arbitrary bytes in its reserved space printed
  `header.id: deadbeefdeadbeefdeadbeefdeadbeef` beside
  `malformations: 0`. ICC.1:2001-04 Table 9 makes bytes **`84..127` one
  44-byte reserved block** — the only mention of it in that document —
  and the `profileID` field does not exist before v4.

  Two defects, one cause. iccce checked **`100..128`, 28 bytes, on every
  profile of every edition**, so **16 bytes of a v2 profile's reserved
  block were checked by nothing while those same 16 bytes were being
  reported as an identifier**. The check is now edition-selected, the
  report names the range it actually checked, and `iccce inspect` prints
  `header.id: n/a (no profileID field before v4; bytes 84..128 are
  reserved)`. ★ **The bytes are still disclosed. What stopped is the
  mislabelling** — and a fabricated identifier is worse than a false
  accusation, because it is in the shape a reader verifies against.

- **★★★ `convert_buffer` silently dropped black preservation.** A caller
  who asked for the policy received it **one pixel at a time** and
  **silently did not receive it for a buffer**. Measured at the fix:
  **`7.195269e-1` of ink** at `[0, 0, 0, 1]` on
  `v2-cmyk-chromatic-neutral` at grid 17, with the non-qualifying control
  at **exactly `0.0`**. `convert_buffer` now delegates to `convert`
  rather than reproducing its body, and the guarantee is asserted by
  **bit-equality** in
  `crates/iccce-cmm/tests/compiled_buffer_agrees_with_single_pixel.rs`.

  ★★★ **The transferable half is why it survived, not what it was.** The
  only in-repository caller of `convert_buffer` is `iccce bench`, and
  `bench` cannot request black preservation — so the defect was
  **unreachable from the CLI, and the differential suite drives the
  CLI.** *A defect that only a **library** consumer can reach is
  invisible to a CLI-driven suite, however green it is.* That sentence is
  the reason this bullet is longer than the bug.

- **The rendering-intent report was edition-specific and iccce was
  applying one edition's rule to both.** ICC.1:2022 7.2.15 requires the
  header `renderingIntent` field's high 16 bits to be zero; **ICC.1:2001-04
  6.1.11 imposes nothing on either half**, and its high half is vendor
  space by the same construction 6.1.8 uses for the profile flags. The
  same bytes are now reported differently by edition, and **in v2 a
  high-half value is not reported at all** — reporting it would have made
  a false statement about that edition.

### Changed

- **The version number is duplicated in exactly one place now, and it is
  three lines below the number it must match.** The `0.1.0` → `0.2.0`
  bump **broke the build**: `workspace.package.version` moved and five
  inter-crate dependency literals did not, producing an error about a
  *missing crate* rather than about a stale version pin. **Two of the
  five carried a comment warning that they "must move with it in the same
  commit" — the warning existed and was insufficient**, which is the
  finding. They are hoisted into a root `[workspace.dependencies]` table
  and the members inherit with `iccce-color.workspace = true`.

  ★ Cargo cannot remove the duplication entirely, and the entry says so
  rather than implying a fix that was not made:
  `workspace.package.version` **cannot** be inherited into a dependency
  *requirement*, and the literal is **not optional** — a path dependency
  with no version builds locally and makes the crate unpublishable.

- **`docs/GHENT_COMPATIBILITY.md` is no longer tracked**, and
  `tools/patch-compare/` replaces a predecessor that was deleted the same
  day it was written. Operator decisions of 2026-08-18 and 2026-08-25:
  detailed analysis of, and quotation from, a **licensed** third-party
  test suite does not belong in an MIT repository, and the predecessor
  had inlined that suite's specifics as constants even though it took its
  paths as flags.

  ★★ **The transferable clause: the private half lives OUTSIDE the
  repository, not in a gitignored path inside it** — a gitignored path is
  one `git add -f` away from the entanglement the decision was made to
  prevent. `.gitignore` now records that reasoning where a future author
  will meet it.

- **The licence manifest's generator gained its fifth crate**, and the
  failure it had was **silent**. `iccce-measure` joined the workspace on
  2026-08-12 and `about.toml` was written the same day with four
  `clarify` blocks. A workspace member with no block does not error —
  `cargo-about` falls back to the **generic SPDX MIT template**, whose
  copyright line reads *"Copyright (c) &lt;year&gt; &lt;copyright
  holders&gt;"* — so a regenerated manifest would have shipped an
  unfilled-looking licence for a crate that is ours. The invariant is now
  written down: **one `clarify` block per `members` entry**, with the
  check that verifies it.

- **The `0.1.0` entry's `0.90 ULP` footnote is the work that ships here.**
  That number was annotated after tagging as a partial cancellation of
  two ≈2.5 ULP terms; the measurement, the decomposition and `NA-010`'s
  registration are commits in this release.

- **`iccce inspect` and the CLI's usage text now state, in the product,
  several distinctions that previously lived only in doc comments** — the
  disclosure/violation split above, the mandatory preservation policy
  with both definitions named, and the built-in sRGB's status as a
  **destination and not a working space** (ISO 32000-2:2020 §11.7.2
  NOTE 4: a non-linear CIE-based space *"can be unsuitable for use as a
  group colour space"*). ★ That last one is **disclosed, not guarded** —
  iccce cannot see what a caller does with the model it returns.

### Measured

Numbers a reader may quote. **Each carries its evidence class, and the
class is part of the number.** Everything with a ΔE or a `pass=` in it
was produced on **one machine, one toolchain, one day** unless the row
says otherwise.

| claim | value | class |
|---|---|---|
| Bradford D65→D50 vs **ICC's published** matrix, worst of nine cells | **`4,164 937×10⁻⁵`** = **`2,730` ULP** of `s15Fixed16` — **exactly at an independently derived prediction** | `published-ground-truth` (a `tools/difftest` row) |
| **the perceptual cost of K-only preservation** — the preserved answer against **iccce's own colorimetric answer**, one cross-press CMYK pair, media-relative, no BPC, 101-point qualifying set | **`3.681203` ΔE2000 max** (at `K = 1.00`), **`1.580674` mean**, median ≈ `1.61` | **self-comparison** |
| the same, **reversed pair** | **`2.432964` ΔE2000 max** — the price is a property of the **destination's** black, not of the pair | self-comparison |
| the same over **all 30 ordered pairs** of six real CMYK press profiles | **11 pairs are entitled to price the policy and `0` of them find it imperceptible**, cost `2.023698` … `3.686985`. ★ **Without the entitlement gates, 9 of the 30 would have read as "nearly free"** | self-comparison, graded at **0 counterexamples** |
| ★★ **the discontinuity the policy installs** | **`3.712251` ΔE2000** — **larger than the cost itself**, one 8-bit code of cyan outside the qualifying set | self-comparison |
| K-only leak: chromatic ink on inputs that do **not** qualify | **exactly `0.000000`** on all ten CMYK destinations of the licensed corpus, at a `5×10⁻⁷` observation floor | self-comparison |
| ★ iccce implements **the definition it names**, at the oracle's own 17 CLUT nodes, cross-press | **`3.100000e-5`** against a run-time bound of `1.090000e-4`; **the rival definition sits `4.889900e-2` away** | implementation-cross-check |
| **which reading of sRGB lcms2 implements** — 275 neutral probes, `transicc`, lcms2 pinned at `21c582a` | residual against **C⁰ `5.300706×10⁻⁵` `L*`** (below one printed quantum) vs **C¹ `1.230354×10⁻³`** — a ratio of **`23.2×`**; and **`0` of `204`** separable probes sit closer to C¹ | implementation-cross-check |
| **what choosing between the two sRGB readings costs**, in the PCS | **`1.857 907×10⁻³` ΔE2000** max — **538× below** the perceptibility anchor | **self-comparison** |
| …and what it still moves | **`14`**, **`11`** and **`2`** of **`5 169`** probes change an **8-bit output code** end to end, on three destinations | **self-comparison** |
| `convert_buffer` vs single-pixel, at the fix | **`7.195269e-1`** of ink at `[0,0,0,1]`, grid 17; control **exactly `0.0`**. Now **bit-equal** | self-comparison |
| workspace tests, run locally 2026-08-25 | **`205` passed, `0` failed** | apparatus census, `[REPORTED]` |
| dependency-tree purity, `cargo tree --workspace`, 2026-08-25 | **zero** non-`iccce-` lines — the whole tree is five packages and all of them are ours | dated observation |

★★★ **One measurement in this release is `rendering-cross-check-screen-capture`
— a new and DELIBERATELY WEAK class**, added because a comparison partner
weaker than lcms2 had to be named rather than borrowed. It compares
`iccce transform` output against a screen capture of another
application's render of a licensed test document. **No ΔE may be computed
against it**, by decision and in code: a ΔE against a screen grab dresses
a rendering comparison up as colorimetry, and an out-of-gamut value
clamped at a channel endpoint would make the ΔE measure the **clamp**
rather than the colour. Its most useful row is the mechanism one — a
seven-point sweep in which blue stays at **exactly `0.000000`** and red
saturates at **exactly `1.000000`**, which is the signature of a gamut
clamp (ICC.1:2022 Annex F.8–F.16, applied before the inverse TRC) and not
of a converged value. ★ **Those rows are protected by nothing**: no test
fails if the transform drifts, and no `pass=` count moves.

### ★★★ Two of this project's own claims were FALSIFIED, and by us

A release note that reported the sRGB-variant feature and omitted this
would be the wrong shape for this project.

`SrgbTrc`'s doc comment shipped with two statements. Both were **correct
and narrowly scoped**, and both were **written as though they were
general**.

| the claim as first written | what measurement returned |
|---|---|
| *"no image changes, ever"* | ★ **FALSE end to end** — 8-bit output codes do change. **True** of sRGB's own encoding: `0` of 256 codes change, and the two curves are `9.76×10⁻⁶` apart at most over a 200 001-point sweep |
| *"below one 16-bit PCS quantum"* | ★ **FALSE as ΔE.** **True** in the encoded domain. In the PCS the maximum is `1.857 907×10⁻³` ΔE2000, which is **`2.01×`** one 16-bit `L*` quantum evaluated **at that same point** |

★★★ **The general form, which is why this is in the changelog and not in
a commit message:** *"below one quantum"* and *"no code changes"* are
**claims in a UNIT**, and a claim in a unit is silently scoped to the
domain that unit lives in. **Neither statement was ever wrong where it
was measured.** Both became wrong by being **read one transform later**,
and nothing in either sentence carried the domain it was true in. The
corrections were made **in place, with the original wording preserved
beside them**, because the author made them and then repeated them.

★ **A second self-correction, smaller and the same shape.** The built-in
sRGB's doc comment said its colorant residual was *"entirely accounted
for by which D65 matrix each side starts from"*. Exact decomposition
says the word *"entirely"* is false: there are **two** terms of the same
size (`2,482` and `2,480` ULP), and on the one cell that argument was
about, the flatteringly small total is a **cancellation between two
errors five times its size**. The `3.02` / `11.13` comparison it framed
is unaffected; the **attribution** was wrong.

★★ **And once, the disagreement ran the other way** (rule 7). On a
same-profile pair the equal-lightness construction **is the identity** by
algebra. iccce returns it **exactly**; lcms2's intent 11 sits
**`6.100000e-5`** away. This is the one row in the black-preservation
work whose expectation is not some implementation's output.

### ★ What this release does NOT claim

The section this project cares most about. Every line below is a
statement someone would otherwise reasonably assume.

- **★★★ Not that any CI run has been observed.** No agent or human on
  this project has observed a CI run of this repository, across
  twenty-five-plus consecutive filings. **A workflow file is a
  configuration, not a run.** Every gate quoted above — tests, `fmt`,
  `clippy`, the `wasm32-unknown-unknown` build, the dependency-tree
  check — was run **locally, on one machine, on 2026-08-25**, and is
  `[REPORTED]` on that basis.
- **★★★ Not that a `published-ground-truth` row exists for any
  transform.** It does not, and the debt is unmoved at its fourteenth-plus
  consecutive filing. **IEC 61966-2-1 remains paywalled and unobtained.**
  ★ The ground-truth rows this project *does* hold are about **values,
  never about a transform path**, and they live in **three separate
  populations with three separate owners** — the ΔE2000 dataset row, the
  sRGB colorant row, and `tools/difftest`'s own rows including this
  release's adaptation matrix. **A count from one is not a count of
  another.** ★★ And for the LUT path the gap is **structural**, not
  unfinished: ICC.1 mandates no interpolation method, so no expected
  value could be published even in principle.
- **★★ Not that Pass L settles what sRGB is.** It establishes **a fact
  about an implementation** — lcms2 implements the C⁰ reading — on **one
  build, one pin, one profile, one intent, one direction, neutral probes
  only**. The two standards remain in disagreement and that register
  entry stays **open**. Anyone quoting a Pass L number as evidence about
  **sRGB** is quoting it wrongly. ★ Had lcms2 come out C¹ it would have
  been a **finding**, not a failure, and the blackbox measurement was
  designed to be able to contradict the source read that preceded it.
- **★★ Not that the black-preservation numbers are accuracy.** Their
  oracle is **iccce itself**, structurally: ICC.1 describes no such
  construct, so no published value for what preservation *should* return
  can exist. The measurement establishes what the policy **costs**, never
  that the policy is **right**. ★ And the headline row is **REPORTED, not
  graded** — no requirement bounds what an opt-in policy may cost, so
  that row **can never go red**; what defends it is two neighbouring rows
  failing under injection.
- **★ Not that the sRGB-variant cost is reproducible from the CLI.** No
  shipped binary can select the C¹ reading; the variant is reachable
  only through the library API. Every cost number above was produced in
  process.
- **Not that iccce is more accurate than any other CMM.** Its oracle for
  LUT paths is still lcms2 — a cross-check against another
  implementation, not ground truth.
- **★★ Not a claim of compatibility with, or a pass on, any third-party
  test suite.** One suite is used **privately** to check behaviour;
  publishing a pass/fail claim, a screenshot, or the suite's name in
  release material requires the rights holder's **written permission**
  and is an operator decision that has not been taken. ★ Separately, an
  operator decision on **whether published numbers may live in an MIT
  repository as fixtures at all** — covering ICC's informative-annex
  values, CIE's CC BY-SA tables and one vendor's self-contradicting
  copyright string together — **is still open**, and the second
  ground-truth fixture is blocked on it.
- **★★★ Nothing is published to crates.io, and this release does not
  publish anything there.** The release workflow builds the CLI from the
  tagged commit and attaches binary assets to a GitHub release; **it
  contains no publish step** *(verified by reading it)*. `cargo package`
  succeeds for `iccce-color`, `iccce-profile` and `iccce-measure` and
  fails for `iccce-cmm` and `iccce-cli` with *"no matching package named
  `iccce-color` found"* — ★ **that failure is the ordinary publish-order
  chicken-and-egg, pre-existing, and reproduces identically at `0.1.0`**;
  it is not a regression from the manifest change. ★★ Any check that a
  crate name is unregistered is a **dated observation that reserves
  nothing and authorises nothing**; re-check immediately before any
  publish, which is an operator's act (rule 9).
- **Not that the API is stable.** `0.x`, and the entry above changes
  behaviour a caller can observe.
- **Not certified against any conformance suite.** Certification requires
  measurement hardware and is organisationally closed to a library.

### Known limitations

- **The evaluation surface is `f64` throughout.** No `f32`, no integer
  path. A 300 DPI A4 CMYK raster is **268 MB in / 201 MB out**. A
  narrower surface **solves the memory** (→ 67 MB / 25 MB) and **does
  nothing for the time**, because the throughput is grid evaluation and
  is per-pixel however the pixel arrived. Known, requested by a real
  consumer, and deliberately deferred under *optimise only after correct*.
- **`--preserve-black` has no default and will not acquire one** while
  the two published definitions disagree. `k-only-ratio` is refused by
  name, not planned.
- **The C¹ sRGB reading is not reachable from the CLI.**
- **The cost of black preservation is unmeasured at any other intent,
  with `--bpc`, or off the `C = M = Y = 0` axis** — and **any weighting
  by how often a real document contains K-only content** belongs to the
  consumer, not to this library. That last term is the one that decides
  whether the policy is worth its price.
- `multiProcessingElements` (`mpet`) is not implemented — declined per
  8.10.2 b), and disclosed.
- iccMAX (ICC.2) is refused by design, by name.
- Above four input channels there is **no ΔE claim of any kind** for the
  compiled path.
- **Overprint is not iccce's** — it is *compositing, not conversion*, and
  it happens in a device space before any conversion. iccce supplies the
  conversion at the end of that pipeline and owns none of the
  compositing in front of it.

---

## [0.1.0] — 2026-08-17

The first tagged release. Passes 0–7 were complete before it; what this
version adds is the surface a real consumer needs, and — more usefully —
the measurements that say how far to trust it.

### Added

- **A built-in sRGB destination, constructed from published constants.**
  `iccce_cmm::builtin::srgb()`. No file, no embedded profile blob, no
  I/O, no dependency. Primaries and white point from **ITU-R BT.709-6**
  items 1.3/1.4, transfer function and both breakpoints from **W3C CSS
  Color 4**, chromatic adaptation from **ICC.1:2022 Annex E.3**.

  Used when a caller states it has no destination profile, via the new
  `Chain::with_destination`.

- **`Destination` — a two-variant enum, deliberately not
  `Option<&Profile>`.** `Profile(&p)` or `None`. The distinction is the
  whole safety story: *"doesn't exist"* must mean **absent**, never
  **unresolved**. If a document declares an output intent whose profile
  fails to parse, and that error is flattened into a `None` on the way
  in, a silent substitution would render a plausible picture to the wrong
  destination with no error anywhere. An `Option` being `None` cannot
  distinguish "there was none" from "I failed to get one"; only the
  caller can, so the type makes the caller say.

- **`DestinationProvenance` — the fallback is disclosed, not silent.**
  `Chain::destination_provenance()` reports whether the destination was
  caller-supplied or constructed, with ready-to-log wording. Rule 6's
  CMM analogue: a silent substitution of the destination is the
  transform-layer version of a silently corrected tag.

- **`SpecDeviation` — the 8.10.2 tags iccce declines are now named.**
  `Chain::spec_deviations()` and `transform::mpet_deviation_for()`.
  ICC.1:2022 8.10.2 a) prefers `D2Bx`/`B2Dx` multiProcessingElements;
  iccce does not implement them and takes step b), which the clause's own
  exception permits. lcms2 does implement them. **Both engines are
  conformant and they differ by 33.13 L\*** on ICC's own
  `Probev2_ICCv4.icc`. The clause permits declining; it does not require
  silence.

- **A colour-space signature → component count accessor.**
  `iccce_profile::components()`, `channel_agreement()`, `is_valid_pcs()`.
  Answers *how many components does this profile declare* **from the
  header, before any chain is built** — which is what a PDF engine needs
  to validate an `/ICCBased` stream's `/N`.

  Sourced from ICC.1:2022 clause 7.2.6 Table 19 joined with Table 41,
  transcribed twice with two independent PDF engines and matched against
  ICC's own `icProfileHeader.h` and lcms2's `lcms2.h`.

- **`std::error::Error` for `ChainError`, `ModelError` and
  `CurveError`**, with a `source()` chain. Without it a consumer could
  not `?` a chain build into `Box<dyn Error>` — the most common error
  shape in Rust — and had to hand-wrap or `.ok()` the result and **lose
  the named refusal entirely**.

- **`iccce inspect` now reports** the declared component count, an
  unrecognised colour-space signature, a PCS field invalid for the
  device class, and any declined 8.10.2 tags.

- **CI gates wasm32 and the dependency tree.** The library crates build
  for `wasm32-unknown-unknown` on every push, and every crate in
  `cargo tree` must be one of ours. Both matter to a downstream consumer
  that enforces a wasm gate and classifies every dependency.

### Fixed

- **★ A seven-channel profile aborted the process.** `iccce bench` on a
  real `7CLR` press profile died with `0xC0000409` — *"memory allocation
  of 1022842631448 bytes failed"* — because a grid constant justified for
  3-D and 4-D was applied to every higher dimension by a `_ => 33`
  catch-all (`33⁷` ≈ 952 GiB), and the guard used `checked_pow`, which
  catches **wrap** and not **size**.

  An abort is the worst available failure for a library: not an `Err`,
  not a catchable panic, it takes the consumer's process down. Now a
  named refusal (`ChainError::GridExceedsBudget`), and the ≥5-channel
  recommendations are computed rather than tabulated.

- **A conformant `A2B`-only profile was wrongly refused**, with an error
  naming the wrong clause. The built-in-destination path obtained its
  source model by building a scaffold `src → src` chain and discarding
  the destination half; for a one-directional profile that half failed
  and surfaced *"matrix/TRC model requires PCSXYZ"* — true,
  clause-cited, and about a model iccce was about to throw away. **A
  refusal that names the wrong clause is worse than a vague one, because
  the citation makes it persuasive.**

### Changed

- **The sRGB rule-4 approximation was mis-attributed, and is corrected.**
  ICC's own *"Specification of sRGB"* (Holm, 2015) publishes the
  D50-adapted colorants at 15 decimal places — values this project had
  recorded as published by no document. Measured against them:

  | | worst cell | `bXYZ.Z` |
  |---|---|---|
  | **this construction** | **3.02 ULP** | 0.90 ULP † |
  | the shipped HP 1998 / `sRGB2014.icc` file | **11.13 ULP** | **11.13 ULP** |

  The long-standing ~12 ULP blue-Z residual is **the file's error**, not
  iccce's. The gap in the literature had been silently treated as
  evidence that the file *was* the reference.

  > **† Corrected after tagging — compare worst cells, not `bXYZ.Z`.**
  > Both numbers are right; the juxtaposition is not. Measured by
  > substituting one side at a time, this construction's `0.90 ULP` in
  > blue-Z is **a partial cancellation of two terms of ≈2.5 ULP**
  > (`−2.483` from our chromatic adaptation, `+1.585` from our D65
  > matrix) — not a small error. **The defensible figure is the worst
  > cell, `3.02 ULP`**, which is what the test bounds and what the
  > release notes quote.
  >
  > Left in place rather than deleted: removing a number is worse than
  > annotating it, and *a figure that looks like accuracy and is actually
  > a coincidence of signs* is exactly the failure this project exists to
  > catch. It survived a day because it pointed the flattering way.
  > Registered as **NA-010**.

### Measured

Numbers a reader may quote, each with its evidence class:

| claim | value | class |
|---|---|---|
| constructed sRGB vs ICC's **published** colorants | **3.02 ULP** worst (bound 4) | `published-ground-truth` |
| constructed sRGB vs `sRGB2014.icc`, 10 probes | **0.033 ΔE2000** max; black exact | constructed-vs-reference-file |
| real-world profiles parsed, two corpora | **50 accepted, `malformations: 0`** | acceptance population |
| iccMAX refused **by name**, exit 1 | **10 of 10** | refusal population |
| 7-channel `mAB` PCS corners vs lcms2 | **4.9×10⁻⁵ L\*** | cross-check |
| conformance suite, operator machine | **`pass=274 fail=0 skip=9`** | mixed; see `TOLERANCES.md` |

### ★ What this release does NOT claim

- **Not that iccce is more accurate than any other CMM.** Its oracle for
  LUT paths is lcms2 — a cross-check against another implementation, not
  ground truth. **ICC.1 mandates no interpolation method, so no published
  ground truth for a LUT path can exist even in principle**, and ICC's
  own reference implementation ships zero expected colour values.
- **Not that the CI badge evidences the conformance suite.** On a
  corpus-free runner the suite executes **15 of 201 rows** — the rest
  need a Windows system profile or private corpora no runner holds. The
  numbers that matter come from the operator's machine. A coverage floor
  now makes that visible rather than silent.
- **Not that the API is stable.** `0.x`. Several types here were added
  the same day this was tagged, and no external consumer has used them
  yet.
- **Not certified against any conformance suite.** Certification requires
  measurement hardware and is organisationally closed to a library.

### Known limitations

- **The evaluation surface is `f64` throughout.** No `f32`, no integer
  path. An 8 Mpix CMYK page is **256 MB in / 192 MB out**. This is the
  most consequential open item and it was found by a real consumer, not
  by internal review.
- `multiProcessingElements` (`mpet`) is not implemented — declined per
  8.10.2 b), and now disclosed.
- iccMAX (ICC.2) is refused by design, by name.
- Above four input channels there is **no ΔE claim of any kind** for the
  compiled path.

[0.2.0]: https://github.com/KenM76/iccce/releases/tag/v0.2.0
[0.1.0]: https://github.com/KenM76/iccce/releases/tag/v0.1.0
