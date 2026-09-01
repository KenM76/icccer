---
name: run-the-conformance-demonstration-not-just-the-diagnosis
description: When a consumer render looks wrong, Ken wants the measured proof that iccce does or does not fix it — not only the diagnosis of whose bug it is
metadata:
  type: feedback
---

When Ken brings a rendering that looks wrong, **diagnosing whose defect it is
is only half the answer he wants. Run the demonstration that measures whether
iccce actually closes the gap.**

**Why:** 2026-08-28 he sent an Acrobat/`pdfceGUI` pair of a customised
print-conformance page. I answered with the diagnosis — the failure is
`pdfce`'s missing CMM, iccce is not even linked into that render path — and
his reply was *"If it would help and you can do it, please test your
conformance."* ★ He was not asking for reassurance; the demonstration is what
he considered the deliverable. It also turned out to be where the value was:
it produced the fixture gap, a Tier-A reclassification, and the finding that
**iccce fixes almost none of two of the four panels** — which is the opposite
of what the diagnosis alone implied and is the part that changes a consumer's
planning.

**How to apply:** when a consumer-facing colour question arrives, dispatch
`icc-conformance` to convert the case's *own* values through the *real*
profiles **at the same time** as answering the attribution question — do not
wait to be asked. Report the number even when it goes against this project's
interest; "we fix ~8 % of this one and essentially none of that one" is the
finding, not a disappointment. See [[fixture-separation-and-cancelling-rivals]]
for the trap that makes a clean demonstration untrustworthy, and
[[suite-compatibility]] for where suite-derived numbers may live.
