#!/usr/bin/env python3
"""
scrub.py — apply the operator's 2026-08-25 substitution contract across this
repository, once, mechanically and reviewably.

WHY THIS EXISTS AS A SCRIPT RATHER THAN AS A SESSION OF EDITS
=============================================================
The ruling was *"keep mentions of <the suite> out of the public repo"*, and the
substitution contract lives outside this repository at
`D:\\Dev\\pdfce-private\\suite\\SCRUB_SPEC.md` — shared with the sibling project
so that two repositories end up with ONE vocabulary rather than two dialects.

A scrub done by hand across ~450 occurrences in ~58 files produces a diff no
one can review: every hunk looks plausible, and a single inconsistent choice
(`the suite` here, `the conformance suite` there) is invisible until someone
greps for a phrase that no longer exists uniformly. A script makes the
substitution table itself the reviewable artifact, and the diff merely its
consequence.

★ **It is written to be run ONCE and kept, not re-run.** After the first pass
the needles are gone, so a second run is a no-op — which is the point: the file
documents what was done to every historical entry in this tree, and a reader of
a scrubbed `SESSION_LOG.md` entry can come here to see exactly which term was
replaced by which, rather than wondering whether an entry was reworded.

ORDERING IS THE WHOLE CORRECTNESS ARGUMENT
==========================================
Substitutions are applied **longest-pattern-first**. `<NAME> PDF Output Suite
5.0` must be consumed before `<NAME> Output Suite`, which must be consumed
before bare `<NAME>`, or the bare rule fires first and leaves
`the print-conformance suite PDF Output Suite 5.0` behind. The table below is
in deliberate order and MUST NOT be sorted.

★ `<NAME>` stands in for the term itself, which this file must not contain --
the first draft of this docstring spelled it out in the worked example and
the CI gate caught it. A file that performs the scrub is exactly as bound by
the rule as any other.

WHAT IT REFUSES TO DO
=====================
* **It does not touch binary files.** `git grep -I` semantics: text only.
* **It does not rewrite a past record's meaning.** Per SCRUB_SPEC §6, this
  changes vocabulary only — every number, date, verdict, hash and conclusion in
  a `SESSION_LOG.md` or ledger entry is left exactly as it was. There is no
  substitution in the table that can alter one.
* **It does not delete entries** that are awkward to scrub.
* **It does not fix anything else while it is in there.** A scrub whose diff
  also contains unrelated corrections stops being reviewable as a scrub.

QUOTED CAPTIONS — HANDLED SEPARATELY, ON PURPOSE
================================================
SCRUB_SPEC §3 requires verbatim quotations of a patch's printed caption to be
**deleted rather than translated**, because the caption is the part carrying
the notice requirement and no substitution launders it. A regex cannot decide
what is a caption. This script reports candidates and edits none of them; the
engineer resolves each by hand.

USAGE
=====
    python tools/suite/scrub.py --dry-run     # counts per pattern, no writes
    python tools/suite/scrub.py               # apply
    python tools/suite/scrub.py --verify      # assert both needles are absent

Needles are base64 in the source for the same reason the CI gate encodes
them (`tools/check-suite-name-absent.py`): a file whose job is to assert a
word never appears must not itself be the word's most frequent occurrence, or
the gate can no longer tell a real mention from its own enforcement.
"""

import argparse
import base64
import pathlib
import re
import subprocess
import sys

REPO = pathlib.Path(__file__).resolve().parents[2]

# The two forbidden needles. Encoded so this file does not become the thing it
# forbids -- see the module docstring.
NAME = base64.b64decode("Z2hlbnQ=").decode()      # the suite's name
ACRO = base64.b64decode("Z3dn").decode()          # its publisher's acronym
NAME_T = NAME.title()
NAME_U = NAME.upper()
ACRO_U = ACRO.upper()

