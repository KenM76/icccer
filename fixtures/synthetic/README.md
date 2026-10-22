# fixtures/synthetic

Profiles authored byte-by-byte by this project's own generator
(`tools/gen-profiles`, built during Pass 2). Unrestricted, category (a)
per `docs/LEGAL.md` §3 — **prefer these for everything**.

**46 fixtures as of 2026-08-18: 18 well-formed, 28 deliberately malformed, and
0 disputed.** (`gen-profiles list` is the authoritative count; the figure here
is a typed numeral in prose and is dated for that reason — it has already been
wrong twice.) The malformed group is the one usually missing from a fixture
corpus and the more valuable here, because `docs/ARCHITECTURE.md` §3.2 makes *reporting* — not repairing —
the parser's contract, and a contract with no failing input has no test. Each
malformed fixture carries exactly **one** named defect: a fixture broken in two
ways cannot tell you which one the consumer reported.

★ **The third category is an admission rather than a claim, and it is
currently EMPTY.** A **disputed** fixture is one whose *required consumer
behaviour cannot yet be written down* — the specification has not been read on
the exact point the fixture probes, so neither “this must report” nor “this must
be silent” can be recorded without inventing the answer. Its `MANIFEST.md` row
therefore carries a **dated measurement of what icccer does today**, not a
requirement on a consumer.

★★ **The category emptied on the day it was created, and that is it working.**
Its only member was `v2-rendering-intent-high-bits.icc`, filed disputed on
2026-08-18 while the question *“does ICC.1:2001-04 restrict a v2 `renderingIntent`
field to 0..3?”* was outstanding. `icc-spec-librarian` returned the clause text
the same day: 6.1.11 defines four values, contains **no `shall`**, and never
uses the *“other values are reserved”* formula the same document uses elsewhere
when it means to close a set. The fixture moved to **well-formed** and its row
now asserts a **silence** — a report icccer must not make. Nothing about the
bytes changed; only what could truthfully be claimed about them. That
transition is the entire purpose of the category, and the empty state is the
record that it completed.

**What puts a fixture here** — one condition, and it is about the *sourcing*,
never about the *code*: the bytes exercise a rule whose text has not been read
on the exact point at issue, so a dispatch to `icc-spec-librarian` is
outstanding and its answer would change which real category the fixture belongs
to. **What does not**: (a) *icccer’s behaviour is believed wrong* — that is a
defect, and the fixture is filed under what the **standard** says, which is what
makes the suite go red; a category is not a place to park a bug. (b) *the text
has been read and licenses more than one consumer behaviour* — that is settled,
not disputed; the fixture is filed under what the file **is**, and the project’s
own choice is recorded in the row **as a choice**.
`v2-rendering-intent-low-half.icc` is the worked example of (b): it violates
nothing, and icccer reports it anyway, deliberately.

**Every fixture in this directory is reproducible from a generator invocation
recorded alongside it** — a synthetic fixture that cannot be regenerated is just
a binary blob with better branding. The invocations are in
[`MANIFEST.md`](MANIFEST.md), which is itself *generated* so the record cannot
drift from the artefact:

```text
cd tools/gen-profiles
cargo run -- all ../../fixtures/synthetic
cargo run -- verify ../../fixtures/synthetic     # byte-for-byte, names the
                                                 # first differing byte
cargo run -- manifest > ../../fixtures/synthetic/MANIFEST.md
```

Generation is deterministic: no clock, no RNG, no environment. **Do not edit a
fixture by hand** — `verify` will fail, which is the point.

★ **Nothing here is a colorimetric reference.** The colorant columns are an
arbitrary split of the encoded D50 white point (chosen so they sum to it
exactly), the tone curves are exact powers of two or linear ramps, and the CLUTs
are simple documented functions of their grid indices. These files are evidence
about **structure** — that bytes in a stated layout decode to stated values —
and never about colour. See `tools/gen-profiles/README.md` §4.

★★ **One fixture is an INSTRUMENT rather than a sample, and the distinction is
worth knowing before quoting it.** `v2-cmyk-chromatic-neutral.icc` exists so
that the predicate *"K-only in implies K-only out"* has **two different
answers** on a profile that needs no licence: its `B2A0` separates a neutral
into all four inks by construction, and a black-preserving consumer and a
non-preserving one differ by **`0.420 705`** in device units on its K ramp. Its
sibling `v2-cmyk-mft2-lab.icc` cannot do that — its `B2A0` emits `[0,0,0,k]` at
every node, so the two answers coincide. **Both are kept**: the second is the
measurement that says why the first had to exist.

