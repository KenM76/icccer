---
name: rename-to-icccer
description: Project renamed iccce→icccer on 2026-10-22 (v0.4.0/v0.4.1); work from D:\Dev\icccer; old channel path is a temporary junction
metadata:
  type: project
---

Ken renamed the project to **icccer** on 2026-10-22 to match the `-cer` PDF projects. Crates, binary, GitHub repo (`KenM76/icccer`), private fixtures folder and channel folder (`icccer_FeatureRequests`) all moved (`DL-084`).

**Why:** naming consistency, done before any crates.io publish so no names get orphaned.

**How to apply:**
- Work in `D:\Dev\icccer`. Ken asked for a **copy**, not a move, and deletes `D:\Dev\iccce` himself. If the old folder still exists, it is stale.
- `D:\Dev\FeatureRequests\iccce_FeatureRequests` is a **junction** to the new channel, kept because pdfcer's `tools/check-requests-scoped.py` reads the old path. Suggest removing it once pdfcer acknowledges the channel note.
- **Never** rename the "iccce" strings inside fixture profile bytes; `gen-profiles verify` will catch it.
