---
name: commit-message-claims-the-index-not-the-work
description: DL-073 — a commit message's claim about "this commit" (especially a NEGATIVE scope claim) is a claim about the INDEX, not about the work just done; `git add -A` after a whole-tree mechanical pass is what makes the two diverge, and after a push the only remedy is a dated correction in the log
metadata:
  type: project
---

**A commit message may describe the WORK, or it may describe the COMMIT —
and they are different sets. If it describes the commit, and especially if
it makes a negative scope claim (*"vocabulary only"*, *"nothing else was
fixed while in here"*, *"docs only"*), the claim must be read off the
index before committing, not written from memory of what was done.**

**Why:** on 2026-08-25 commit **`767233d`** ("scrub: the suite's name and
its publisher's acronym leave this repository") asserted *"nothing else
was fixed while in here — a scrub whose diff also contains corrections
stops being reviewable as a scrub."* The same commit also carried the
**entire consumer-requirements filing** — `NC-307`, `DL-072`,
`NUMERIC_CLAIMS.md` §3.39/§7.29, `ARCHITECTURE.md` §3.1/§4.1 — because
that filing was **uncommitted in the working tree** when a **whole-tree**
mechanical pass ran, and the commit was made with **`git add -A`**. The
clause was true of the pass's own edits and false of the commit. No step
was careless; the divergence is created by `-A` and frozen by the message.

**How to apply:**

- **Before staging: `git status`. Before committing: `git diff --cached
  --stat`.** One command; it is the only thing that makes a
  claim-about-the-commit checkable at the moment it is written.
- **The high-risk case is a mechanical pass over a dirty tree** — the pass
  edits files the author is not thinking about. Commit the in-flight work
  first, or stage the pass by path.
- **After a push, the remedy is the LOG, never an amend.** Rewriting a
  published ref is a destructive operation outside the working tree,
  reserved to the operator by rule 9 — and an operator instruction about
  *content* (*"doesn't matter if it is in the history"*, said about a
  name) is not a licence to rewrite *history*. File a dated
  `SESSION_LOG.md` entry that **names the commit hash**, so a reader who
  arrives at the message can grep for it.
- ★★★ **When a commit turns out to carry two changesets, ENUMERATE THEM
  SEPARATELY.** Retracting the false clause is not enough: an auditor
  reading a scrub commit that contains ledger sections will conclude that
  **measured content was produced by a substitution table**, which
  corrupts the evidence class of everything in it. That misreading is
  worse than the original error.
- **Do not fix anything else while filing the correction** — the
  correction for a commit that mixed two changesets must not mix two
  changesets.

**Not a duplicate of [[iccce-count-needs-its-apparatus]] (`DL-031`)**,
which is about a *count* in a commit message whose apparatus was omitted.
Here there is no number: the claim is *scope*, and its truth lived in the
index rather than in the author's terminal. The shared property, and the
reason both are decision-log entries: **a commit message is the one
artifact this project produces that cannot be corrected in place.**

★ **Related shape, opposite direction, same day:**
`tools/check-suite-name-absent.py` originally ran `git ls-files` (index
only) and reported OK on a tree whose *untracked* new files violated the
rule — **a gate whose input set is "what is already committed" cannot see
the commit you are about to make.** Same lesson from the other end: the
index and the work are not the same set. See
[[iccce-stale-status-decays-faster-than-a-number]] for the general habit
of carrying a dated reading instead of a remembered state.
