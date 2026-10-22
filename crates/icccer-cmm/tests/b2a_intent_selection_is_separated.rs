//! # Intent → tag selection on the DESTINATION side, on a committed fixture
//!
//! ## What this exists to catch, and why nothing in the tree could
//!
//! [`Chain::new`] implements clause 8.10.2's selection twice: once for
//! the source (`A2Bx` → `A2B0` → matrix/TRC) and once for the
//! destination (`B2Ax` → `B2A0` → matrix/TRC). Before this file, the
//! **destination** half had no evidence anywhere in the repository that
//! did not depend on a machine-local profile:
//!
//! - `docs/NEXT_SESSION.md` records that **no synthetic fixture carried
//!   `A2B2` or `B2A2` at all**, and that the only two committed
//!   fixtures carrying `B2A1` push **byte-identical payloads** into
//!   `B2A0` and `B2A1` — deliberately, and for a stated, sound reason.
//! - The one test that did exercise the saturation intent through a
//!   real LUT destination reads
//!   `C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc`, and
//!   **returns success when that file is absent.**
//!
//! ★★★ **A skipped test and a passing test print the same colour.** On
//! any machine without the Windows colour directory — every CI runner
//! this project has — the suite was green and the claim was unmeasured.
//! That is the state `pdfce` would have inherited had it built its own
//! saturation-vs-media-relative regression on the same file, which is
//! what its 2026-09-01 request asked about.
//!
//! ## The fixture, and the one property everything here rests on
//!
//! `fixtures/synthetic/v2-cmyk-intent-separated-b2a.icc`, recipe
//! `v2-cmyk-intent-separated-b2a` in `tools/gen-profiles/src/recipes.rs`
//! §(c4). It carries **six** `mft2` tags at **six distinct offsets** —
//! `A2B0`/`A2B1`/`A2B2` and `B2A0`/`B2A1`/`B2A2` — no clause-7.3.1
//! aliasing anywhere.
//!
//! The three `B2Ax` are one another's twins with **one scalar changed**:
//! a per-intent **black floor** added to the `K` channel of every node,
//!
//! ```text
//! IS_K_FLOOR = [0,00,  0,20,  0,40]      for B2A0, B2A1, B2A2
//! ```
//!
//! with `C`, `M` and `Y` held **identical** across all three.
//!
//! ★★★ **A constant added to every CLUT node survives every
//! interpolation scheme exactly.** Multilinear, tetrahedral, prism,
//! nearest — each is an affine combination of the enclosing cell's
//! corners with weights summing to one, so a constant present at every
//! corner appears at the interpolated point undiminished. And the
//! offsets are **exact 16-bit codes** (`65 535 = 3 × 5 × 17 × 257`, so
//! `0,20 × 65 535 = 13 107` and `0,40 × 65 535 = 26 214` are integers,
//! and `round(v + n) = round(v) + n` for integral `n`), so the
//! quantisation of the node values cancels rather than being bounded.
//!
//! **Consequence, and it is the reason this file can assert equalities
//! rather than hope for inequalities:** the `K` separation between two
//! intents is the constant itself — at every PCS point, for every
//! probe, under any conformant CMM, independent of the source profile
//! and of the interpolation geometry.
//!
//! ## ★★★ EVIDENCE CLASS — read this before quoting any number below
//!
//! **`derived-expectation`, and about STRUCTURE only.** Every expected
//! value here is arithmetic on the fixture's own authored constants plus
//! clause 8.10.2's stated selection order. **No implementation's output
//! is in any expectation** — not lcms2's, not icccer's. That is stronger
//! than a cross-check (which is defeated when two implementations share
//! a misreading) and weaker than ground truth (which this is not, and
//! must never be called).
//!
//! ★★★ **AND IT IS NOT COLORIMETRY.** A saturation table that differs
//! from a media-relative table by a flat 20 % black floor is **not** what
//! a saturation intent does. ICC.1 says nothing that would make it so, no
//! instrument produced these numbers, and the profile describes no press
//! that exists. What is asserted is exactly one thing: **the byte block
//! the CMM read for the saturation intent is the one clause 8.10.2
//! names, and not another one.** Anybody citing `0,20` as a fact about
//! rendering intents, black generation, or ink has misread this file.
//!
//! ## What the arms SHARE — `DL-077`, applied to this file before trusting it
//!
//! A differential tests only what differs between its arms. Here the
//! arms share the source profile, the probe, the PCS connection, the
//! interpolation code, and every constant of the destination's
//! separation except one. So the `K` difference has exactly one
//! available cause. Two independent guards keep that true:
//!
//! 1. **[`cmy_is_bit_identical_across_the_three_intents`]** — if any
//!    future default (BPC, K-only preservation, a gamut-mapping step)
//!    ever begins firing on an intent basis, it disturbs `C M Y` and
//!    *that* test goes red, instead of silently supplying the `K`
//!    difference the headline test reads as proof of tag selection.
//! 2. **[`the_sibling_fixture_is_the_zero_separation_control`]** — the
//!    named rival hypothesis (*the CMM ignored the intent and read
//!    `B2A0` every time*) is not merely argued against, it is **run**,
//!    on `v2-cmyk-chromatic-neutral.icc`, which carries only `B2A0`. Its
//!    measured separation is `0.0` exactly. The headline row's candidate
//!    separation is therefore `0,20 − 0,0 = 0,20`, measured in the same
//!    run rather than asserted.
//!
//! ★★ The standing lesson this file was built against, from Pass K §F:
//! **a zero-separation fixture does not merely fail to inform, it
//! MANUFACTURES A FALSE PASS.** Which is why the control above exists as
//! a test rather than as a paragraph.
//!
//! ## Coverage, stated
//!
//! One source profile (`v2-rgb-matrix-trc-curv.icc`, matrix/TRC, XYZ
//! PCS), one destination (`v2-cmyk-intent-separated-b2a.icc`, `mft2`
//! `lut16`, Lab PCS, 9³ grid), **125** RGB probes, three intents plus
//! ICC-absolute, `Chain::new` only — **not** the compiled path, **not**
//! `mAB`/`mBA` v4 element pipelines, **not** any real profile. It says
//! nothing about whether icccer's *colour* is right, and nothing about
//! what any press would do.

