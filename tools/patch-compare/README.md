# `tools/patch-compare` — comparing icccer against another renderer, on the record

A development tool. Not shipped, not a crate, not in `Cargo.lock`.

It answers one recurring question — *"does `icccer` agree with some other
renderer on this colour?"* — in a way that makes the answer citable: the
comparison is written down as **data** (a *case file*), and the caveats are
**printed by the program** rather than remembered by whoever ran it.

```
python tools/patch-compare/patch_compare.py --case <case.json>
```

---

## 1. Why it is split into a tool and a case file

**The tool contains no numbers.** No colour values, no pixel coordinates, no
document names, no structural facts about anybody's test file. All of that
lives in a case file supplied on the command line.

That is a licensing decision, not a style one.

`docs/SUITE_COMPATIBILITY.md` was untracked by operator decision on
**2026-08-18**: detailed analysis of, and quotation from, a licensed
third-party test suite does not belong in an MIT repository, because that
suite's licence carries an affirmative-notice requirement and a "commercial
context" restriction. The decision was to *avoid creating the entanglement at
all* rather than to rely on a fair-quotation argument.

On **2026-08-25** a first version of this tool
(`tools/suite/shading_cell.py`) was written to answer a question from `pdfce`,
and to do its job it inlined a licensed patch file's shading dictionary, its
indirect-object numbers, its endpoint colour values and the identity of the
press profile it embeds. It inherited exactly the same problem. The operator's
decision was to **split it**: this generic half may be public; the specifics
may not.

> ★★ **Case files belong OUTSIDE this repository, not in a gitignored path
> inside it.** A gitignored path is one `git add -f` away from the thing the
> decision was made to prevent. `.gitignore` says so at the bottom.

★ **If you are about to write a colour value, a pixel coordinate, a stitch
bound or a document's name into `patch_compare.py`, it belongs in a case file
instead.** That is the whole rule and it is the only one that keeps this file
publishable.

---

## 2. Why it refuses to print a ΔE

**Output is 8-bit channel counts. Always. There is no flag.**

For a screen-capture reference this is a standing prohibition —
`docs/ARCHITECTURE.md` **DL-070** — with two independent reasons:

1. **A ΔE against a screen grab dresses a rendering comparison up as
   colorimetry.** The capture went through the other application's display
   transform to whatever monitor profile the OS happened to have, then through
   8-bit quantisation. None of those stages is recorded. A number in ΔE units
   asserts a colorimetric relationship that the artifact cannot support.
2. **The value may be gamut-clamped.** ICC.1:2022 Annex F.8–F.16 clamps each
   linear component to `[0,1]` *before* its inverse TRC — see
   `MatrixTrc::pcs_to_device` in `crates/icccer-cmm/src/matrix_trc.rs` and the
   test `out_of_gamut_clamps_before_inverse_trc` beside it. When a colour is
   outside the destination gamut, a ΔE computed in that destination measures
   **the clamp**, not the colour.

If a future case genuinely needs a ΔE, that requires **reversing a decision**
rather than filling in a gap. The friction is the point.

---

## 3. Why it refuses to run without an evidence class

`reference.evidence_class` is **mandatory and has no default**, because a
defaulted evidence class is precisely how *"icccer matches the other one"* becomes a
claim nobody chose to make. The tool prints the declaration above every table.

| class | what it means | strength |
|---|---|---|
| `published-ground-truth` | a value from the literature | strongest |
| `implementation-cross-check` | another implementation whose path is *pinned* (lcms2, via `tools/difftest`) | |
| `rendering-cross-check-screen-capture` | pixels out of a grab of another application's window | |
| `acceptance` | a comparison against an independent artifact happened at all | weakest |

The third class was added to `docs/NUMERIC_CLAIMS.md` §1 on 2026-08-25 and is
ranked deliberately **below** `implementation-cross-check`: an lcms2
comparison names its version, build, profile and intent, while a window grab's
display leg is unrecorded and unreproducible.

---

## 4. The case file, field by field

