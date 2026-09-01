---
name: iccce-stale-status-decays-faster-than-a-number
description: DL-062 (2026-08-18) — a document said "not fixed here, deliberately" about a defect fixed 28 s earlier; a stale STATUS decays faster than a stale number because the fix lands in someone else's commit. NOW EIGHT INSTANCES: #4/#5 are consecutive handoffs carrying a wrong PUSH COUNT ⇒ DL-068 rules the field itself wrong (carry the two REF HASHES); #6 is a "next free ids" line naming a DL its OWN filing had just consumed — FALSE AT THE INSTANT OF WRITING; ★★★ #7/#8 (2026-09-01) both came from OUTSIDE — pdfce flagged a README "In progress" row TWICE across two files, and #8 is a version line that NAMES ITS OWN VERIFICATION METHOD AND DATE and went stale anyway, which is why "re-verify harder" is not the fix. ★★★ AND THE REVERSE FAILURE (2026-09-01, later): DL-068's run-id form was discharged BY REQUEST and the VERY NEXT CI observation omitted the databaseId — a field fixed by asking is not a field enforced
metadata:
  type: project
---

**A stale *status* is a distinct and faster-decaying failure than a stale
*number*, because the event that invalidates it happens in someone
else's commit.** File the superseded observation and the current one as
**two rows**, each dated, with the fixing commit named — never edit the
old numbers away, and never let the ledger's correctness depend on
another agent's document being fixed.

**Why:** at the Pass K filing (2026-08-18, tip `60c32dd`)
`docs/TOLERANCES.md` §3.10.12.7 read *"**Not fixed here, deliberately.**
The remedy is a `crates/` change and belongs to the engineer"* and *"★
what this role owes when it is fixed: a row"*, and presented
`0.617121` / `0.617148` in a table headed as a measurement. **The fix had
landed in `a05476c` — the commit immediately BEFORE `a1bd818`, the
grading commit that filed it as open.** The author of the section was not
careless; the world changed under it between two adjacent commits.
`icc-conformance` caught it re-running everything and **contradicted its
own agent memory** to say so.

The asymmetry that makes this worse than a stale numeral: **a wrong
number invites re-derivation; a wrong status invites a reader to act on
an obligation already discharged, or to distrust working code.** It is
DL-048's carrier mechanism (*a wrong pointer authenticates its
destination*) applied to a **verb tense**.

**How to apply:**

- When a dispatch reports a defect as open, **check whether it is still
  open in the tree**, not just whether the numbers reproduce. Commit
  ORDER is checkable without a shell — `.git/logs/HEAD` is plain text and
  records adjacent commits with their messages
  ([[iccce-git-files-readable-without-shell]]). Say plainly that this is
  a statement about a file's contents, not about `git status`.
- **Two rows, not one edited row.** NC-265 carries the pre-fix numbers
  dated `2026-08-18 (pre-a05476c)`; NC-264 carries what is true now. The
  ledger is then correct **whatever state the other document is in**, and
  §7's owed item can name the correction without the filing depending on
  it.
- **Do not edit another agent's file to fix it**, especially while they
  are in it — file the owed item and say who owns it. `TOLERANCES.md` is
  `icc-conformance`'s.
- **Split a paired claim when one half retires.** *"`O(1)` beside
  `O(h^1.32)`"* — the `O(1)` half was the **defect's signature and is
  gone**; `O(h^1.32)` is Pass 6's still-live measured order (DL-025,
  NC-149). Quoting the pair keeps a dead number alive inside a live one.
- **Superseded numbers in a doc comment or test that SAY "under the
  defect" are correct usage** — that is how a retired number should
  appear. The defect is a number presented as current. When sweeping,
  distinguish the two rather than counting occurrences
  ([[iccce-count-from-a-sample-is-not-the-population]]).

**★★★ FILED AS DL-062 on 2026-08-18, WITH A SECOND INSTANCE — and the
second is what makes it structural.** The mechanism reproduced **inside
the same session, with the roles reversed**: this librarian read
`TOLERANCES.md`, correctly filed `NUMERIC_CLAIMS.md` §7.23 item 1 as
owed, and `icc-conformance` discharged it **concurrently** — so the
ledger's own §7 prose became a stale status claim about another role's
file, inside the filing that documented the first instance. ★★ **It is
not about haste: the second happened under full awareness of the
first.** It is structural to the ownership rule — *no role edits
another's file* guarantees every cross-role status claim is a claim
about a tree its author cannot fix.