use icccer_cmm::matrix_trc::Intent;
use icccer_cmm::transform::Chain;
use icccer_profile::Profile;

// ---------------------------------------------------------------------------
// The authored constants, restated — and why they are restated rather
// than imported
// ---------------------------------------------------------------------------
//
// `tools/gen-profiles` is a separate, out-of-tree crate that the shipping
// crates deliberately do not depend on (it must never become a dependency
// of anything that ships). So the two numbers below are transcribed, not
// linked. The recipe's doc comment names this file and says so, and the
// `covers`/`expect` strings in `MANIFEST.md` carry the same pair, so a
// change to one without the other is visible in three places.

/// `IS_K_FLOOR[2] − IS_K_FLOOR[1]` — the `K` separation between the
/// **saturation** and **media-relative** answers at this destination.
///
/// **The headline number of this file.** Exact, at every probe, for the
/// reason the module header gives.
const K_SEPARATION_SAT_MINUS_REL: f64 = 0.20;

/// `IS_K_FLOOR[1] − IS_K_FLOOR[0]` — media-relative minus perceptual.
/// The same constant by construction; asserted separately because a CMM
/// that got *one* of the two selections right and the other wrong would
/// otherwise pass on the sum.
const K_SEPARATION_REL_MINUS_PER: f64 = 0.20;

