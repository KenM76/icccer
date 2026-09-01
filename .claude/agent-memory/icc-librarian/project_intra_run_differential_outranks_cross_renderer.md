---
name: iccce-intra-run-differential-outranks-cross-renderer
description: DL-077 GENERALISES this — a differential subtracts everything its two arms SHARE, non-selectively; shared observation path = the method's whole value, shared MECHANISM UNDER TEST = a test that cannot fail for the reason it is named after; so ENUMERATE WHAT THE TWO ARMS SHARE before trusting one. DL-074 — an equality the TEST MATERIAL'S AUTHOR asserts, measured between two objects of ONE render, beats a comparison between two renders because the uncontrolled path cancels; but it is ONE-SIDED (failure yes, conformance no), and an OBSERVED failure corroborates a classification without attributing a LAYER. ★ SETTLED by DL-076: the class survives INTACT, its carve-out paragraph is NARROWED — cancellation licenses the COMPARISON, never the UNIT, so a capture-derived result is stated in 8-bit counts.
metadata:
  type: project
---

**`DL-074`, 2026-08-28. When the test material's own documentation asserts
that two objects must resolve to the same colour, measure the distance
BETWEEN THOSE TWO OBJECTS INSIDE ONE RENDER — never between two renderers'
renders.** New evidence class **`intra-run-fixture-invariant`**
(`NUMERIC_CLAIMS.md` §1), pointer section §3.40, operational form
`TOLERANCES.md` §1.2.

**Why:** a cross-renderer screenshot comparison carries a variable neither
image discloses — whether each application colour-manages to the display
profile — plus scaling if the captures differ in size, and nothing in the
pixels subtracts it out. Taken **inside one render**, both sides crossed the
identical display path, scaling, quantisation and table, so **everything
uncontrolled is common-mode and cancels.** And the expectation is a **third
party's**, asserted before anyone measured, so there is **no oracle in the
claim at all** — which is what puts it above `implementation-cross-check`.
The other renderer is a **CONTROL** (evidence the contract is satisfiable),
and ★ **the finding survives the control's removal** — the sharpest test of
whether an oracle was load-bearing.

**How to apply:**

- ★★★ **Never quote the class without its bound: IT IS ONE-SIDED.** A
  **large** divergence is robust — no shared path pushes two equal inputs
  apart. A **near-zero agreement is NOT** — a shared path can clamp,
  saturate or quantise two genuinely different colours onto one value. ⇒
  **State a FAILURE of the declared relation; never certify that it HOLDS.**
  That is `DL-069`/`DL-070`'s clamp argument inside one render, and
  [[iccce-agreement-can-be-the-symptom]] in a new place. **The direction a
  demonstration usually hopes for is the weak one.**
- ★★★ **DECIDED THE SAME DAY, `DL-075`: `DL-070` REACHES a ΔE between TWO
  DIFFERENT captures** — the prohibition attaches to the **ARTIFACT**, not to
  what sits opposite it (§1's class names *pixels read out of a capture* in
  its head clause; the `iccce` side is only the *"Here:"* instance). Two
  captures = two unknown display legs, nothing cancels, reason 1 applies
  *harder*. **Nothing widened, nothing exempted; decided by `icc-engineer`,
  no operator words carried — sufficient in the contraction direction only.**
  The escalation was still right: *widening a prohibition and exempting from
  it are both DECISIONS, not filings* — refuse and record, and this is what
  the record produced. See
  [[iccce-prohibition-buys-nameability-not-prevention]] and
  [[iccce-fitted-table-cannot-express-a-clamp]].
- ★★★ **SETTLED THE SAME DAY, `DL-076` (reading 2): `DL-070`'s reason 1 is
  about the UNIT, so it reaches EVERY ΔE with a capture pixel on either side —
  INCLUDING an intra-render one.** `icc-engineer`: *"a capture-derived
  quantity is stated in 8-bit channel counts whoever is on the other side of
  it, and however well the display path cancels."* ⇒ **This entry's class,
  its two properties, its ranking, its one-sidedness and its bound SURVIVE
  INTACT and lose no result** (they answer *which comparison is admissible*
  and none was ever in ΔE); **only the carve-out paragraph is NARROWED** (it
  answers *which unit*). ★★ **The reason, and it is what generalises: a
  display transform is NON-LINEAR, so it CANCELS IN THE DIFFERENCE WITHOUT
  PRESERVING THE DISTANCE.** Cancellation is an **attribution** argument — the
  discrepancy is the renderer's, not the observation's — and says **nothing
  about size**; an intra-render ΔE measures a distance in the **display's**
  space and reports it as a perceptual distance in the **PCS**. ★ Reason 1
  *reads* as an asymmetry (*"one side is not a colour"*) but two capture
  pixels are not two colours either — **symmetry removes the imbalance, it
  does not supply the missing colours.** ★ **The bound needed no metric: it is
  ORDINAL** (*far apart* / *coincide*), which is why it was never at risk.
  ⇒ **State a capture-derived intra-render result in 8-bit channel counts. A
  ΔE stays lawful only where NO capture is in the path** — every stage run and
  pinned by this project, which is what this entry already demanded for any
  result in the *satisfied* direction.
