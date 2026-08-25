#!/usr/bin/env python3
"""
check-suite-name-absent.py — CI gate: two terms must never appear in this
repository, in file contents or in file names.

WHY
===
Operator ruling, 2026-08-25: *"Keep mentions of <the suite> out of the public
repo."* The one-off scrub is `tools/suite/scrub.py`; this is the gate that
keeps it true. Without a gate the rule survives exactly as long as everyone
who writes a `SESSION_LOG.md` entry remembers it, which is not long — the
terms are the natural words for the thing, and a single new entry
reintroduces them.

The forbidden terms are the licensed print-conformance suite's name and its
publisher's acronym. `D:\\Dev\\pdfce-private\\suite\\SCRUB_SPEC.md` holds the
substitution contract and names them in full; this repository does not.

★ THE PROPERTY WORTH UNDERSTANDING BEFORE EDITING THIS FILE
===========================================================
**The needles are base64-encoded, and that is not security theatre.** A gate
for *"this word must never appear"* written as a plain `git grep -i <word>`
makes the gate itself the rule's first and most conspicuous violation. Worse,
it destroys the gate's own signal: a reader grepping the repository can no
longer tell a real mention from the gate hunting for one, and `git grep` run
by a human returns the enforcement machinery as a hit forever.

Encoding is the only way for a rule and its enforcement to coexist in one
repository without contradiction. There is nothing secret here — the private
contract spells both terms out, and this file's own docstring says exactly
what they are.

TWO THINGS IT CHECKS THAT A CONTENTS-ONLY GREP MISSES
=====================================================
1. **File names.** A path is published in every directory listing, every diff
   and the repository's web view, so it survives a contents-only scrub
   completely. Five paths in this repository carried the name before
   2026-08-25, including an entire directory.
2. **Its own exclusions are narrow.** Binary files are skipped by git's own
   heuristic (a NUL byte in the first 8 KiB) rather than by an extension
   allowlist, because a match inside compiled data is a coincidence of bytes
   and not a mention — but an extension list would also have skipped real
   text files nobody thought of.

WHAT IT DELIBERATELY DOES NOT DO
================================
★ **It never echoes an offending term.** CI logs on a public repository are
public, so a gate that printed the matching line would publish the term it
exists to suppress, on every failure, in a log that outlives the fix. It
prints the file path and the count; the engineer greps locally.

★★ **And a violating FILE NAME is masked before printing**, which is easy to
miss: the contents branch has nothing to leak because it prints only a count,
but the name branch's whole payload IS the path, and the path is the
violation. Caught by this gate's own injection test rather than by review --
the first version printed `docs/_probe_<term>_name.md` in full.

EXIT CODES
==========
    0  neither term appears in any tracked text file or file name
    1  at least one occurrence — paths and counts printed, never the text
"""

import base64
import re
import subprocess
import sys

# The two forbidden terms. Encoded — see the docstring; this is the one
# property of this file that must not be "simplified" in review.
NEEDLES = [base64.b64decode(b).decode() for b in ("Z2hlbnQ=", "Z3dn")]
PATTERN = re.compile("|".join(re.escape(n) for n in NEEDLES), re.IGNORECASE)

# This file necessarily contains the base64 of the needles, and the scrub
# script does too. Neither contains the decoded terms, so neither is exempt
# from the CONTENT check — only from nothing at all. Kept as a list so that
# adding an exemption is a visible, reviewable act rather than a silent one.
EXEMPT: list[str] = []


def tracked_files():
    """Every file that IS or WOULD BE committed — tracked plus untracked.

    ★★ `--others --exclude-standard` is the whole point of this function and
    was added after the omission cost a red CI run on 2026-08-25.

    A bare `git ls-files` lists only files already in the index. Run locally
    BEFORE staging, that silently excludes exactly the files a session has
    just written — which on that day were three: this repository's own scrub
    script, whose docstring spelled the term out in a worked example, and two
    agent-memory notes written minutes earlier. The gate reported OK. Then
    `git add -A` made all three tracked, and CI — which checks out a tree
    where everything is tracked — failed on all three.

    ⇒ **A gate whose input set is "what is already committed" cannot see the
    commit you are about to make.** Locally it answers a question about the
    past; on CI it answers one about the present, and the two disagree
    precisely on the new work. Including untracked-but-not-ignored files makes
    the local answer and the CI answer the same question.

    `--exclude-standard` keeps `.gitignore` honoured, so build output and the
    untracked local reference document are still skipped — they are not going
    to be committed and are not in scope.
    """
    out = subprocess.run(
        ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"],
        capture_output=True, check=True).stdout
    # `--cached --others` can list the same path twice; dedupe, keep order.
    seen, files = set(), []
    for r in out.split(b"\x00"):
        if not r:
            continue
        rel = r.decode("utf-8")
        if rel not in seen:
            seen.add(rel)
            files.append(rel)
    return files


def main() -> int:
    content_hits: list[tuple[str, int]] = []
    name_hits: list[str] = []

    for rel in tracked_files():
        if rel in EXEMPT:
            continue

        # File NAMES first — a contents-only scrub leaves these behind and
        # they are the most visible form of all.
        if PATTERN.search(rel):
            name_hits.append(rel)

        try:
            with open(rel, "rb") as fh:
                head = fh.read(8192)
                if b"\x00" in head:      # git's own binary heuristic
                    continue
                body = head + fh.read()
        except OSError:
            continue

        try:
            text = body.decode("utf-8")
        except UnicodeDecodeError:
            text = body.decode("latin-1")

        n = len(PATTERN.findall(text))
        if n:
            content_hits.append((rel, n))

    if not content_hits and not name_hits:
        print("OK: neither forbidden term appears in any tracked text file "
              "or file name.")
        return 0

    # Paths and counts only. Never the matching line -- see the docstring.
    print("FAIL: the suite name or its publisher's acronym is present.")
    print("      See D:\\Dev\\pdfce-private\\suite\\SCRUB_SPEC.md for the")
    print("      substitution contract, and tools/suite/scrub.py for the")
    print("      table that was applied on 2026-08-25.")
    if name_hits:
        # ★ MASKED, and this is the branch that is easy to get wrong. The
        # contents branch has nothing to leak because it prints only a count;
        # the name branch's entire payload IS the path, and the path is the
        # violation. Printing it verbatim would publish the term in a public
        # CI log on every failure — the exact outcome this gate exists to
        # prevent. Caught by the gate's own injection test on 2026-08-25,
        # not by review. The mask leaves the path locatable.
        print(f"\n  {len(name_hits)} FILE NAME(S), term masked as ***:")
        for rel in name_hits:
            print(f"    {PATTERN.sub('***', rel)}")
    if content_hits:
        print(f"\n  {len(content_hits)} FILE(S) BY CONTENT "
              f"(count only; grep locally to see them):")
        for rel, n in sorted(content_hits, key=lambda t: -t[1]):
            print(f"    {n:5d}  {rel}")
    return 1


if __name__ == "__main__":
    sys.exit(main())
