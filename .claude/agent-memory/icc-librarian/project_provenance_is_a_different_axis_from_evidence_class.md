---
name: provenance-is-a-different-axis-from-evidence-class
description: DL-080 — a provenance challenge is discharged by RE-MEASURING, never by a hash or a commit message; twelve rows changed provenance and NOT their evidence class, and the challenge was cheap only because it was raised before the numbers left the repository. ★ Resolved hours later by the diff: a THIRD axis — provenance, evidence class, and WHICH INSTRUMENT was pointed at the question; the class stayed [REPORTED] and only the instrument improved
metadata:
  type: project
---

**A refusal to accept a provenance chain is discharged by re-taking the
measurement, and by nothing else.** Filed as `DL-080` (`ARCHITECTURE.md`
§5) on 2026-09-01, at the evidence-supply follow-up to the `v0.3.0`
release.

**Why:** `NC-308` … `NC-319` were measured on an **uncommitted tree**;
the release filing then supplied four corroborated commit hashes, and
this role refused them in one sentence — ***"a commit existing is not
evidence that it contains what was measured."*** The engineer **did not
argue; he re-ran** on a from-scratch rebuild. All twelve reproduced
exactly. The three cheaper responses available (a hash, a commit
message, an assurance the tree was clean) share one defect: each is a
claim *about* the artefact made by the party whose earlier claim is in
question — so accepting one closes the question without putting an
instrument on it. Same shape as [[documented-is-not-tested]] (inspection
could not find what injection found) and
[[injection-measures-one-magnitude]].

**How to apply — three things, and the second is the one that gets
rounded away:**

1. **When a dispatch offers a commit hash as provenance for a
   measurement, ask whether the measurement was re-run.** A hash
   anchors *code*. Nothing anchors a *number* except taking it again.
2. ★★★ **Provenance and evidence class are ORTHOGONAL AXES.** The
   re-run moved the rows from "deleted uncommitted tree" to "a tree that
   builds and runs" and moved **nothing else**: still `self-consistency`,
   still no oracle, still REPORTED-not-graded, still **asserted by no
   test**. **Never write a provenance upgrade as an accuracy upgrade,
   and never let "it reproduces" be read as "it is defended."**
   *Re-running is not a test* — a test fails on its own; a re-run
   happens only when someone chooses to do it. Same family as
   [[unfailable-row-protected-only-by-siblings]].
3. ★★★ **A re-measurement that reproduces its prior value EXACTLY mints
   NO ledger row.** Minting one double-counts: the ledger holds twelve
   observations taken twice, not twenty-four.

**★ The timing rule, which is the expensive half.** The challenge cost
one rebuild **because nothing downstream had quoted the figures yet**.
Raised a month later it would have required auditing every place they had
reached. **The boundary had already been crossed once** — the `pdfce`
channel reply quoting the twelve went out *before* the re-run, at a
provenance strength this ledger had already refused. Numbers were
correct; **that is an outcome, not a method.** ⇒ *The moment a figure is
about to leave the repository is the last cheap moment to challenge where
it came from.*

**★★ What I found by reading that the dispatch did not carry.** It
described the rebuild as *"from the tagged tree"* and supplied the output
that shows it is not: **`git describe` = `v0.3.0-1-gd328b95`** — the
`-1-g` means **one commit past the tag**. The gap is bridged by *"that
commit touches agent-memory and no source"*, which is a claim about a
commit's **scope** — precisely [[commit-message-claims-the-index-not-the-work]]
(`DL-073`), minted three commits earlier. **Read a `git describe` string
for its `-N-g` suffix before accepting "built from the tag."**

**★★★ RESOLVED hours later, 2026-09-01, and the resolution is a THIRD
axis.** The diff was run: `git diff --name-only a4d9003 d328b95` returns
**two paths, both under `.claude/agent-memory/icc-engineer/`**. So the
rebuilt artefact **is** the release artefact.

> ★★★ **The evidence CLASS did not move — it is still `[REPORTED]`, read
> out of a dispatch by a role with no shell. What moved is the
> INSTRUMENT: from the commit message `DL-073` rules out, to the diff
> `DL-073` prescribes.** Provenance, evidence class, and *which
> instrument was pointed at the question* are three separate things, and
> a filing can improve exactly one of them at a time.

★ **Two residuals that must not be swept** *(a correction does not sweep
its own block)*: the trees are **not byte-identical** — two Markdown
files differ, and *"Markdown under `.claude/` is not a build input"* is a
**labelled reading**; and the **path-filtered confirmation
(`-- crates/ Cargo.toml Cargo.lock fixtures/ tools/` ⟶ `0`) is WEAKER
than the unfiltered command it corroborates** — an enumerated pathspec
cannot see a root `build.rs`, `rust-toolchain.toml`, `.cargo/` or
`.github/` ([[gate-catches-only-enumerated-failures]]). **Quote the
unfiltered diff, never the filtered one, when the question is scope.**
