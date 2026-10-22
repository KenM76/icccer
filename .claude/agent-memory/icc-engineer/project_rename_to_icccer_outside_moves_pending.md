---
name: rename-to-icccer-outside-moves-pending
description: Project renamed iccce→icccer on 2026-10-22 (v0.4.0); GitHub repo, checkout folder, private-fixtures folder and channel folder still named iccce pending Ken's OK
metadata:
  type: project
---

Ken renamed the project to **icccer** on 2026-10-22 to match the `-cer` PDF projects (pdfcer, OCRcer). Crates, binary, env vars and prose moved in `v0.4.0` (`DL-084`).

**Why:** naming consistency, done before any crates.io publish so no names get orphaned.

**How to apply:** four renames outside the tree were held back for his explicit OK: the GitHub repo `KenM76/icccer`, `D:\Dev\icccer`, `D:\Dev\icccer-private-fixtures` and the channel folder `icccer_FeatureRequests`. Text paths to them deliberately still say `iccce`. When one moves, fix its strings in the same commit. **Never** rename the "iccce" strings inside fixture profile bytes; `gen-profiles verify` will catch it.