/// The tolerance on those equalities.
///
/// ★ **Derived, not tuned, and it is not a colour tolerance at all.**
/// The graded quantity is the difference of two affine interpolations
/// that use *identical* weights over nodes differing by exactly `13 107`
/// integer codes. The only departure available is IEEE-754 double
/// rounding: each node value is formed as `code / 65 535` (≤ ½ ulp of a
/// number below 1, ≈ 1,1 × 10⁻¹⁶), and at most a few dozen such terms are
/// combined by the interpolator and the 2-entry output curve. A generous
/// bound on the accumulated error is therefore of order 10⁻¹⁵.
///
/// `1 × 10⁻¹²` is **three orders of headroom** over that, and still
/// **eleven orders below the quantity being measured**. It is a
/// floating-point bound, not a perceptual one; `docs/TOLERANCES.md` §1's
/// perceptibility anchor has nothing to do with it and must not be cited
/// for it.
const EXACT: f64 = 1e-12;

/// The **stated minimum separation** the headline test additionally
/// requires, independent of the equality above.
///
/// ★★ **Why a floor as well as an equality, and the injection that
/// proves the floor is not redundant.** The equality alone would still
/// pass if somebody re-authored the fixture with a *smaller* offset and
/// updated the constant to match — the suite stays green while the
/// instrument quietly loses its power. Run as injection **I6b** on
/// 2026-09-01: `IS_K_FLOOR` reduced to `[0, 655/65 535, 1310/65 535]`
/// with `K_SEPARATION_*` updated in step. The equality **passed** (the
/// offsets are still exact codes, so the exactness argument still
/// holds); **only this floor caught it**, at a measured
/// `9,994 659 × 10⁻³`.
///
/// ★ **The number is `4 × 10⁻²`, and it is REUSED rather than
/// minted.** It is Pass K §F's declared device-separation floor
/// (`docs/TOLERANCES.md` §3.10.11, row `F3`), itself `10 ×` §3.7's
/// `SWEEP_DEVICE` — the loosest **device** bound this project has ever
/// justified. Reusing it is deliberate: the two rows floor the same kind
/// of quantity (an absolute device-space distance between two candidate
/// answers) for the same reason (an instrument must be able to tell them
/// apart), and a second constant would be a second thing to defend.
/// Derived from the tolerance budget, declared in advance, never from
/// what this fixture happens to measure. Headroom over the authored
/// `0,20`: **5×**.
const MINIMUM_USEFUL_SEPARATION: f64 = 4e-2;

// ---------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------

fn fixture(name: &str) -> Profile {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/synthetic")
        .join(name);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|e| panic!("committed fixture {} must be readable: {e}", path.display()));
    Profile::parse(&bytes).unwrap_or_else(|e| panic!("{} must parse: {e:?}", path.display()))
}

/// The destination under test: six distinct `A2Bx`/`B2Ax` tags.
///
/// ★ Note there is no `Option` and no early return anywhere in this
/// file. **A committed fixture that cannot be read is a hard failure**,
/// deliberately — the whole point of the exercise is to remove the
/// "returns success when the file is absent" escape hatch that made the
/// system-profile test silently vacuous.
fn separated() -> Profile {
    fixture("v2-cmyk-intent-separated-b2a.icc")
}

/// The zero-separation control: a CMYK profile carrying **only**
/// `B2A0`, so clause 8.10.2's fallback sends all three intents to the
/// same table.
fn zero_separation_sibling() -> Profile {
    fixture("v2-cmyk-chromatic-neutral.icc")
}

/// The source: a v2 matrix/TRC RGB profile with `curv` table TRCs.
///
/// ★ **Chosen because it is intent-BLIND.** A matrix/TRC source has no
/// `A2Bx` of any kind, so clause 8.10.2's source-side selection has
/// nothing to choose and cannot contribute a difference. Every
/// difference this file measures is therefore attributable to the
/// destination half, which is the half with no coverage.
fn rgb_source() -> Profile {
    fixture("v2-rgb-matrix-trc-curv.icc")
}

/// 125 RGB probes on a 5×5×5 grid over the unit cube, corners included.
///
/// Corners are kept rather than avoided: the fixture's `K` column never
/// clamps (`0 … 0,80` over the whole table), so the exactness argument
/// holds at the extremes too, and a probe set that quietly excluded the
/// hard points would be choosing where to be right.
fn rgb_probes() -> Vec<[f64; 3]> {
    let mut v = Vec::with_capacity(125);
    for r in 0..5 {
        for g in 0..5 {
            for b in 0..5 {
                v.push([f64::from(r) / 4.0, f64::from(g) / 4.0, f64::from(b) / 4.0]);
            }
        }
    }
    v
}

