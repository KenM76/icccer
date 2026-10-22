---
name: icccer-release-range-is-not-the-dispatch-list
description: A release note is a claim about a COMMIT RANGE — verify it against the reflog, not the dispatch. Pass G/H were listed as 0.2.0 content and had shipped in 0.1.0; and refs/tags/vX is an ANNOTATED TAG OBJECT, not the tagged commit.
metadata:
  type: project
---

**When a dispatch tells you what a release contains, that list is a claim
about a commit range, and the range is the one thing the dispatch cannot
establish by describing it. Check it before writing a public changelog.**

**Why:** on 2026-08-25 the `0.2.0` dispatch listed nine content items and
one of them — *"the suite corpus measured (Pass G), with §3.30.5's bound
stated plainly"* — **had shipped in `0.1.0`**. Commit `4bf291a`
*"conformance: Pass G and Pass H — real-world profiles, graded"* sits at
`.git/logs/HEAD` line **79**; the four `0.1.0` release commits are at
lines **89–92**; and the dispatch's own range began at `46de522`, line
**93**. Printing it would have claimed a previous release's work as this
one's, **in the artifact a downstream consumer reads to decide what
changed** — and a changelog is not a document anyone re-audits.

★★ **The trap that made it non-obvious, and it is reusable:**
`.git/refs/tags/v0.1.0` holds `d637318`, and **`d637318` appears nowhere
in the reflog.** That is not corruption — it is an **annotated tag**, so
the ref holds a *tag object*, not the commit it points at. **A tag ref
does not give you the tagged commit**, and with no shell you cannot
`rev-parse` it. What worked instead: locate the release's *own* commits
by subject in the reflog (`release: 0.1.0 — …`, `release: the lockfile
the 0.1.0 bump left behind`) and read the ordering off the line numbers.

★ Related properties of the same evidence, worth knowing before relying
on it: a reflog line evidences a commit's **existence, time and subject**
and **never its contents**; and reflog ordering is *this clone's* HEAD
history, which is linear here but is not a substitute for ancestry.

**How to apply:** for any changelog, release note, "what shipped" summary
or Pass-completion claim — before writing a single content bullet, read
`.git/logs/HEAD` and locate the previous release's commits in it. Any
dispatch item whose commit is *older* than those is **previous-release
work**: drop it and say so. This costs one read. See
[[icccer-pass-status]] for the specific instance, and
[[icccer-count-needs-its-apparatus]] for the same shape one level down —
a number is scoped to the run that produced it exactly as a feature is
scoped to the release that shipped it.

★★ **CONFIRMED AGAIN 2026-09-01 at `v0.3.0`, and the method that works is
now settled.** `.git/refs/tags/v0.3.0` reads `e3e78bb…` and matches **no**
commit in the reflog — third tag, third annotated tag object. What
corroborated the release without a shell: `.git/logs/refs/heads/master`
for the commits, their **order** and their **subjects**, and
`.git/logs/refs/remotes/origin/master` for `update by push` events. ★★★
**Both refs and the reflogs are readable; a CI RESULT IS NOT.** No filing
in this project has ever carried a workflow **run id**, so *"CI is green"*
has no durable form here and must be written `[REPORTED]` every time —
`NUMERIC_CLAIMS.md` §7.35 newly-owed 2. ★ And the dispatch's *"`v0.2.0` is
the precedent"* turned out to be **false**: there was no `v0.2.0` release
entry at all, for the reason in
[[icccer-unowned-refusal-is-a-slow-deletion]].

★★ **The other half of the same filing, because it arrived together:**
the dispatch also asked for the sentence *"NC-001 remains the only
`published-ground-truth` row in the whole ledger."* **Retired at §7.19,
eleven filings earlier** — `NC-230` is the second and `tools/difftest`
holds a third population. **The live sentence is the narrower one: *no
`published-ground-truth` row exists for any TRANSFORM*.** A retired
sentence keeps circulating because it is short, quotable and flatters
nobody, so **nothing in the suite ever goes red over it**. See
[[icccer-ground-truth-cannot-exist]].
