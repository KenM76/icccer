#!/usr/bin/env python3
"""
patch_compare.py — compare `icccer transform` output against pixel regions of a
reference render, for a set of device-colour patches described by a CASE FILE.

WHAT THIS IS FOR
================
The question "does icccer agree with some other renderer on this colour?" keeps
recurring, and answering it by hand each time invites two failures this project
cares about: the comparison gets described loosely ("close enough"), and the
evidence class gets forgotten ("matches the other renderer" said as though it
were ground truth). This tool exists so that the comparison is written down
as DATA — a case file — and the caveats are printed by the program rather than
remembered by the operator.

It does exactly three things:

  1. Converts a list of device colours through `icccer transform`, for every
     (source profile x rendering intent) pair the case file names.
  2. Samples a reference image over the regions the case file names.
  3. Prints both, side by side, in 8-bit channel counts.

★★ THE SEPARATION THAT MATTERS, AND WHY THIS FILE HAS NO NUMBERS IN IT

This program knows **nothing** about any particular test document, corpus, or
reference render. Every device value, every pixel coordinate, every profile
path, every structural fact about a source file lives in a **case file** that
is supplied at the command line and is **NOT part of this repository**.

That is not a style preference. `docs/SUITE_COMPATIBILITY.md` was untracked by
operator decision on 2026-08-18 because detailed analysis of, and quotation
from, a licensed third-party test suite does not belong in an MIT repository:
the suite's licence carries an affirmative-notice requirement and a
"commercial context" restriction, and the decision was to avoid creating that
entanglement at all rather than to rely on a fair-quotation argument. The first
version of this tool (`tools/suite/shading_cell.py`, 2026-08-25) inlined a
licensed patch file's shading dictionary, its indirect-object numbers, its
endpoint colours and the identity of the press profile it embeds — and so
inherited the same problem and had to be untracked too.

★ The operator's decision, 2026-08-25, was to **split it**: a generic tool that
may be public, plus a local case file that may not. This is the generic half.
Keep it that way. **If you find yourself about to write a colour value, a pixel
coordinate, a stitch bound or a document's name into this file, that belongs in
a case file instead.**

★★ Case files should live OUTSIDE the repository, not merely be gitignored
inside it. A gitignored path is one `git add -f` away from the thing the
decision was made to prevent.

EVIDENCE CLASS — THE PART THE PROGRAM WILL NOT LET YOU SKIP
===========================================================
A case file MUST declare `reference.evidence_class`, and the tool prints that
declaration above every table. There is no default, because a defaulted
evidence class is exactly how "icccer matches X" turns into a claim nobody
chose to make. `icccer`'s ledger recognises, from strongest to weakest:

    published-ground-truth              a value from the literature
    implementation-cross-check          e.g. lcms2, whose path is pinned
    rendering-cross-check-screen-capture  a grab of another app's window
    acceptance                          a comparison happened at all

★★★ **NO DELTA-E IS EVER PRINTED BY THIS TOOL.** Output is 8-bit channel
counts only. For a screen-capture reference this is a standing prohibition
(`docs/ARCHITECTURE.md` DL-070) with two reasons: a delta-E against a screen
grab dresses a rendering comparison up as colorimetry, and an out-of-gamut
value clamped at a channel endpoint (ICC.1:2022 Annex F.8-F.16, applied by
`MatrixTrc::pcs_to_device` before the inverse TRC) would make the delta-E
measure the clamp rather than the colour. If a future case genuinely needs a
delta-E, that requires REVERSING a decision, which is the intended friction.

CASE FILE FORMAT
================
JSON. See `README.md` in this directory for a complete worked example.

    {
      "name":        "<short label, printed>",
      "notes":       "<free text, printed under the name>",

      "destination": "<path to the destination ICC profile>",
      "intents":     ["media-relative", "perceptual"],

      "sources": [
        {"label": "<name>", "path": "<path to an ICC profile>"},
        {"label": "<name>", "from_pdf": "<path>", "extract_to": "<path>"}
      ],

      "reference": {
        "image":          "<path to a rendered PNG>",
        "evidence_class": "<one of the four names above>",
        "provenance":     "<how it was produced. Printed verbatim.>"
      },

      "patches": [ <patch>, ... ]
    }

A `sources` entry with `from_pdf` has the PDF's `/OutputIntents` ->
`/DestOutputProfile` stream inflated and written to `extract_to` first. That
extraction is generic PDF/ICC plumbing and so lives here; WHICH pdf is case
data and so does not.

Two patch kinds:

    {                                   a single constant colour
      "kind":   "constant",
      "id":     "<label>",
      "device": [c, m, y, k],           floats in 0..1; length = source's
                                        channel count
      "region": <region>,
      "caveat": "<optional, printed with the row>"
    }

    {                                   a parametric ramp, area-weighted
      "kind":   "ramp",
      "id":     "<label>",
      "stops":  [{"t": 0.0,  "device": [...]},
                 {"t": 0.75, "device": [...]},
                 {"t": 1.0,  "device": [...]}],
      "weighting": {"kind": "radial_area_over_square", "samples": 1201},
      "region": <region>,
      "caveat": "<optional>"
    }

`ramp` interpolates linearly in t between consecutive stops, which reproduces a
PDF FunctionType 3 stitch over FunctionType 2 exponentials with N = 1 provided
the stops are placed at the stitch bounds. `radial_area_over_square` weights
each t by the area of the square [-1,1]^2 at radius t, with t clamped to 1 —
the distribution you get from a radial shading with both Extend flags true
painted over its bounding square.

★ A `ramp`'s weighting is an ASSUMPTION about geometry, never a fact read out
of a document. Say so in `caveat`; the tool prints it, and a ramp row without
a caveat prints a warning of its own.

Two region kinds, both in pixels of the reference image:

    {"kind": "box",     "box": [x0, x1, y0, y1], "inset": 2}
    {"kind": "annulus", "box": [x0, x1, y0, y1], "r_min": 1.0, "r_max": 1.3}

`annulus` normalises radius to the box's half-width, so `r_min: 1.0` starts
just outside the inscribed circle — the corners. That is the useful region for
a radial ramp whose outer plateau is constant: it samples one colour and no
interpolation.

USAGE
=====
    python tools/patch-compare/patch_compare.py --case <case.json>
    python tools/patch-compare/patch_compare.py --case <case.json> --patch plateau
    python tools/patch-compare/patch_compare.py --case <case.json> --dump-devices

`--dump-devices` prints the device values a patch expands to and converts
nothing — the cheap way to check a case file says what you meant.

DEPENDENCIES
============
`numpy` and `Pillow`, plus a built `icccer` binary (`--icccer`, default
`target/release/icccer`). None of these enter the crate graph; this is a
development tool and `Cargo.lock` is unaffected.

EXIT CODES
==========
    0  every requested comparison ran
    1  an input was missing, or `icccer transform` refused a request
    2  usage error, or a case file that does not satisfy the schema
"""