/// Convert every probe at one intent. Panics rather than skipping.
fn run(src: &Profile, dst: &Profile, intent: Intent) -> Vec<Vec<f64>> {
    let chain = Chain::new(src, dst, intent)
        .unwrap_or_else(|e| panic!("chain must build at {intent:?}: {e}"));
    rgb_probes()
        .iter()
        .map(|p| {
            chain
                .convert(p)
                .unwrap_or_else(|e| panic!("convert must succeed at {intent:?} on {p:?}: {e}"))
        })
        .collect()
}

// ---------------------------------------------------------------------------
// A0 — the precondition, on raw bytes
// ---------------------------------------------------------------------------

/// **The precondition for reading every other test in this file as a
/// measurement**, and the row Pass 4b learned to write first.
///
/// Clause 7.3.1 permits two tags to share one data element, and real
/// profiles use it — the Windows system SWOP profile aliases `A2B0` and
/// `A2B2` onto **one** offset, which is why the test written against
/// that file asserts the two intents give *identical* answers. Had this
/// fixture been laid out the same way, every assertion below would have
/// been reading one table three times and reporting green.
///
/// Evidence class: **`derived-expectation`** on the parsed tag table.
/// The comparison is of integers, with no arithmetic in it, so `0` is
/// honestly available as the tolerance.
#[test]
fn the_six_lut_tags_occupy_six_distinct_offsets() {
    let p = separated();
    let wanted = [*b"A2B0", *b"A2B1", *b"A2B2", *b"B2A0", *b"B2A1", *b"B2A2"];
    let mut found = Vec::new();
    for sig in wanted {
        let want = u32::from_be_bytes(sig);
        let entry = p
            .tags
            .iter()
            .find(|t| t.sig.0 == want)
            .unwrap_or_else(|| panic!("fixture must carry {}", String::from_utf8_lossy(&sig)));
        found.push((sig, entry.offset, entry.size));
    }
    for (i, (sa, oa, _)) in found.iter().enumerate() {
        for (sb, ob, _) in found.iter().skip(i + 1) {
            assert_ne!(
                oa,
                ob,
                "{} and {} share offset {oa} — the fixture has collapsed into an alias and \
                 every other test in this file would be reading one table under three names",
                String::from_utf8_lossy(sa),
                String::from_utf8_lossy(sb),
            );
        }
    }
    // Also assert there are no malformations: a fixture the parser
    // complains about is not a fixture anybody should measure through.
    assert_eq!(
        p.malformations.len(),
        0,
        "fixture must be well-formed; got {:?}",
        p.malformations
    );
    eprintln!("A0 six distinct offsets: {found:?}");
}

// ---------------------------------------------------------------------------
// A1 — the headline
// ---------------------------------------------------------------------------