**★★★ THE CONSTRUCTIVE HALF, which is the part to actually apply:
re-checking before quoting is INSUFFICIENT** — the claim can go stale
**between being written and being committed**, and that is the interval
both instances died in. So **either re-verify at commit time, or write
the filing to be correct regardless of the other file's state.** A
**row** is a claim about a *measurement* and does not change under
another agent's commit; a **§7 owed item** is a claim about a *state*.
★ **Prefer to discharge a doubt with a row.** Where a §7 item is needed,
write its *done when* clause naming **checkable text**, so a third party
can adjudicate it in one reading (§7.23 item 1 did, and that is why the
discharge was clean).

**Two things verified at the DL-062 filing, worth reusing:**

- **The 28 seconds are MEASURABLE without a shell** — `.git/logs/HEAD`
  epochs `1787035205` / `1787035233`. But **a reflog line never
  evidences a commit's CONTENTS**: "the fix landed in `a05476c`" stays
  `[REPORTED]`; what a librarian can verify is **the tree at the tip**.
- **The stale text survived FOUR subsequent commits**, two of them
  documentation commits, and was found only by a deliberate currency
  re-check. Use that as the evidence, not a claim about what anyone's
  review covered — that part is a reading.

★ **Carrier phrases to grep at every currency check:** *"not fixed
here"*, *"belongs to the engineer"*, *"when it is fixed"*, *"is
measurably wrong"*, *"has no implementation"*, *"needs a `crates/`
edit"*. One (`KMapping::Ratio` *"has no implementation"*) was swept and
**survived** — `transform.rs` refuses the variant, and a refusal is not
an implementation. **A phrase list that has only ever fired is untested
in the passing direction.**

★★ **And a citation drifted through two documents.** The DL-062 draft
glossed DL-053 as *"`[VERIFIED]` certifies a date, not an evidence
class"*; DL-053's actual rule is the **denominator** axis. The gloss
came from **this librarian's own Pass K `SESSION_LOG` entry**, which
extended DL-053 without labelling the extension. **Cite the AXIS, not
just the DL number** — three readings of `[VERIFIED]` are now in play
and only one is DL-053's.

**★★★ UPDATED 2026-08-20 — THE "REVISIT IF A THIRD INSTANCE APPEARS"
CLAUSE HAS FIRED TWICE, AND THE RECURRING CARRIER IS ONE ROW OF ONE
DOCUMENT. Filed as DL-068.**

| # | date | the claim | how it went stale |
|---|---|---|---|
| 1 | 2026-08-18 | `TOLERANCES.md` *"not fixed here"* | fixed **28 s** earlier |
| 2 | 2026-08-18 | `NUMERIC_CLAIMS.md` §7.23 item 2 | discharged **concurrently**, while writing up #1 |
| **4** | 2026-08-18 | `NEXT_SESSION.md` *"14 commits. NOTHING IS PUSHED"* | the operator pushed |
| **5** | 2026-08-19 | `NEXT_SESSION.md` *"ahead of `origin/master` — **4 commits**… **Measured**"* | the operator pushed; it is **`0`** |