import argparse
import json
import os
import re
import subprocess
import sys
import zlib

import numpy as np


class UsageError(Exception):
    """A case file or command line that does not satisfy the schema.

    Separated from operational failure so the exit code can match the
    engine's own convention (`crates/icccer-cli/src/main.rs`): **2 means the
    caller asked for something malformed, 1 means the request was well formed
    and could not be carried out.** Python's bare `raise SystemExit("msg")`
    exits 1 for both, which silently collapses that distinction — it did here,
    and the header documented a 2 the program never produced.
    """


class OperationalError(Exception):
    """A well-formed request that failed: missing file, refused transform."""


EVIDENCE_CLASSES = (
    "published-ground-truth",
    "implementation-cross-check",
    "rendering-cross-check-screen-capture",
    "acceptance",
)


# --------------------------------------------------------------------------
# Source profiles
# --------------------------------------------------------------------------

def extract_output_intent(pdf_path, out_path):
    """Inflate a PDF's `/OutputIntents` -> `/DestOutputProfile` to `out_path`.

    Deliberately crude: this scans for `N 0 obj` markers rather than reading
    the cross-reference table, because all it needs is to reach one stream and
    the result is then checked by a test that does not depend on having parsed
    the PDF correctly at all — bytes 36..40 of an ICC profile are the ASCII
    signature `acsp` (ICC.1:2022 section 7.2.5). If the wrong stream were
    reached, that check fails rather than producing a plausible profile.

    Returns `out_path`. Raises `SystemExit` with a reason on any failure; a
    silently substituted profile would produce plausible wrong colour, which
    is the failure mode this project exists to avoid.
    """
    data = open(pdf_path, "rb").read()
    m = re.search(rb"/OutputIntents (\d+) 0 R", data)
    if not m:
        raise OperationalError("no /OutputIntents in %s" % pdf_path)
    offsets = {int(o.group(1)): o.end()
               for o in re.finditer(rb"[\r\n](\d+) 0 obj", data)}
    arr = data[offsets[int(m.group(1))]:][:200]
    ref = re.search(rb"\[(\d+) 0 R", arr)
    if not ref:
        raise OperationalError("/OutputIntents is not an array of references")
    dic = data[offsets[int(ref.group(1))]:][:800]
    prof = re.search(rb"/DestOutputProfile (\d+) 0 R", dic)
    if not prof:
        raise OperationalError("output intent carries no /DestOutputProfile")
    p = offsets[int(prof.group(1))]
    s = data.find(b"stream", p)
    length = re.search(rb"/Length (\d+)", data[p:s])
    if not length:
        raise OperationalError("/DestOutputProfile stream has no direct /Length")
    icc = zlib.decompress(data[s + 8: s + 8 + int(length.group(1))])
    if icc[36:40] != b"acsp":
        raise OperationalError("inflated stream is not an ICC profile "
                               "(no `acsp` at bytes 36..40)")
    os.makedirs(os.path.dirname(out_path) or ".", exist_ok=True)
    open(out_path, "wb").write(icc)
    return out_path