Its two CLUT models are **affine with no cross terms**, and its `B2A0` is
`a*`/`b*`-**independent across three node lines about the neutral axis**, both
on purpose — those properties are what make every expectation taken from it
exact for *any* conformant interpolation scheme rather than only for the one a
particular consumer uses. That also means it is **less like a real press than
any profile in the suite corpus**, and a number measured on it is evidence
about the *predicate*, never about ink. `tools/gen-profiles/src/recipes.rs`
carries the full derivation; `docs/TOLERANCES.md` §3.10.11 carries the
tolerances that rest on it.

★★ **A second instrument, added 2026-08-18: a PAIR, and the pair is the
instrument.** `v2-rgb-header-intent-perceptual.icc` and
`v2-rgb-header-intent-relative.icc` are **byte-identical except for one byte,
at file offset 67** — the low byte of the header's `renderingIntent` field
(clause 7.2.15), `00h` against `01h`. Neither member means anything on its
own; the *difference* is the whole apparatus, and it exists so the question
*"does a profile's declared intent reach the transform when the caller names
none?"* has an answer that is a measurement rather than a reading of the code.

Two properties make it able to answer that, and both are choices:

* **Its `A2B0` and `A2B1` differ at every CLUT node** — 12 units apart in both
  `a*` and `b*`, so no input can produce equal output from the two tags. A pair
  whose two tables coincided would let "the outputs are identical" pass while
  the mechanism under test was fully live: a zero-separation fixture does not
  merely fail to inform, it turns a red result green.
* **Every table entry is well inside the sRGB gamut**, so the separation cannot
  be erased by clipping at a destination.

Measured behaviour, the open specification question it does **not** answer, and
the two defect injections that prove the test can fail are in
`crates/icccer-cli/tests/header_rendering_intent_not_consumed.rs` and
`docs/TOLERANCES.md` §3.11.

★★ **A third instrument, added 2026-09-01, and this one is the DESTINATION
side.** `v2-cmyk-intent-separated-b2a.icc` carries **all three `A2Bx` and all
three `B2Ax`** as six distinct, non-aliased `mft2` tags. Until it existed, no
committed fixture carried `A2B2` or `B2A2` at all, and the only two carrying
`B2A1` pushed **byte-identical payloads** into `B2A0` and `B2A1` — so the whole
tree's evidence that a saturation request reaches a *different destination
table* than a media-relative one rested on the Windows system SWOP profile, in
a test that returns **success** when that file is absent. **A skipped test and
a passing test print the same colour.**

The three `B2Ax` are `v2-cmyk-chromatic-neutral`'s separation with **one scalar
changed** — a per-intent black floor added to `K` at every node, `0,00` /
`0,20` / `0,40` — with `C`, `M` and `Y` held **identical** across all three. A
constant added to every CLUT node survives every interpolation scheme exactly
(each is an affine combination of the cell's corners with weights summing to
one), and the offsets are exact 16-bit codes (`0,20 × 65 535 = 13 107`), so the
quantisation cancels rather than being bounded. **The measured separation is
therefore the constant itself, at every PCS point, under any conformant CMM.**
`crates/icccer-cmm/tests/b2a_intent_selection_is_separated.rs` asserts it as an
*equality*; `docs/TOLERANCES.md` §3.12 carries the rows, the scope and nine
injection runs.

★★★ **It is the fixture in this corpus most likely to be mistaken for a press
model, because it is the only one that names an intent.** A saturation table
that differs from a media-relative table by a flat 20 % black floor is **not**
what a saturation intent does; ICC.1 says nothing that would make it so and no
instrument produced these numbers. It supports exactly one claim, and the claim
is structural: *which byte block the consumer read*.

★ **What it got wrong that the pair above got right, recorded because the rule
was already written here and was still missed.** The paragraph above says of
the header-intent pair: *"every table entry is well inside the sRGB gamut, so
the separation cannot be erased by clipping at a destination."* This fixture's
**`A2Bx`** offsets (`b*` `+0` / `+8` / `+16`) do **not** honour that, and on
first run two of eighty-one CMYK probes clipped to the same cube corner at the
destination and reported a real difference as **exactly zero**. Its **`B2Ax`**
offsets do honour it — `K` spans `0 … 0,80` and never reaches a clip — which is
why the graded rows are on the destination side. **Clipping is idempotent and
many-to-one; so is a gamut-mapping step and any saturating encode. A
differential that is sound where it is injected can be annihilated downstream.**

The verification record (what the shipped `icccer` binary and lcms2's `transicc`
do with each fixture, dated and scoped), the coverage statement, and the open
finding **GP-001** are in `tools/gen-profiles/README.md` §§5–7.