/// ★★★ **THE ROW THIS FILE EXISTS FOR.** Through a chain whose
/// destination is the separated fixture, the `K` the CMM emits at
/// `Intent::Saturation` exceeds the `K` at `Intent::MediaRelative` by
/// exactly `IS_K_FLOOR[2] − IS_K_FLOOR[1] = 0,20`, at every one of 125
/// probes.
///
/// **Signed, not absolute**, deliberately: a CMM that swapped `B2A1`
/// and `B2A2` would satisfy an absolute-value assertion and fail this
/// one. The magnitude says the intent reached *a different* table; the
/// sign says it reached *the right* one.
///
/// The same statement is made for media-relative minus perceptual, so a
/// CMM that got one of the two selections right and the other wrong
/// cannot pass on the sum.
///
/// Rival hypothesis, named and measured elsewhere in this run: *the CMM
/// ignored the intent and read `B2A0` three times*, which gives exactly
/// `0,0` — see [`the_sibling_fixture_is_the_zero_separation_control`].
/// Candidate separation `0,20`, i.e. `2 × 10¹¹` times [`EXACT`].
#[test]
fn saturation_and_media_relative_select_different_b2a_tables() {
    let (src, dst) = (rgb_source(), separated());
    let per = run(&src, &dst, Intent::Perceptual);
    let rel = run(&src, &dst, Intent::MediaRelative);
    let sat = run(&src, &dst, Intent::Saturation);

    let probes = rgb_probes();
    let (mut worst_sr, mut worst_rp) = (0.0f64, 0.0f64);
    let (mut min_sr, mut min_rp) = (f64::INFINITY, f64::INFINITY);

    for (i, p) in probes.iter().enumerate() {
        assert_eq!(sat[i].len(), 4, "destination is CMYK");
        let (k_per, k_rel, k_sat) = (per[i][3], rel[i][3], sat[i][3]);

        let sr = k_sat - k_rel;
        let rp = k_rel - k_per;
        worst_sr = worst_sr.max((sr - K_SEPARATION_SAT_MINUS_REL).abs());
        worst_rp = worst_rp.max((rp - K_SEPARATION_REL_MINUS_PER).abs());
        min_sr = min_sr.min(sr);
        min_rp = min_rp.min(rp);

        assert!(
            (sr - K_SEPARATION_SAT_MINUS_REL).abs() <= EXACT,
            "probe {p:?}: K(saturation) - K(media-relative) = {sr}, expected \
             {K_SEPARATION_SAT_MINUS_REL} within {EXACT}. \
             0.0 would mean the intent was ignored and B2A0 read twice; \
             a negative value would mean B2A1 and B2A2 are swapped."
        );
        assert!(
            (rp - K_SEPARATION_REL_MINUS_PER).abs() <= EXACT,
            "probe {p:?}: K(media-relative) - K(perceptual) = {rp}, expected \
             {K_SEPARATION_REL_MINUS_PER} within {EXACT}"
        );
    }

    // The floor, asserted separately from the equality — see
    // MINIMUM_USEFUL_SEPARATION for why both are needed.
    assert!(
        min_sr >= MINIMUM_USEFUL_SEPARATION,
        "the fixture's saturation-vs-media-relative separation has fallen to {min_sr}, \
         below the {MINIMUM_USEFUL_SEPARATION} floor: the instrument no longer has power \
         even if the equality above still holds"
    );
    assert!(min_rp >= MINIMUM_USEFUL_SEPARATION);

    eprintln!(
        "A1 K(sat)-K(rel): min {min_sr:.15} worst deviation from {K_SEPARATION_SAT_MINUS_REL} \
         {worst_sr:.3e} over {} probes",
        probes.len()
    );
    eprintln!(
        "A1 K(rel)-K(per): min {min_rp:.15} worst deviation from {K_SEPARATION_REL_MINUS_PER} \
         {worst_rp:.3e}"
    );
}

// ---------------------------------------------------------------------------
// A2 — the guard that keeps A1 honest
// ---------------------------------------------------------------------------

/// `C`, `M` and `Y` are authored **identically** in all three `B2Ax`,
/// so the three intents must return them **bit-identically**.
///
/// ★★ **This is what makes A1's `K` difference attributable.** Today
/// `Chain::new` applies neither black point compensation nor K-only
/// preservation — both are opt-in, by an explicit and documented choice.
/// If either ever became intent-conditional by default, it would move
/// `C M Y` and redden *this* test, rather than quietly becoming the
/// source of the `K` difference A1 reads as proof of tag selection.
///
/// `==` and not a tolerance: the three arms run the same arithmetic on
/// the same bytes in the same order, so anything but bit-equality is a
/// structural difference and a tolerance would be a place for it to
/// hide.
#[test]
fn cmy_is_bit_identical_across_the_three_intents() {
    let (src, dst) = (rgb_source(), separated());
    let per = run(&src, &dst, Intent::Perceptual);
    let rel = run(&src, &dst, Intent::MediaRelative);
    let sat = run(&src, &dst, Intent::Saturation);
    for (i, p) in rgb_probes().iter().enumerate() {
        for ch in 0..3 {
            assert_eq!(
                (per[i][ch], rel[i][ch]),
                (per[i][ch], sat[i][ch]),
                "probe {p:?} channel {ch}: C/M/Y moved with the intent. The three B2Ax \
                 tables carry identical C, M and Y columns, so something OTHER than tag \
                 selection is intent-conditional in this chain — and A1's K difference is \
                 no longer attributable to tag selection alone."
            );
            assert_eq!(rel[i][ch], sat[i][ch]);
        }
    }
}