def resolve_sources(case):
    """Turn the case file's `sources` list into [(label, path), ...]."""
    out = []
    for src in case["sources"]:
        if "path" in src:
            out.append((src["label"], src["path"]))
        elif "from_pdf" in src:
            out.append((src["label"],
                        extract_output_intent(src["from_pdf"], src["extract_to"])))
        else:
            raise UsageError("source %r has neither `path` nor `from_pdf`"
                             % src.get("label"))
    return out


# --------------------------------------------------------------------------
# Conversion
# --------------------------------------------------------------------------

def resolve_icccer(path):
    """Find the engine binary, tolerating a missing `.exe` on Windows.

    Two Windows behaviours make the obvious `target/release/icccer` fail, both
    with the same uninformative "cannot find the file specified":

    * `subprocess` does NOT apply PATHEXT to a path containing a separator, so
      the `.exe` has to be found here rather than by the OS.
    * `CreateProcess` does not accept a RELATIVE path written with forward
      slashes. The same path spelled absolutely works, which is why this
      returns `os.path.abspath` rather than the candidate it matched — a
      difference that costs half an hour if it is not written down.

    Fails with a sentence rather than an errno.
    """
    for cand in (path, path + ".exe"):
        if os.path.isfile(cand):
            return os.path.abspath(cand)
    raise OperationalError("icccer binary not found at %r (nor %r). Build it "
                           "with `cargo build --release -p icccer-cli`, or pass "
                           "--icccer." % (path, path + ".exe"))