```jsonc
{
  "name":  "<short label, printed above the tables>",
  "notes": "<free text, printed under the name>",

  // Destination profile. One, for the whole case.
  "destination": "C:/Windows/System32/spool/drivers/color/sRGB Color Space Profile.icm",

  // Every intent is run against every source. Names as `icccer transform`
  // spells them: media-relative, perceptual, saturation, absolute.
  "intents": ["media-relative", "perceptual"],

  "sources": [
    { "label": "AdobeRGB", "path": "<path to an ICC profile>" },

    // Or: inflate a PDF's /OutputIntents -> /DestOutputProfile first.
    // The extraction is generic plumbing and lives in the tool; WHICH pdf
    // is case data and lives here.
    { "label": "embedded", "from_pdf": "<path>", "extract_to": "<path>" }
  ],

  "reference": {
    "image":          "<path to a rendered PNG>",
    "evidence_class": "rendering-cross-check-screen-capture",
    "provenance":     "<how it was produced. Printed verbatim. Say what is
                        NOT recorded, not just what is.>"
  },

  "patches": [ /* see below */ ]
}
```

### Patch kind `constant` — one colour

```jsonc
{
  "kind":   "constant",
  "id":     "plateau",
  "device": [0.1, 0.2, 0.3, 0.0],   // floats in 0..1, one per source channel
  "region": { "kind": "annulus", "box": [100, 199, 40, 139],
              "r_min": 1.0, "r_max": 1.3 },
  "caveat": "<printed with the table>"
}
```

★ **Prefer a constant patch over a ramp wherever the document gives you one.**
A large area of a single colour is entangled with no gradient geometry and no
renderer interpolation, so a disagreement there is about the colour transform
and nothing else.

### Patch kind `ramp` — a parametric family, area-weighted

```jsonc
{
  "kind":  "ramp",
  "id":    "whole-cell",
  "stops": [ { "t": 0.0,  "device": [0.4, 0.0, 0.1, 0.0] },
             { "t": 0.75, "device": [0.0, 0.3, 0.2, 0.0] },
             { "t": 1.0,  "device": [0.0, 0.3, 0.2, 0.0] } ],
  "weighting": { "kind": "radial_area_over_square", "samples": 1201 },
  "region": { "kind": "box", "box": [100, 199, 40, 139], "inset": 2 },
  "caveat": "THE AREA WEIGHTING IS A RECONSTRUCTION. ..."
}
```

Colour is **piecewise-linear in `t`** between consecutive stops, which
reproduces a PDF `FunctionType 3` stitch over `FunctionType 2` exponentials
with `N = 1` provided the stops sit at the stitch bounds.

`radial_area_over_square` weights each `t` by the area of the square
`[-1,1]²` at radius `t`, with `t` clamped to 1 — the distribution a radial
shading with both `Extend` flags true produces over its bounding square.

> ★ **A ramp's weighting is an ASSUMPTION about geometry, never a fact read
> out of a document.** How much of a cell's *area* sits at each `t` depends on
> the clip path and CTM the content stream sets before `sh`. Declare it in
> `caveat`. A ramp with no `caveat` makes the tool print a warning of its own.

### Regions

| | |
|---|---|
| `{"kind": "box", "box": [x0,x1,y0,y1], "inset": 2}` | the whole rectangle, inset to drop antialiased edges |
| `{"kind": "annulus", "box": [x0,x1,y0,y1], "r_min": 1.0, "r_max": 1.3}` | radius normalised to the box's **half-width**, so `r_min: 1.0` starts just outside the inscribed circle — the **corners** |

`annulus` is the right region for a radial ramp whose outer plateau is
constant: past the stitch bound, every corner pixel carries the same colour.

---

## 5. A worked example you can run

This one uses only profiles Windows ships, so it needs nothing licensed. Save
it **outside the repository** — `D:/Dev/temp/example.case.json`:

