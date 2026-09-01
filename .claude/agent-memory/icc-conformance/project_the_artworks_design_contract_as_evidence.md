---
name: the-artworks-design-contract-as-evidence
description: A consumer document's own authored values outrank oracle and reference render; the triviality probe; and — read a patch's RESOURCE DICTIONARY first, because two arms sharing one colour-space object cancel the CMM exactly.
metadata:
  type: project
---

# A document's DESIGN CONTRACT outranks both the oracle and the reference render

**The fact.** When a test document is authored so that two differently
specified areas are *supposed to land on the same colour*, the numbers the
author baked into the page are recoverable from the page, and the
agreement between them is a **ground-truth-class** claim about that page —
stronger than "lcms2 agrees" and stronger than "a reference viewer renders
it this way".

**Why:** it is the page's stated design intent, not an implementation's
opinion. A reference render is a cross-check against another
implementation (this project's rule 3). An oracle is a cross-check against
another implementation. The author's own arithmetic is neither — it is
what the artefact *means*. When the engine recovers the authored value it
has reproduced the author's forward step, and there is no third
implementation in the loop to be wrong in the same direction.

**How to apply:** for any consumer-supplied document with an internal
consistency contract, **read the content stream, not the render.** Recover
the device values and the colour space each area declares, and close the
chain in the engine. Report *that* number as the finding and the reference
viewer's number as the weaker corroboration. Label both. Related:
[[stale-claim-strings-in-emitted-records]] on labelling discipline.

---

## ★ The probe that must run BEFORE you believe a closed round trip

**Ask: are the two profiles in the chain near-duplicates?** A source→PCS→
destination round trip between two profiles that are the same table with a
cosmetic difference will close to ~0 for reasons that have nothing to do
with the engine being right. Measure the distance between the two profiles
directly on a small probe set (paper white, single-colorant 50%, K-only
50%, a mid neutral). If they diverge substantially, the round trip closing
is informative. If they do not, say so and downgrade the claim.

**Why:** without it, "the chain closed to 0.4 ΔE00" and "the chain was an
identity" are indistinguishable from the outside, and the flattering
reading is the one that gets written down.

---

## ★★ The failure I actually committed and caught, in one run

I built a "how load-bearing is this assumption?" probe using, as the
stand-in for *some other profile*, **a profile that carried the very defect
the whole exercise was about.** It returned an enormous number. The number
was the defect, not the assumption. Re-running with a genuinely neutral
stand-in collapsed it by more than two orders of magnitude, and the
original conclusion was **retracted, not softened**.

**Why:** a control that shares the treatment is not a control. The tell is
that the probe's answer is the *same size* as the effect under study —
when a sensitivity probe returns roughly the headline number, suspect that
it is measuring the headline, not the sensitivity.

**How to apply:** when picking a stand-in, alternative, or rival, state
what makes it *independent of the defect*. If you cannot, the probe does
not run. Related: [[prove-the-arm-by-injecting-the-defect]] (the mirror
image — there, sharing the defect is the point).

---

## ★ A corpus can disclaim a criterion its own artwork still carries

A test corpus's stated pass/fail wording can declare a signal "not a
failure" while the artwork underneath still separates the cases by a large,
measurable amount. **The corpus's criterion being unusable does not make
the artwork uninformative.** Measure the artwork; report the corpus's
disclaimer as a fact about the corpus, not about the physics.

**How to apply:** when a document says "differences here are acceptable",
that is a licence for *implementations*, not an instruction to stop
measuring. Sweep the parameter the corpus disclaimed and report the
spread — it is often the cheapest discriminating fixture in the file.

---

## ★★★ Before believing a patch tests colour: read its RESOURCE DICTIONARY

**The test.** A conformance patch whose pass condition is *"these two areas
must match"* can only detect an error in a stage that is **not common to both
arms**. So the first question is not what the readme says the patch tests — it
is **which resources the two arms share**.

* **Both arms name the SAME colour-space object** ⟹ whatever the CMM does, it
  does to both, and it cancels **exactly**. Such a patch cannot detect a
  colour-management error *at all* — not a wrong transform, not a wrong
  intent, not the total absence of colour management. Whatever else varies
  between the arms (bit depth, filter, sample decode) is what it actually
  tests, and that is usually the consumer's.
* **The arms name DIFFERENT colour spaces** ⟹ the CMM is in one arm only, so
  the patch is informative about it — but it is also informative about every
  other difference, and it cannot separate them. Say how many variables the
  comparison carries before claiming it grades any one of them.

**Why this is worth its own rule:** this is `DL-059`'s argument recovered from
the *file* rather than from clause text, and it is cheaper and harder to argue
with. DL-059 needed four clauses to establish that a patch was not a CMM's
problem; one shared indirect-object number settles the same question in a
line, because *cancellation is a property of the document, not a reading of a
standard.*

**How to apply:** enumerate each arm's `/ColorSpace` (following the indirect
reference to its object number, not just its name), its `/BitsPerComponent`,
its filter, and its declared intent, **before** writing what the patch demands
of the engine. Two arms with one shared colour-space object number is a
finished argument. Then still apply DL-059's clause test — it tells you which
layer owns what remains.

---

## ★★ Two instrument errors of my own, both worth not repeating

**1. A device-space grid node is NOT a CLUT node.** To get an
interpolation-free probe of a LUT tag you need points that land on CLUT
lattice nodes *after* the per-channel input curves (`mft2` input tables, `mAB`
A-curves) have been applied. Feeding `k/(n−1)` device values assumes those
curves are the identity. When they are not, the "on-node" residual is not
zero and looks like an arithmetic disagreement when it is still geometry. The
device **corners** (all-0/all-1 combinations) are the probe that survives any
monotone curve, which is why the existing corner rows use them.

**2. A near-duplicate pair is free apparatus — go and look for it.** Producer
corpora contain the same profile embedded more than once with a byte or two
changed. Hash each file **with the fields the ICC profile-ID computation
excludes masked out** (the flags word, the header rendering-intent word, and
the profile-ID field itself); files that collide under that hash but differ as
bytes are a **perfectly controlled pair**, varying in exactly one header
field. That is the controlled experiment a synthetic fixture usually has to be
authored for, and it costs one pass over the corpus.


---

**Where the corpus-specific numbers live:** `docs/SUITE_COMPATIBILITY.md`
(untracked, per `CLAUDE.md` rule 11). Nothing measured on that corpus may
be added to a tracked file, and this memory directory **is** tracked —
which is why this entry carries method and no measurements.
