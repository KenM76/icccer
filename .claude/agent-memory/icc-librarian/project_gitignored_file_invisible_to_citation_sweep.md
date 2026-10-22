---
name: icccer-gitignored-file-invisible-to-citation-sweep
description: My Grep tool honours .gitignore on a directory walk, so a citation sweep of docs/ silently MISSES docs/SUITE_COMPATIBILITY.md — 9 hits reported where 13 existed. It under-counts in the direction that produces a false all-clear. Search that file BY NAME, every time.
metadata:
  type: project
---

**A directory sweep with the Grep tool does NOT search
`docs/SUITE_COMPATIBILITY.md`, because that file is gitignored and
ripgrep skips ignored files when it walks a directory.** Naming the file
explicitly searches it normally.

**Why:** the operator untracked that document on **2026-08-18** (licensed
third-party suite, affirmative-notice + commercial-context terms). It is
still on disk, is still the suite workstream's narrative in full, and is
**cited by section from the tracked documents** — so it is simultaneously
one of the project's most citation-dense files and invisible to the
default search.

**Measured, 2026-08-25:** a sweep for `shading_cell` over `docs/`
returned **9** hits. The true count was **13** — the missing four were
all in `SUITE_COMPATIBILITY.md`. A shell `grep -rn "..." docs/*.md`
expands the glob and finds all 13; the discrepancy is entirely the tool's.

★★★ **The reason this matters more than an ordinary tool quirk: it
under-counts in exactly the direction that produces a FALSE ALL-CLEAR.**
*"Zero remaining citations"* is a sentence this role writes at almost
every filing. The failure mode is a clean sweep that missed a whole file
— and nothing about the output says so.

**How to apply:**

- **Any** sweep, audit, or "confirm the count is zero" task: run the
  directory sweep **and** a second search naming
  `D:\Dev\iccce\docs\SUITE_COMPATIBILITY.md` explicitly. A sweep that did
  not name it is **incomplete**, and say so if you cannot run both.
- The same applies to anything else `.gitignore` covers that still holds
  prose — check `.gitignore` before trusting a repo-wide count.
- ★ **Report the method with the number.** *"13 across four documents,
  found by a directory sweep plus a named-file search"* is a claim a
  reader can check; a bare *"13"* is not.
- Sibling risk in the same family: a count that looks like an inventory —
  see [[icccer-count-from-a-sample-is-not-the-population]] and
  [[icccer-count-needs-its-apparatus]].

Related: [[icccer-licensed-tool-split-outside-the-tree]],
[[icccer-stale-citation-worse-than-stale-number]],
[[icccer-pass-status]].