```json
{
  "name": "AdobeRGB primaries into sRGB",
  "notes": "A shape demonstration. The reference image is a placeholder.",
  "destination": "C:/Windows/System32/spool/drivers/color/sRGB Color Space Profile.icm",
  "intents": ["media-relative", "perceptual"],
  "sources": [
    { "label": "AdobeRGB", "path": "C:/Windows/System32/spool/drivers/color/AdobeRGB1998.icc" },
    { "label": "AppleRGB", "path": "C:/Windows/System32/spool/drivers/color/AppleRGB.icc" }
  ],
  "reference": {
    "image": "D:/Dev/temp/some-render.png",
    "evidence_class": "acceptance",
    "provenance": "placeholder — replace before quoting any row of this case"
  },
  "patches": [
    { "kind": "constant", "id": "red",   "device": [1.0, 0.0, 0.0],
      "region": { "kind": "box", "box": [0, 9, 0, 9] } },
    { "kind": "ramp",     "id": "r-to-g",
      "stops": [ { "t": 0.0, "device": [1.0, 0.0, 0.0] },
                 { "t": 1.0, "device": [0.0, 1.0, 0.0] } ],
      "caveat": "shape demo; the weighting is not claimed to match anything",
      "region": { "kind": "box", "box": [0, 9, 0, 9] } }
  ]
}
```

Check that it says what you meant — **this needs no reference image, because
`--dump-devices` converts nothing and samples nothing**:

```
python tools/patch-compare/patch_compare.py \
    --case D:/Dev/temp/example.case.json --dump-devices
```

```
red (constant): 1 device rows
    1.000000 0.000000 0.000000
r-to-g (ramp): 1995 device rows
    1.000000 0.000000 0.000000
    0.998500 0.001500 0.000000
    0.997500 0.002500 0.000000
    0.996500 0.003500 0.000000
    ... 1991 more
```

★ That transcript is a **paste of a real run**, not an illustration. If you
edit this README, re-run the command and paste again — an invented transcript
in a document about not inventing numbers would be a poor look, and the first
draft of this section had one.

The `1995` is worth a sentence, because it is not `2001`. A ramp quantises `t`
into 2001 bins and **six of them come out empty — bins 1, 2, 4, 6, 8 and 16**,
i.e. the ones nearest `t = 0`, where a discrete sample grid simply has no
point at that radius. (Checked, not reasoned: the last bin *is* populated, by
every corner pixel clamping into it.) The count is therefore a property of the
sampling grid and carries no meaning about the colour.

Then point `reference.image` at a real render and drop `--dump-devices`.

---

## 6. Running it, and the two Windows traps

```
python tools/patch-compare/patch_compare.py --case <case.json> \
       [--icccer target/release/icccer] [--patch <id>]... [--dump-devices]
```

Build the engine first: `cargo build --release -p icccer-cli`.

★ `--icccer` defaults to `target/release/icccer`, and **two separate Windows
behaviours** make that spelling fail with the same uninformative
`[WinError 2] The system cannot find the file specified`:

1. `subprocess` does not apply `PATHEXT` to a path containing a separator, so
   the `.exe` must be found explicitly.
2. `CreateProcess` **does not accept a relative path written with forward
   slashes**. The identical path spelled absolutely works.

`resolve_icccer()` handles both and fails with a sentence instead of an errno.
This is recorded here because it cost real time to diagnose and looks like a
missing build rather than a path-spelling quirk.

Requires `numpy` and `Pillow`. Neither enters the crate graph.

**Exit codes:** `0` ran · `1` missing input or a refused transform · `2` usage
error or a case file that fails the schema.

---

## 7. Relationship to the other tools here

| tool | question it answers |
|---|---|
| `tools/difftest` | does icccer agree with **lcms2**, numerically, to a stated tolerance? |
| `tools/gen-profiles` | synthetic fixtures with **known** contents |
| `tools/suite/extract_icc.py` | what profiles do **real producers** actually embed? |
| **`tools/patch-compare`** | does icccer agree with **another renderer's picture**, on colours a document actually paints? |

`difftest` is the strong instrument and this is the weak one — but it reaches
somewhere `difftest` cannot: a colour as a *document* specifies it, compared
against what a *shipping application* draws. Read the evidence-class table in
§3 before quoting any row it prints.
