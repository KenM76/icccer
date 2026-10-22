# icccer — project instructions

A from-scratch, MIT-licensed **ICC colour management module** in Rust.
Read `README.md` for scope, `docs/ARCHITECTURE.md` for shape,
`docs/ROADMAP.md` for the plan, and `docs/NEXT_SESSION.md` before doing
anything.

The global rules in `C:\Users\Ken\.claude\CLAUDE.md` apply — especially
documentation-first, dispatch-agents-freely, and the personal_rag
lesson-writing discipline. This file adds what is specific to colour.

## Project agents

| Agent | Role |
|---|---|
| `icc-engineer` | Lead engineer. **Be this agent** if you are orchestrating; read its file at session start. |
| `icc-spec-librarian` | Builds `D:\Dev\Rag-Specialized\ICC_Spec\`. Dispatch for EVERY sourcing question. |
| `icc-conformance` | The oracle, the fixtures, the tolerance budget. Owns "how do we know?" |
| `icc-librarian` | ROADMAP / SESSION_LOG / decision log / the numeric-claims ledger. No shell — dispatches must carry evidence. |

## The rules that are specific to this project

### 1. A wrong colour looks exactly like a right one

This is the whole reason the discipline below is heavier than it would be
for a parser. Nothing about a 3 ΔE error announces itself. Assume every
plausible-looking result is unverified until measured.

### 2. Never write colour maths from memory

Adaptation matrices, transfer-function breakpoints, Lab encodings — all
things one half-remembers correctly. Dispatch `icc-spec-librarian` and
cite the standard and clause in the doc comment.

### 3. Expected values come from the literature

A test whose expectation came from the code under test detects change,
not error. Where no published value exists, use lcms2 and label the
expectation a **cross-check against another implementation**, which is a
weaker claim than ground truth and must not be written as though it were
the same.

### 4. Every approximation is named and measured

A CMM is a stack of interpolations. State what each departure from exact
colorimetry is and what it costs in ΔE. An unstated approximation is
indistinguishable from a bug.

### 5. Tolerances are justified, not tuned

"Within 1 ΔE2000 because that is the accepted perceptibility threshold"
is a tolerance. "Within 0.5 because it passed" is a number someone moved
until the suite went green. When a test fails, the first question is
whether the code is wrong.

### 6. The parser reports; it does not repair

A silently corrected tag hides the malformation from the only layer that
could disclose it.

### 7. Disagreement with lcms2 is a finding, not a failure

lcms2 is an implementation, not the standard. When the two differ, settle
it from the specification text and record the outcome — a case where
icccer is deliberately right and lcms2 is wrong is worth writing down.

### 8. Optimise only after correct

A fast wrong answer is harder to fix than a slow one, because the speed
becomes load-bearing before anyone checks the arithmetic.

### 9. MIT, and publishing is the operator's act

Classify every dependency before adding it. Never push, tag or release
without an explicit current go-ahead.

### 10. ★ FIRST, EVERY SESSION: check the request channel

**`D:\Dev\FeatureRequests\icccer_FeatureRequests\open\`**

List that directory at the start of every session, before anything else.
It is the communication channel between this session and the **`pdfce`**
session at `D:\Dev\pdfce\` — a PDF engine with no colour management at
all, and the consumer this project's own `README.md` names first. Created
by the operator 2026-08-17. Neither session can ask the other a question
in real time; this folder is how they do it asynchronously.

- **`open/` empty = nothing is owed.** That is the whole check, and it is
  cheap.
- **`INDEX.md` is the memory** — one row per *closed* exchange, naming
  where in a repository the durable answer lives. Grep it; do not read
  `archive/` unless a row points there.
- **`README.md` in that folder is the contract.** Read it before writing
  anything into the channel.

Four things about it that are load-bearing and will otherwise be
discovered late:

1. **Requests flow BOTH ways.** Unlike the GUI channel `pdfce` also runs,
   this one is not a one-way queue. **`icccer` may write
   `open/request_<topic>.md` asking `pdfce` things** — *"what shape does a
   PDF actually hand you an ICC profile in?"*, *"how many components does
   a real `DeviceN` carry in the wild?"*, *"is this API callable from a
   per-pixel loop?"*. A consumer's real usage is **the best available
   check on this library's API shape**, and that check only works if this
   side asks. Not asking is the failure mode, not a courtesy.
2. **`pdfce` has hard gates that constrain anything it adopts from here.**
   It must build for **`wasm32-unknown-unknown` (CI-enforced)**; it takes
   **no copyleft anywhere in a dependency tree** (it is MIT, same as this
   project); and **`pdfce-core`/`pdfce-render` must never gain a network
   client under any future decision**. icccer satisfies all three today —
   the whole `Cargo.lock` is five packages, all of them ours — but that is
   a *dated observation, not a standing guarantee*. Classify against those
   three before adding anything, not just against rule 9.

   ★★ **CORRECTED 2026-08-25: this rule used to say "icccer does not gate
   wasm32 in CI", and that was FALSE.** `.github/workflows/ci.yml` has a
   `wasm32` job that builds all four library crates for
   `wasm32-unknown-unknown` on every push to `master`, and a companion gate
   asserting every line of `cargo tree` is an `icccer-*` crate at a local
   path. Both landed in `68d4601`, **before `v0.1.0`** — so the sentence was
   already wrong when it was written here. ★ It was also **repeated to
   `pdfce` in the request channel as a caveat about this library**, which
   is the part that makes it worth a paragraph rather than a deletion: a
   false caveat *understates* what a consumer can rely on, so it is the one
   direction of error that never gets challenged by the person it misleads.
3. **★ Overprint is `pdfce`'s problem, not this project's.** It is
   **compositing, not conversion** — it decides which colorant channels a
   paint operation writes to, in a device space, *before* any conversion
   to a display. This is the single row of the channel's boundary table
   most likely to be mis-filed, and
   `open/note_boundary_and_overprint.md` explains why. icccer supplies the
   conversion at the *end* of that pipeline and owns none of the
   compositing in front of it. The dependency runs one way: **icccer's half
   unblocks `pdfce`'s overprint work, and none of the overprint work is
   icccer's.**
4. **`pdfce`'s shipped `DeviceCMYK`→sRGB table is fitted to *pdfium*
   output** (`crates/pdfce-core/src/color/cmyk_table.rs`) — a 6×6×6×6
   lookup obtained by rendering patches and measuring them. That makes it
   a **cross-check against another implementation, not ground truth** —
   this project's own rule 3, applied to `pdfce`'s existing work. It
   matters because **replacing it with icccer is a *lateral* move in
   evidence class**, since icccer's oracle for a CMYK LUT path is lcms2,
   also a cross-check. Adopting icccer is defensible as a **conformance**
   gain (the document's declared output intent gets honoured at all) and
   **not** as an accuracy gain. Do not let it be written as the latter.

**The channel folder is in NO git repository, deliberately.** Consequence,
and it is binding: **nothing may exist only there.** A reply is a
*pointer plus an executive summary*; the durable finding lands in this
project's own `docs/` in git — a measurement in `docs/NUMERIC_CLAIMS.md`,
a tolerance in `docs/TOLERANCES.md`, a behaviour in a test, an API in a
doc comment. **One topic per file.** And the channel's own extra rule,
which exists because of rule 1 above: **a colour claim carries its
reference and its number, and names its oracle** — ground truth,
cross-check against another implementation, and self-comparison are three
different strengths of claim in that folder exactly as they are here.

### 11. ★★★ NO NEW SUITE MATERIAL IN A TRACKED FILE — operator decision, 2026-08-25

**Ken's words: *"ship it and stop adding suite material."*** Effective
immediately, binding on every session.

**What the decision settles.** The print-conformance suite is licensed,
non-redistributable, carries a **no-commercial-use** clause and an
**affirmative obligation to reproduce the suite publisher's notice** in any substantial
copy — the most restrictive terms of the four corpora this project touches.
`docs/SUITE_COMPATIBILITY.md` was untracked for that reason on 2026-08-18,
but `docs/NUMERIC_CLAIMS.md` kept accepting suite-derived rows, and that
document **is** tracked and **has been public on GitHub throughout**. The
decision does not undo that. It stops the growth.

**The rule, stated so it can be applied without re-deciding:**

- **Nothing suite-derived may be ADDED to a tracked file from now on.** Not
  a measurement, not a patch identifier, not a file name, not an
  indirect-object number, not a device colour read out of one of its
  documents, not the identity or `cprt` of a profile embedded in one, and
  not the suite's name in a newly-written passage.
- **What is already there STAYS.** It ships in `v0.2.0`. Removing it would
  mean rewriting published history, which the operator did not ask for and
  which would not un-publish anything.
- **suite work itself continues** — in `docs/SUITE_COMPATIBILITY.md`
  (untracked, still the reference), in `tools/suite/` for anything generic,
  and in `tools/patch-compare/` case files, which live **outside the
  repository** rather than in a gitignored path inside it.
- **Tracked documents may still POINT at the untracked ones** by section, as
  they already do. A pointer is not a quotation.
- ★ **`tools/patch-compare/patch_compare.py` is the shape to copy.** A tool
  that must handle licensed material gets split: a generic engine holding no
  numbers, plus a data file outside the tree. See `ARCHITECTURE.md` `DL-071`.

**The one thing that stays open and is NOT settled by this**: whether
published numbers from *other* licensed sources — ICC's informative-annex
values, CIE's CC BY-SA tables, a vendor `cprt` — may live in this
repository. `NUMERIC_CLAIMS.md` §7.15 newly-owed 6 has carried that since
2026-08-17 and it still blocks the second ground-truth fixture. **Do not
read today's decision as having answered it.** It answered the corpus with
the strictest terms, which is the one where the answer would have been "no"
anyway.

> ★ **Placed at 11 rather than 10 deliberately.** Inserting it *before*
> the request-channel rule would have renumbered that rule to 11, and
> **eleven passages across `ARCHITECTURE.md`, `SUITE_COMPATIBILITY.md` and
> `NEXT_SESSION.md` cite "rule 10" meaning the channel** — including four
> that cite a specific item number within it. Renumbering breaks a
> cross-reference silently: the citation still resolves, to the wrong rule.

## Cross-project knowledge bases

- `D:\Dev\Rag-Specialized\ICC_Spec\` — this project's standards corpus.
- `D:\Dev\Rag-Specialized\PDF_Spec\` — the PDF side. **Cross-reference
  §8.6, do not duplicate it.**
- `D:\dev\rag\rust\` — Rust/Cargo/packaging quirks that generalise.
- `C:\personal_rag\` — empirical lessons, all subjects.