def convert(icccer, src, dst, intent, lines):
    """Run `icccer transform` over newline-separated device values.

    Returns `(array_of_rgb_floats, None)` or `(None, first_line_of_stderr)`.

    ★ Values come back exactly as the engine printed them. The clamp that
    matters has already happened inside the engine — each linear component
    clamped to [0,1] BEFORE its inverse TRC, ICC.1:2022 Annex F.8-F.16, see
    `MatrixTrc::pcs_to_device` in `crates/icccer-cmm/src/matrix_trc.rs` and the
    test `out_of_gamut_clamps_before_inverse_trc` beside it. Nothing here
    re-clamps, so a channel sitting at exactly 0 or exactly 1 is the engine's
    normative gamut clamp and not this script tidying up. That distinction is
    load-bearing: it is the difference between "the colour is outside the
    destination gamut" and "the tool rounded".
    """
    p = subprocess.run(
        [icccer, "transform", "--src", src, "--dst", dst, "--intent", intent],
        input=lines, capture_output=True, text=True)
    if p.returncode != 0:
        return None, (p.stderr.strip().splitlines() or ["refused"])[0]
    rows = [[float(v) for v in ln.split()]
            for ln in p.stdout.strip().split("\n")]
    return np.array(rows), None


def to8(rgb):
    """Quantise to 8 bits.

    `np.round` is round-HALF-TO-EVEN (the IEEE 754 default), not
    round-half-away. That is stated rather than silently accepted because a
    patch whose value lands exactly on a .5 boundary would then differ by one
    count from a renderer that rounds half-away — and would differ silently.
    No such value has been observed; the sentence exists so that the day one
    appears, the explanation is already written.

    The clip afterwards is a no-op for anything the engine produces, since the
    engine already clamped per Annex F. It is here so this function cannot
    emit 256 if that ever stops being true.
    """
    return np.clip(np.round(np.asarray(rgb) * 255.0), 0, 255)


# --------------------------------------------------------------------------
# Patches -> device values
# --------------------------------------------------------------------------

def lerp(a, b, u):
    return [x + (y - x) * u for x, y in zip(a, b)]


def ramp_devices(patch):
    """Expand a `ramp` patch into (lines, weights).

    Colour is piecewise-linear in t between consecutive stops. Weight is the
    area of the square [-1,1]^2 at each radius, with t = min(radius, 1) — i.e.
    a radial shading with both Extend flags true, painted over its bounding
    square. Values are quantised in t before conversion so that a 1201x1201
    sample grid costs a couple of thousand conversions rather than 1.4 million;
    colour depends on t alone, so the quantisation loses nothing but ties.
    """
    stops = sorted(patch["stops"], key=lambda s: s["t"])
    w = patch.get("weighting", {"kind": "radial_area_over_square"})
    if w.get("kind") != "radial_area_over_square":
        raise UsageError("unknown ramp weighting %r" % w.get("kind"))
    n = int(w.get("samples", 1201))

    xs = np.linspace(-1.0, 1.0, n)
    gx, gy = np.meshgrid(xs, xs)
    t = np.clip(np.hypot(gx, gy), 0.0, 1.0)
    q = np.round(t * 2000).astype(int)
    uniq, counts = np.unique(q, return_counts=True)
    weights = counts.astype(float)
    weights /= weights.sum()

    lines = []
    for tv in uniq / 2000.0:
        lo = stops[0]
        hi = stops[-1]
        for a, b in zip(stops, stops[1:]):
            if a["t"] <= tv <= b["t"]:
                lo, hi = a, b
                break
        span = hi["t"] - lo["t"]
        u = 0.0 if span <= 0 else (tv - lo["t"]) / span
        lines.append(" ".join("%.6f" % v
                              for v in lerp(lo["device"], hi["device"], u)))
    return "\n".join(lines) + "\n", weights


def patch_devices(patch):
    """(lines, weights) for any patch kind. Weights are None for `constant`."""
    kind = patch.get("kind", "constant")
    if kind == "constant":
        return " ".join("%.6f" % v for v in patch["device"]) + "\n", None
    if kind == "ramp":
        return ramp_devices(patch)
    raise UsageError("unknown patch kind %r" % kind)


