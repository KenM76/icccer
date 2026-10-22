---
name: icccer-unowned-refusal-is-a-slow-deletion
description: Declining to record an unevidenced claim is a HOLD, not a verdict — name who owes the evidence and what would discharge it, in the same paragraph. v0.2.0 ended up with no release record at all because two correct refusals were never re-asked.
metadata:
  type: project
---

**A refusal to record is a HOLD, not a verdict — and a hold with no owner
and no re-ask is indistinguishable, a month later, from a finding that the
event never occurred.**

**Why:** on 2026-09-01, filing the `v0.3.0` release, I searched for the
`v0.2.0` precedent the dispatch named and **there was none.**
`docs/ROADMAP.md` had no release entry for `v0.1.0` or `v0.2.0`, and
`docs/SESSION_LOG.md`'s only statements about `v0.2.0` were **refusals**
(`:7453`, `:7818`): a dispatch had asserted it was tagged, released and
CI-green, **carried evidence for none of the three**, and the filing
correctly declined to write any of it down.

★★ **The refusal was right** — this role may not assert an unmeasured fact
about the environment, and that rule exists for good reasons. **The hole it
left was not.** Nobody returned with the evidence, so the project's own
history was silent about its second release for six days, and would have
stayed silent indefinitely. Filed as `ARCHITECTURE.md` **`DL-079`** §3.

★ **It is [[icccer-negative-finding-removes-its-auditor]]'s mechanism
arriving in the paperwork instead of in the measurements.** There, a
**wrong** rejection survives forever because nothing re-audits the reason.
Here a **right** rejection did comparable damage by leaving a hole that
nothing pointed at. Both are failures of *ownership*, not of judgement.

★★ **Why it is invisible.** A wrong entry can be found by anyone who
re-checks it. **An absent entry has no text to grep, no id to look up, and
no date to notice going stale** — the only way it surfaces is somebody
going looking for a precedent that was never written. That is the same
asymmetry as [[icccer-gitignored-file-invisible-to-citation-sweep]]: the
error runs toward a **false all-clear**, and false all-clears are the ones
nobody audits.

**How to apply — the operative rule, on every filing:**

1. When you decline an item for want of evidence, **name who owes the
   evidence and what evidence would discharge it, in the same paragraph as
   the refusal.** Not in a separate to-do list, which decays separately.
2. **Record what you CAN evidence, and say plainly what you cannot.** At
   this filing the loose refs and reflogs supported *"the `v0.2.0` tag
   exists and reads `e9fc6b7…`"* and did **not** support any CI claim.
   Both halves went into `ROADMAP.md`'s release table with their evidence
   class in the column. A partial record beats an empty one and does not
   overstate.
3. **Open a numbered owed item** so the hold has an id that later status
   passes sweep (here: `NUMERIC_CLAIMS.md` §7.35 newly-owed 1).
4. Watch for the sibling failure — a claim that has **no durable form at
   all** in this project. *"CI is green"* is one: no filing has ever
   carried a run id, so the sentence decays exactly like *"N commits
   ahead"* did (`DL-068`) because **`success` and `failure` look identical
   in authority once the sentence is a month old.**

See [[icccer-pass-status]] for the release itself and
[[icccer-release-range-is-not-the-dispatch-list]] for the tag-object trap
that sits beside it.