- ★★★ **GENERALISED 2026-08-28 by `DL-077`, and this entry is one of its two
  poles.** **A differential subtracts everything its two arms SHARE, and the
  subtraction is NOT SELECTIVE** — it removes nuisance terms and terms under
  test alike and cannot tell them apart. ⇒ **Shared = the uncontrolled
  observation path** ⇒ this entry: cancellation is the method's entire value.
  **Shared = the MECHANISM UNDER TEST** ⇒ the differential is **incapable of
  failing for its stated reason**, and is green under a correct
  implementation, a wrong one, and **none at all**. ★★ **Operational rule,
  checkable rather than exhortatory: BEFORE TRUSTING A DIFFERENTIAL,
  ENUMERATE WHAT ITS TWO ARMS SHARE. Anything shared is outside its reach.**
  The enumeration is the **deliverable** — a written list can be wrong,
  incomplete and re-checked; *"be careful with differentials"* can be none of
  those. ★ Five instances, and **the escalation in evidence class is the
  finding within the finding**: `DL-059` argued from **clause text**, a later
  row from **readme prose**, the newest from **the artifact's own internal
  references** — a clause assignment is a *reading of a standard*, but
  cancellation read off a document's own structure is **a property of the
  document** ⇒ **applicable BY INSPECTION, not BY ARGUMENT.** ★ Third face:
  **fixture DESIGN** — a fixture whose two arms are copies of each other is
  zero-separation and green for the wrong reason, and **the remedy is a LOWER
  BOUND asserted in the test**, never care taken while authoring.
- **Where WE author both objects the class collapses to `self-consistency`.**
  The whole promotion rests on the assertion being somebody else's. (The
  sibling project's own same-value-two-routes technique is exactly that
  weaker, still-useful case.)

**★★★ The second lesson, and it was not in the dispatch: AN OBSERVED
FAILURE CORROBORATES A CLASSIFICATION AND CANNOT ATTRIBUTE A LAYER.**
`SUITE_COMPATIBILITY.md` §3.1's surviving rows owe a **clause-attribution**
test — *"name the clause and the standard that assigns the behaviour to a
layer."* Watching a consumer that lacks capability C exhibit the predicted
failure is **an observation, not that test**. `DL-059` is the proof they
differ: **a device-space routing defect and an absence of colour management
produce the same visible failure on the same page**, which is exactly how
that table's withdrawn row came to be misclassified. ⇒ Record such a result
as **one row of five, corroborated by observation** — never as the table
vindicated — and **leave §7.20 owed for all five, including the observed
one.** See [[iccce-patch-named-for-what-it-looks-at]] and
[[iccce-compatibility-not-certification]].

**★★ A rule-11 observation raised to the operator and NOT acted on.**
`NUMERIC_CLAIMS.md` §3.39.3 states *"Those numbers are not reproduced here"*
and, two sentences earlier, **reproduces that census's findings in words**.
No integer appears, which is presumably how it passed. **Nothing was
edited** — rule 11 says what is already there stays, and a librarian does
not rewrite a published passage on its own reading. It matters because it
decides **how tightly rule 11 binds the next filing**: if a *qualitative*
finding counts as a measurement, the boundary is tighter than the ledger has
been applying. ★ **Write to the tighter reading** — §3.40 carries no finding
of any kind and is safe under either. Adjacent to §7.28 item 6; filed as
§7.30 item 4. See [[iccce-producer-census-is-not-a-consumer-fact]].

**★ Mechanical note for anything written into this repository, including
these memory files:** `tools/check-suite-name-absent.py` forbids two
base64-encoded terms in **file contents AND file names**, across tracked
**and untracked-but-not-ignored** files — so **this agent-memory directory
is in scope** (its own docstring records two memory notes turning CI red on
2026-08-25). The librarian cannot run it; apply it by hand.
