---
name: always-release-and-clean-build-artifacts-when-done
description: Ken's standing instruction (2026-09-01) — finish every work session by cutting a release, and delete Rust build artifacts from the project folder afterwards
metadata:
  type: feedback
---

**Two standing instructions, given together on 2026-09-01: *"release.
always release when you are done work. also clean up rust temp files in
your project folder when you are done."***

**The release half.** When a body of work is finished, do not stop at "the
tests pass and the docs are filed" and hand back a summary. Take it
through the whole release sequence: run every gate CI runs (fmt, clippy
`-D warnings`, rustdoc, tests, the wasm32 build and dependency-tree gate,
the name gate), bump the version at every co-located site, write the
CHANGELOG entry to this project's standard (why, and what the change does
*not* claim), commit in reviewable groups, tag, push, and watch the
workflow to green.

**Why:** work that sits uncommitted in a working tree is work a consumer
cannot use and a future session has to re-derive. `pdfce` is now a live
consumer pinning git revisions — an unreleased improvement is invisible to
it.

**How to apply:** treat a terse *"release"* as authorising the complete
sequence, per [[release-goaheads-are-terse-flag-before-irreversible]].
★ That memory still governs the exception: interrupt **once**, immediately
before the first genuinely irreversible act, and only if something
material changed since the go-ahead. A **crates.io publish is
irreversible** in a way a git tag is not — a published version can be
yanked but never removed, and the crate name is claimed permanently. Flag
that one; do not flag the tag or the push.

**The cleanup half.** After the release, remove Rust build artifacts from
the project folder — `cargo clean` for `target/`, plus any scratch scripts
or one-shot measurement drivers written during the session. Scratch
measurement code belongs in `/tmp` or gets deleted; it must never be left
in the tree where a later reader mistakes it for apparatus.

Related: [[pdfce-integration-and-the-pending-publish-decision]].