// ---------------------------------------------------------------------------
// A3 — the named rival, RUN rather than argued
// ---------------------------------------------------------------------------

/// The zero-separation control. `v2-cmyk-chromatic-neutral.icc` carries
/// **only** `B2A0`, so clause 8.10.2's fallback must send all three
/// intents to the same table and the measured separation must be
/// **exactly zero**.
///
/// ★★★ Two things at once, and the second is the important one:
///
/// 1. It regresses the **fallback** limb of 8.10.2, which is what every
///    profile in the tree except the new fixture exercises.
/// 2. It **measures the rival hypothesis** for A1. *"The CMM ignored
///    the intent"* is not left as an argument — it is executed, in the
///    same run, on a committed profile, and it produces `0.0`. A1's
///    candidate separation is therefore a measured `0,20 − 0,0`, not an
///    assumed one.
///
/// ★ If this test ever fails while A1 passes, the diagnosis is that
/// something intent-conditional has been added to the chain that is not
/// tag selection; A1 alone could not tell you that.
#[test]
fn the_sibling_fixture_is_the_zero_separation_control() {
    let (src, dst) = (rgb_source(), zero_separation_sibling());
    let per = run(&src, &dst, Intent::Perceptual);
    let rel = run(&src, &dst, Intent::MediaRelative);
    let sat = run(&src, &dst, Intent::Saturation);
    for (i, p) in rgb_probes().iter().enumerate() {
        assert_eq!(
            per[i], rel[i],
            "probe {p:?}: a profile carrying only B2A0 must give one answer at every intent"
        );
        assert_eq!(per[i], sat[i], "probe {p:?}");
    }
    eprintln!("A3 zero-separation control: separation exactly 0.0 on the B2A0-only sibling");
}

// ---------------------------------------------------------------------------
// A4 — ICC-absolute shares B2A1 with media-relative
// ---------------------------------------------------------------------------

/// Clause 8.10.2 gives **ICC-absolute colorimetric** the *same* table as
/// media-relative (`B2A1`); the two differ by the white-point scaling
/// applied ahead of it, not by the table.
///
/// ★ On **this pair** that scaling is the identity: the synthetic source
/// and the synthetic destination both carry the corpus's D50 `wtpt`, so
/// the adaptation is a ratio of equal numbers. The two intents must
/// therefore agree **exactly** — which turns a hard-to-bound claim into
/// an equality.
///
/// ★★ And it is load-bearing rather than decorative: if ICC-absolute
/// wrongly reached `B2A2`, `K` would differ by `0,20` and this test
/// would fail by eleven orders of magnitude. It is the third of the
/// three destination-side selections, and the only one the tree has no
/// other check on.
#[test]
fn icc_absolute_selects_the_same_b2a_as_media_relative() {
    let (src, dst) = (rgb_source(), separated());
    let rel = run(&src, &dst, Intent::MediaRelative);
    let abs = run(&src, &dst, Intent::Absolute);
    let mut worst = 0.0f64;
    for (i, p) in rgb_probes().iter().enumerate() {
        for ch in 0..4 {
            let d = (abs[i][ch] - rel[i][ch]).abs();
            worst = worst.max(d);
            assert!(
                d <= EXACT,
                "probe {p:?} channel {ch}: absolute {} vs media-relative {} differ by {d}. \
                 Both profiles carry the same D50 wtpt, so the adaptation is the identity \
                 and the two intents share B2A1; a difference near 0,20 would mean absolute \
                 reached B2A2 instead.",
                abs[i][ch],
                rel[i][ch]
            );
        }
    }
    eprintln!("A4 |absolute - media-relative| max {worst:.3e}");
}