# --------------------------------------------------------------------------
# Reference image sampling
# --------------------------------------------------------------------------

def sample_region(image_path, region):
    """Return an (N, 3) int array of reference pixels for a region."""
    from PIL import Image
    a = np.asarray(Image.open(image_path).convert("RGB")).astype(int)
    x0, x1, y0, y1 = region["box"]
    kind = region.get("kind", "box")
    if kind == "box":
        i = int(region.get("inset", 0))
        return a[y0 + i:y1 + 1 - i, x0 + i:x1 + 1 - i].reshape(-1, 3)
    if kind == "annulus":
        cx, cy, hw = (x0 + x1) / 2.0, (y0 + y1) / 2.0, (x1 - x0) / 2.0
        ys, xs = np.mgrid[y0:y1 + 1, x0:x1 + 1]
        r = np.hypot(xs - cx, ys - cy) / hw
        sel = (r >= float(region.get("r_min", 1.0))) & \
              (r <= float(region.get("r_max", 1.3)))
        return a[y0:y1 + 1, x0:x1 + 1][sel]
    raise UsageError("unknown region kind %r" % kind)


def wpct(values, weights, p):
    """Weighted percentile. Weights are area, not sample count."""
    order = np.argsort(values)
    v, cw = values[order], np.cumsum(weights[order])
    return v[min(int(np.searchsorted(cw, p)), len(v) - 1)]


# --------------------------------------------------------------------------
# Reporting
# --------------------------------------------------------------------------

def report_constant(patch, sources, case, icccer_bin):
    """Print one constant patch. Returns the number of REFUSED conversions.

    A refusal is printed and the sweep continues — `icccer transform` refuses
    by design in places (`--bpc` outside the estimable subset, an unknown
    intent), and those refusals are exactly what a sweep is for. But the run
    is then incomplete, so the count is propagated and turns into exit 1 at
    the end. Printing a refusal and exiting 0 would let a half-run sweep be
    quoted as though every row had been measured.
    """
    refused = 0
    px = sample_region(case["reference"]["image"], patch["region"])
    med = [int(np.median(px[:, i])) for i in range(3)]
    print("  reference region: %d px, %d distinct RGB"
          % (len(px), len(np.unique(px, axis=0))))
    print("  " + ("%-22s%-16s" + "%7s" * 3)
          % ("source", "intent", "R", "G", "B"))
    print("  " + ("%-22s%-16s" + "%7d" * 3)
          % ("REFERENCE", "-", med[0], med[1], med[2]))
    lines, _ = patch_devices(patch)
    for label, path in sources:
        for intent in case["intents"]:
            rgb, err = convert(icccer_bin, path, case["destination"],
                               intent, lines)
            if rgb is None:
                print("  %-22s%-16s  refused: %s" % (label, intent, err))
                refused += 1
                continue
            v = to8(rgb)[0]
            print("  " + ("%-22s%-16s" + "%7d" * 3)
                  % (label, intent, v[0], v[1], v[2]))
    return refused


def report_ramp(patch, sources, case, icccer_bin):
    """Print one ramp patch. Returns the number of REFUSED conversions."""
    refused = 0
    px = sample_region(case["reference"]["image"], patch["region"])
    head = ("%-22s%-16s" + "%7s" * 6) % (
        "source", "intent", "R med", "G p25", "G med", "G p75", "B med", "B max")
    print("  reference region: %d px" % len(px))
    print("  " + head)
    print("  " + ("%-22s%-16s" + "%7d" * 6) % (
        "REFERENCE", "-", int(np.median(px[:, 0])),
        int(np.percentile(px[:, 1], 25)), int(np.median(px[:, 1])),
        int(np.percentile(px[:, 1], 75)),
        int(np.median(px[:, 2])), int(px[:, 2].max())))
    lines, w = patch_devices(patch)
    for label, path in sources:
        for intent in case["intents"]:
            rgb, err = convert(icccer_bin, path, case["destination"],
                               intent, lines)
            if rgb is None:
                print("  %-22s%-16s  refused: %s" % (label, intent, err))
                refused += 1
                continue
            q = to8(rgb)
            print("  " + ("%-22s%-16s" + "%7d" * 6) % (
                label, intent,
                int(wpct(q[:, 0], w, .50)),
                int(wpct(q[:, 1], w, .25)), int(wpct(q[:, 1], w, .50)),
                int(wpct(q[:, 1], w, .75)),
                int(wpct(q[:, 2], w, .50)), int(q[:, 2].max())))
    return refused