★★★ **Instance 5 was carried by the document that had just recorded
instance 4, in the row directly below it, with the remedy stated in its
own text** (*"Do not carry a push-state claim forward without re-running
the count"*). **The count WAS re-run. It was right when written. It went
stale anyway.**

> **A field that goes wrong twice in a row under full awareness is not
> being filled carelessly. It is the WRONG FIELD.**

**Why a push count specifically fails BOTH of DL-062's constructive
clauses:** its subject is **the one tree no agent may touch** (pushing is
the operator's act, rule 9), so it changes exactly when nobody here is
looking; the interval it must survive is **the longest in the project**
(written at session end, read at the next session's start — which is when
the operator pushes); and ★★★ **nothing in any handoff DEPENDS on it** —
pushing is unauthorised regardless. **No decision, highest staleness
risk: the worst possible ratio.**

**THE DECISION (DL-068): a handoff records the two REF HASHES, not a
derived count.**

```text
local  refs/heads/master           0a88ad6…
remote refs/remotes/origin/master  0a88ad6…   (as of the last fetch/push this repo observed)
```

★ **A hash is a dated READING** — when the world moves it becomes
*historical* and says so by not matching what the reader finds. **`4` and
`0` look identical in authority.** Same move as NC-264/NC-265 surviving a
concurrent correction by being dated measurements with commit anchors.

**Three cheap supporting rules:** state the **method** beside the hashes
(loose ref files vs `git rev-list --count` — they fail differently);
**never write *"nothing is pushed"* as a standalone sentence**, which is
the form both instances decayed into; and **add to the grep list**:
`ahead of origin`, `NOTHING IS PUSHED`, `unpushed`, `commits this
session`.

★★ **Caveat that must travel with the hashes:** `refs/remotes/origin/master`
records **what the last fetch or push observed**, **never** the remote
server's present contents. It supports *"`rev-list --count` returns 0"*;
it supports **no** claim about publication (rule 9).

★ **DL-068 does NOT build the mechanical guard DL-062 imagined.** It
removes **one recurring carrier** by changing what gets written. A
pre-commit phrase-grep for cross-role status claims is still unbuilt and
unowned, and must not be read as done.

**★★★ UPDATED 2026-08-29 — A SIXTH INSTANCE, AND IT HAD NO TRUE INTERVAL
AT ALL.** Found at a `NEXT_SESSION.md` handoff refresh, by verifying the
id counters against live source rather than taking them from the
dispatch.

| # | date | the claim | how it went stale |
|---|---|---|---|
| **6** | 2026-08-28 | `NUMERIC_CLAIMS.md` §7.33's closing *"Next free ids after this filing: `NC-308`, `NA-013`, **`DL-077`**"* | **`DL-077` was filed BY that filing** — `ARCHITECTURE.md`'s `DL-077` header block reads *"Fourth filing of this date"*. Next free was `DL-078` before the ink dried |

★★★ **Every earlier instance was true when written and decayed.** This one
was **false at the instant of writing**, by the hand that had just consumed
the id, in the sentence whose only job is to say which ids are free. **The
interval it had to survive was zero and it still failed.** That is the
tightest form the mechanism has taken here, and it means DL-062's
constructive clause (*"re-verify at commit time"*) would not have caught
it either — nothing decayed.

**The carrier is the same shape as the push count:** a **derived counter
written at the END of a filing**, about state the filing itself moved.

**How to apply — cheap and mechanical:**

- ★ **Write the "next free ids" line LAST, and derive it by reading the two
  live documents**, never by incrementing what the previous filing said.
  `grep '^### DL-0' ARCHITECTURE.md | tail -1` is the check, and this role
  can do it without a shell.
- ★★ **A filing that adds a `DL-` must move the `DL-` counter.** If a filing
  claims *"no `NC-` row added"* — true and common — that is **not** a
  licence to copy the whole id line forward; the three counters move
  independently.
- **Add to the grep list:** `Next free ids`, `next free`, `is last; next`.
- ★ **Correct in place with a strike-through and a dated note, not a silent
  rewrite** — the shape is the record, and the cost of *not* correcting it
  is that the next filing takes the same id twice. **No `NC-` row, no §7
  status pass** (`DL-058`: opening one inflates an integer §7 tracks).

**★★★ UPDATED 2026-09-01 — INSTANCES #7 AND #8, AND BOTH CAME FROM OUTSIDE
THIS PROJECT.** Found at the `--bpc` no-op filing.

| # | date | the claim | how it went stale |
|---|---|---|---|
| **7** | ≤2026-09-01 | `README.md`'s Pass 4 row — *"**In progress** — saturation in B2A, and the ICC-absolute intent, remain"* | the work finished at Pass 4c; `ROADMAP.md:3556` had said *"★ MET AT LAST"* for weeks. ★★★ **`pdfce` flagged it TWICE, across two files**, before anyone here corrected it. Now reads `Done` *(verified)* |
| **★★★ 8** | 2026-09-01 | `README.md:127` — *"Version **`0.1.0`** — read from the workspace manifest on 2026-08-21, **which is the authoritative source**"* | the `0.2.0` bump. `Cargo.toml:40` reads **`version = "0.2.0"`** *(verified)*. **Still wrong at this filing**; owed to `icc-engineer` |

★★★ **Instance 8 is the sharpest demonstration this entry has produced that
`DL-062`'s constructive clause is insufficient. The sentence NAMES ITS OWN
VERIFICATION METHOD AND ITS OWN DATE and still went stale** — because what
invalidated it was a version bump in **somebody else's commit**, which is
exactly the mechanism the entry describes. *"I checked this, on this day,
against the authoritative source"* is **not** a defence; it is a **dated
reading**, and the fix is to say so, not to re-check harder.

**★★★ UPDATED 2026-09-01 (later still) — `DL-068`'s FIELD LESSON RAN IN
REVERSE, ONE FILING AFTER IT WAS DISCHARGED.** Not a ninth staleness
instance — **the opposite failure**, and it belongs here because it is
about the same field.

`§7.35` newly-owed 2 asked, on `DL-068`'s reasoning, that a CI-green
claim **carry the run id**. It was discharged at `§7.36.1` with two ids.
**Hours later the next CI observation arrived without one**: the listing
that finally covered the repository tip requested
`name,status,conclusion,headSha,createdAt` — **`databaseId` was not among
them** — so the only run in this project's record that covers the tip is
the only one with **no re-checkable address**.

> ★★★ **A field that has just been fixed by request is not a field that
> is enforced. `DL-068` changes what gets WRITTEN; it builds no guard, and
> the very next observation skipped it.**

**How to apply:** when a filing discharges an omission **by asking for
it**, record the *form* as owed process in a place the next actor reads
(here: a required column in `ROADMAP.md`'s release table, `§7.36.4`
item 2) — and expect the next instance to miss it anyway. ★ **The check
is one line: does the claim carry an id, a hash or a date I could hand to
someone with a shell?** If not, it is *"CI is green"* wearing a longer
sentence.

★★ **A version line is a WORSE carrier than an ordinary stale number**,
because a version is what a reader uses to decide **which documents apply
at all**. A wrong ΔE misleads about one claim; a wrong version misleads
about the whole document set.

★★★ **Instance 7's real content is WHO FOUND IT.** Two of this project's
own roles read `README.md` regularly; **the consumer found it, and had to
say so twice.** ⇒ **add the consumer's flags to the currency check**: if an
outside reader has reported a staleness, treat the *class* of claim as
suspect across the whole document, not just the line reported. (Had that
been done at the first flag, instance 8 was three lines away and would have
been caught in the same pass.) Related:
[[iccce-consumer-exercises-paths-the-code-is-in]].

★ **New carrier phrases for the grep list**, both from this pair: `which is
the authoritative source`, `read from the ... on 2026-`, `Version \`0.`,
and any status cell containing `In progress`.

**★★★ UPDATED 2026-09-01 (later still) — DL-068'S RULE GENERALISED TO CI,
AND IT WORKED: THE ASK WAS MADE BY NAME AND THE EVIDENCE ARRIVED.** The
`v0.3.0` release filing refused *"CI is green"* and filed §7.35
newly-owed 2 asking for **the run id**, on exactly DL-068's reasoning —
***`success` and `failure` look identical in authority once the sentence
is a month old***, the way `4` and `0` did. A follow-up dispatch supplied
`CI` **`33489492338`** and `Release` **`33489492467`**, both `success`.

- ★★ **The claim stays `[REPORTED]`** — a run id read out of a dispatch
  is not a run observed. What retires is *"has no durable form"*: an id
  is a **re-checkable address** a future reader with a shell can settle
  in one command. **An address is not a corroboration.**
- ★★★ **The id and the verdict came from ONE `gh run list` observation**,
  which the dispatch disclosed itself. `DL-077` applies — enumerate what
  the two arms share; here they share the whole path, so a misread
  listing makes both wrong together.
- ★★★ **CARRY THE COVERAGE WITH THE ID.** The `CI` run is on the release
  commit `a4d9003`, **not on the tip** — the tip was pushed 2 min 29 s
  after those runs started (reflog epochs, readable without a shell:
  [[iccce-git-files-readable-without-shell]]). Write **"CI concluded
  `success` on `<commit>`"**, never *"the repository is green"*.
- ★ **What made this cheap:** the ask named the artefact (`the run id`),
  not a quality (`better evidence`). Compare `DL-079` — the two `v0.2.0`
  holds were never re-asked and the record still has nothing.
  `v0.1.0`/`v0.2.0` **can never get an id retroactively**, which is the
  concrete cost of the years the field went uncarried.

Related: [[provenance-is-a-different-axis-from-evidence-class]],
[[iccce-pass-status]],
[[iccce-artifact-existence-is-not-obligation-status]] (an artifact's
existence is not the obligation's status — this is its twin: a
*document's* claim about status is not the status),
[[iccce-stale-citation-worse-than-stale-number]],
[[iccce-gate-must-not-reward-deletion]].