// ---------------------------------------------------------------------------
// A5 — the SOURCE side, for completeness
// ---------------------------------------------------------------------------

/// The `A2Bx` half of the same fixture, driven as a **source**: the three
/// tables differ by a per-intent `b*` offset of `+0`, `+8`, `+16`
/// CIELAB units, so the three intents must give three different answers.
///
/// ## ★ Why this row is graded on DISTINCTNESS and not on magnitude
///
/// The `B2A` rows above can assert an exact number because a constant
/// added to a **device-side** output arrives at the output unchanged.
/// Here the constant is added in the **PCS**, and the destination is a
/// matrix/TRC profile with a non-linear `curv` table — so the device
/// difference at the far end is a real function of the probe, with no
/// closed form this file may write down without minting colour maths
/// (`CLAUDE.md` rule 2).
///
/// So the **graded** predicate is the one that *is* derivable: the three
/// intents did not read the same table. Its named rival — the
/// source-side fallback firing and `A2B0` being read three times —
/// produces **bit-identical** output, i.e. exactly `0,0`, by the same
/// argument [`the_sibling_fixture_is_the_zero_separation_control`]
/// measures on the destination side. Any non-zero difference refutes it.
/// [`DISTINCT_FLOOR`] sits six orders above the floating-point noise of a
/// deterministically identical computation (which is *zero*, not small)
/// and is **not** derived from what was measured. The **magnitude** is
/// `REPORTED, NOT GRADED`, printed below: a number with no derived bound
/// does not get an assertion.
///
/// ## ★★★ THE FINDING THIS ROW PRODUCED ON ITS FIRST RUN — a downstream CLIP CANCELS A DIFFERENTIAL
///
/// The first version of this test asserted the floor at **every** probe
/// and **failed at two of eighty-one**: device CMYK `(0, 1, 0, 0)` and
/// `(1, 1, 0, 0)`. Both are saturated inks whose PCS image lies **well
/// outside the destination RGB profile's gamut**, so both arms clip to
/// the *same* cube corner — `(1, 0, 1)` and `(0, 0, 1)` respectively —
/// and a real, correctly-selected `8` `b*` unit difference arrives as
/// **exactly zero**.
///
/// ★★ **Generalisation, and it is the transferable half:** a differential
/// that is sound at the point where it is injected can be **annihilated
/// by a later stage of the same chain**. Clipping is idempotent and
/// many-to-one; so is a gamut-mapping step, a quantiser, and any
/// saturating encode. **`DL-077` says list what the arms SHARE; this
/// says also list what the pipeline DESTROYS after they diverge.** The
/// destination-side rows above are immune for a stated reason — the
/// fixture's `K` column spans `0 … 0,80` and so never reaches a clip —
/// which is why the design put the label where it did.
///
/// **How the exclusion is handled**, and it is not by deleting the two
/// probes: a probe is exempt from the floor only if **every channel of
/// both arms is pinned at a cube boundary (`0,0` or `1,0`)**, which is a
/// *checked* criterion rather than a chosen list. A probe that collapses
/// for any other reason still fails. The number excluded is printed, so
/// a change that quietly starts clipping everything is visible as a
/// count rather than as continued green.
#[test]
fn the_three_a2b_tables_are_reached_as_the_source_side_of_a_chain() {
    /// See the doc comment: the row refutes a bit-identical rival, so any
    /// positive floor suffices; six orders above zero-noise.
    const DISTINCT_FLOOR: f64 = 1e-6;

    let (src, dst) = (separated(), rgb_source());
    let probes: Vec<[f64; 4]> = {
        let mut v = Vec::new();
        for c in 0..3 {
            for m in 0..3 {
                for y in 0..3 {
                    for k in 0..3 {
                        v.push([
                            f64::from(c) / 2.0,
                            f64::from(m) / 2.0,
                            f64::from(y) / 2.0,
                            f64::from(k) / 2.0,
                        ]);
                    }
                }
            }
        }
        v
    };
    let go = |intent: Intent| -> Vec<Vec<f64>> {
        let chain = Chain::new(&src, &dst, intent)
            .unwrap_or_else(|e| panic!("chain must build at {intent:?}: {e}"));
        probes
            .iter()
            .map(|p| chain.convert(p).expect("convert"))
            .collect()
    };
    let per = go(Intent::Perceptual);
    let rel = go(Intent::MediaRelative);
    let sat = go(Intent::Saturation);

    /// Every channel of both arms pinned at a cube boundary: the one
    /// condition under which a correctly-selected difference is
    /// *required* to arrive as zero.
    fn fully_clipped(a: &[f64], b: &[f64]) -> bool {
        a.iter().chain(b.iter()).all(|&v| v == 0.0 || v == 1.0)
    }

    let spread = |a: &[f64], b: &[f64]| (0..3).fold(0.0f64, |m, ch| m.max((a[ch] - b[ch]).abs()));

    // Each PAIR is graded independently. The first version of this
    // loop required BOTH pairs to be clipped before exempting a probe,
    // and that was wrong for a reason worth keeping: at device
    // `(0, 1, 0, 0)` the perceptual and media-relative arms clip to the
    // same corner while the saturation arm — 8 more b* units away —
    // still separates. **Clipping is a property of a pair at a probe,
    // not of a probe.**
    let mut report: Vec<String> = Vec::new();
    for (name, a, b) in [
        ("saturation-vs-media-relative", &sat, &rel),
        ("media-relative-vs-perceptual", &rel, &per),
        // ★★★ THE THIRD PAIR IS NOT REDUNDANT, AND AN INJECTION IS
        // WHY IT IS HERE. With only the first two, injecting
        // `Intent::Saturation => tag::A2B0` into `derive_source_model`
        // — saturation silently aliased onto perceptual, exactly the
        // failure clause 8.10.2's step 3 fallback would produce if step 2
        // never fired — left this test GREEN: `A2B0` and `A2B1` still
        // differ, so both graded pairs still separated. **A
        // distinctness predicate over a subset of the pairs cannot see a
        // COLLAPSE ONTO THE PAIR IT DOES NOT LOOK AT.** All three pairs,
        // always.
        ("saturation-vs-perceptual", &sat, &per),
    ] {
        let (mut lo, mut hi) = (f64::INFINITY, 0.0f64);
        let mut clipped = 0usize;
        for (i, p) in probes.iter().enumerate() {
            let sep = spread(&a[i], &b[i]);
            if fully_clipped(&a[i], &b[i]) {
                clipped += 1;
                assert!(
                    sep == 0.0,
                    "{name}: probe {p:?} is fully clipped in both arms yet they differ by \
                     {sep} — the exemption criterion no longer describes what happened"
                );
                continue;
            }
            lo = lo.min(sep);
            hi = hi.max(sep);
            assert!(
                sep >= DISTINCT_FLOOR,
                "{name}: probe {p:?} gave the same answer in both arms ({sep}) at a probe \
                 that is NOT clipped — the source-side A2Bx selection did not happen"
            );
        }
        assert!(
            clipped * 4 < probes.len(),
            "{name}: {clipped} of {} probes are fully clipped; the probe set has drifted \
             out of the destination's gamut and this row measures the clip, not the selection",
            probes.len()
        );
        report.push(format!(
            "{name}: min {lo:.6e} max {hi:.6e} over {} unclipped probes ({clipped} clipped)",
            probes.len() - clipped
        ));
    }

    eprintln!(
        "A5 REPORTED (not graded): device spread, {} CMYK probes -- {}",
        probes.len(),
        report.join(" | ")
    );
}