def validate(case):
    """Reject a case file that would produce an unlabelled claim."""
    for key in ("name", "destination", "intents", "sources", "reference",
                "patches"):
        if key not in case:
            raise UsageError("case file is missing `%s`" % key)
    ref = case["reference"]
    for key in ("image", "evidence_class", "provenance"):
        if key not in ref:
            raise UsageError("case file's `reference` is missing `%s` — there "
                             "is no default, because a defaulted evidence "
                             "class is how an unclaimed claim gets made" % key)
    if ref["evidence_class"] not in EVIDENCE_CLASSES:
        raise UsageError("unknown evidence_class %r; known: %s"
                         % (ref["evidence_class"], ", ".join(EVIDENCE_CLASSES)))


def main():
    ap = argparse.ArgumentParser(
        description="Compare icccer transform output against a reference "
                    "render, for the patches a case file describes.")
    ap.add_argument("--case", required=True, help="path to a case JSON file")
    ap.add_argument("--icccer", default="target/release/icccer",
                    help="path to the built icccer binary")
    ap.add_argument("--patch", action="append",
                    help="only this patch id (repeatable)")
    ap.add_argument("--dump-devices", action="store_true",
                    help="print each patch's device values and convert nothing")
    args = ap.parse_args()

    case = json.load(open(args.case, encoding="utf-8"))
    validate(case)

    patches = case["patches"]
    if args.patch:
        patches = [p for p in patches if p.get("id") in args.patch]
        if not patches:
            raise UsageError("no patch matched %s" % args.patch)

    if args.dump_devices:
        for patch in patches:
            lines, w = patch_devices(patch)
            rows = lines.strip().split("\n")
            print("%s (%s): %d device rows" % (patch.get("id"),
                                               patch.get("kind", "constant"),
                                               len(rows)))
            for r in rows[:4]:
                print("   ", r)
            if len(rows) > 4:
                print("    ... %d more" % (len(rows) - 4))
        return 0

    ref = case["reference"]
    print("CASE: %s" % case["name"])
    if case.get("notes"):
        print("      %s" % case["notes"])
    print("REFERENCE evidence class: %s" % ref["evidence_class"])
    print("      provenance: %s" % ref["provenance"])
    print("OUTPUT is 8-bit channel counts. No delta-E is printed — see this "
          "file's header and ARCHITECTURE.md DL-070.")
    print("")

    icccer_bin = resolve_icccer(args.icccer)
    sources = resolve_sources(case)
    refused = 0
    for patch in patches:
        kind = patch.get("kind", "constant")
        print("PATCH %s (%s)" % (patch.get("id", "?"), kind))
        if patch.get("caveat"):
            print("  ! %s" % patch["caveat"])
        elif kind == "ramp":
            print("  ! this ramp declares no `caveat`, and a ramp's area "
                  "weighting is an ASSUMPTION about geometry rather than a "
                  "fact read out of a document. Add one.")
        if kind == "constant":
            refused += report_constant(patch, sources, case, icccer_bin)
        else:
            refused += report_ramp(patch, sources, case, icccer_bin)
        print("")
    if refused:
        print("%d conversion(s) were REFUSED; this sweep is incomplete."
              % refused, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except UsageError as e:
        print("patch_compare: %s" % e, file=sys.stderr)
        sys.exit(2)
    except OperationalError as e:
        print("patch_compare: %s" % e, file=sys.stderr)
        sys.exit(1)