# --------------------------------------------------------------------------
# The substitution table. ORDER IS LOAD-BEARING: longest first.
# Mirrors D:\Dev\pdfce-private\suite\SCRUB_SPEC.md sections 1, 2, 4 and 5.
# --------------------------------------------------------------------------
SUBS = [
    # ---- 5. Renames: paths, before anything eats the bare name ------------
    (rf"tools/{NAME}/", "tools/suite/"),
    (rf"{NAME}_probe", "suite_probe"),
    (rf"{NAME}-v50", "suite-v50"),
    (rf"project_passg_{NAME}_population_findings", "project_passg_suite_population_findings"),
    (rf"project_{NAME}_compatibility", "project_suite_compatibility"),
    (rf"project-{NAME}-compatibility", "project-suite-compatibility"),
    (rf"{NAME_U}_COMPATIBILITY", "SUITE_COMPATIBILITY"),
    (rf"reply_{NAME}_render_harness", "reply_suite_render_harness"),

    # ---- 2. Identifiers, most-qualified first -----------------------------
    # A full patch file name collapses to the id alone; the descriptive tail
    # is DROPPED, not translated (SCRUB_SPEC §2, last row).
    (rf"\b(\d)_{ACRO_U}(\d+)_[A-Za-z0-9_.\-]*?\.pdf\b", r"PCS\1_\2"),
    (rf"\b(\d)_{ACRO_U}(\d+)\b", r"PCS\1_\2"),
    (rf"\b{ACRO_U}[ _]?(\d+\.\d+)\b", r"PCS \1"),      # criterion numbers
    (rf"\b{ACRO_U}[ _]?(\d{{2,}})\b", r"PCS\1"),       # bare patch codes
    (rf"{ACRO_U}_GenericCMYK", "PCS_GenericCMYK"),

    # ---- 1. The name, longest form first ----------------------------------
    (rf"{NAME_T} PDF Output Suite 5\.0", "the print-conformance suite"),
    (rf"{NAME_T} PDF Output Suite v?5\.0", "the print-conformance suite"),
    (rf"{NAME_T} PDF Output Suite", "the print-conformance suite"),
    (rf"{NAME_T} Output Suite v?5\.0", "the print-conformance suite"),
    (rf"{NAME_T} Output Suite", "the print-conformance suite"),
    (rf"the {NAME_T} suite", "the print-conformance suite"),
    (rf"{NAME_T} v5\.0", "the print-conformance suite"),
    (rf"{NAME_T} Workgroup", "the suite's publisher"),
    (rf"{NAME_T}-suite", "print-conformance-suite"),
    (rf"{NAME_T}-driven", "suite-driven"),
    (rf"{NAME_T}'s", "the suite's"),
    (rf"{NAME_T}\b", "suite"),
    (rf"{NAME_U}\b", "SUITE"),
    (rf"{NAME}\b", "suite"),

    # ---- 1. The publisher's acronym as an organisation --------------------
    (rf"\b{ACRO_U}'s\b", "the suite publisher's"),
    (rf"\b{ACRO_U}\b", "the suite's publisher"),
    (rf"\b{ACRO}\b", "suite-publisher"),
]

# Phrases that read badly after a mechanical pass. Applied AFTER, as cleanup.
POLISH = [
    ("the the print-conformance suite", "the print-conformance suite"),
    ("the the suite's publisher", "the suite's publisher"),
    ("a the print-conformance suite", "the print-conformance suite"),
    ("the print-conformance suite suite", "the print-conformance suite"),
    ("suite suite", "suite"),
    ("The the print-conformance suite", "The print-conformance suite"),
]

# Caption-quotation candidates: SCRUB_SPEC §3 says delete, never rename, and a
# regex cannot tell a caption from ordinary quoted prose. Reported only.
CAPTION_HINT = re.compile(r'"[^"\n]{20,}"|\u201c[^\u201d\n]{20,}\u201d')


def tracked_text_files():
    """Every tracked file git considers text. Mirrors `git grep -I`."""
    out = subprocess.run(["git", "ls-files", "-z"], cwd=REPO,
                         capture_output=True, check=True).stdout
    for raw in out.split(b"\x00"):
        if not raw:
            continue
        p = REPO / raw.decode("utf-8")
        if not p.is_file():
            continue
        try:
            chunk = p.open("rb").read(8192)
        except OSError:
            continue
        if b"\x00" in chunk:          # git's own binary heuristic
            continue
        yield p


def scrub_text(s):
    for pat, rep in SUBS:
        s = re.sub(pat, rep, s)
    for a, b in POLISH:
        s = s.replace(a, b)
    return s


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--verify", action="store_true")
    args = ap.parse_args()

    needle = re.compile(f"{NAME}|{ACRO}", re.I)

    if args.verify:
        bad = [str(p.relative_to(REPO)) for p in tracked_text_files()
               if needle.search(p.read_text(encoding="utf-8", errors="replace"))]
        names = [str(p.relative_to(REPO)) for p in tracked_text_files()
                 if needle.search(str(p.relative_to(REPO)))]
        if bad or names:
            print(f"FAIL: {len(bad)} file(s) with a forbidden term in contents, "
                  f"{len(names)} in file names")
            for f in (bad + names)[:40]:
                print("   ", f)          # path only; never the offending line
            return 1
        print("OK: neither term appears in any tracked text file or file name")
        return 0

    changed = captions = 0
    for p in tracked_text_files():
        try:
            s = p.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        if not needle.search(s):
            continue
        new = scrub_text(s)
        for line in new.splitlines():
            if CAPTION_HINT.search(line) and "caption" in line.lower():
                captions += 1
                print(f"  CAPTION CANDIDATE (resolve by hand): "
                      f"{p.relative_to(REPO)}")
        if new != s:
            changed += 1
            if not args.dry_run:
                p.write_text(new, encoding="utf-8", newline="")
    verb = "would change" if args.dry_run else "changed"
    print(f"{verb} {changed} file(s); {captions} caption candidate(s) to review")
    return 0


if __name__ == "__main__":
    sys.exit(main())
