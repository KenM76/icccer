# icc-engineer memory index

- [Loop pacing and parallel dispatch](feedback_loop_pacing_and_parallel_dispatch.md) — Ken wants ~300s loop ticks and parallel agents; serialize only on real file conflicts
- [Request channel polling cadence](reference_request_channel_polling.md) — poll the pdfce↔icccer `open/` folder every ~15 min all session, not just at startup
- [Compatibility, not certified compliance](feedback_compatibility_not_compliance.md) — never stall on a standard because certification needs measurement hardware
- [the print-conformance suite target](project_suite_compatibility.md) — the compatibility target, where it lives, and why none of it may enter the MIT repo
- [Check _sources before accepting a corpus gap](feedback_check_sources_before_accepting_corpus_gap.md) — RAG digests report gaps the primary PDFs in `_sources/` do not have
- [Fixture separation and cancelling rivals](feedback_fixture_separation_and_cancelling_rivals.md) — a zero-separation fixture manufactures a false PASS; a rival that enters twice can cancel
- [Dispatched agents arm duplicate channel monitors](feedback_dispatched_agents_arm_duplicate_channel_monitors.md) — tell subagents not to poll the channel; the lead already does, and they cannot be stopped mid-session
- [No paid standards — use a reconstruction RAG](project_no_paid_standards_use_reconstruction.md) — Ken declined IEC 61966-2-1 (CHF 210); reconstruct from free sources, own evidence class, do not re-open
- [Source disagreements become selectable options](feedback_source_disagreements_become_selectable_options.md) — ship both readings, reasoned default, and MEASURE the ΔE between them
- [Terse release go-aheads](feedback_release_goaheads_are_terse_flag_before_irreversible.md) — execute the whole sequence, but interrupt once before the first irreversible act
- [Run the conformance demonstration, not just the diagnosis](feedback_run_the_conformance_demonstration_not_just_the_diagnosis.md) — Ken wants the measured proof icccer does or does not fix it
- [pdfce integration and the pending publish decision](project_pdfce_integration_and_the_pending_publish_decision.md) — pdfce wants a pinned git rev; a crates.io release is an OPEN operator decision, not permission
- [Always release, then clean build artifacts](feedback_always_release_and_clean_build_artifacts_when_done.md) — finish work by cutting the full release; cargo clean afterwards; flag only crates.io as irreversible
- [Rename to icccer — outside moves pending](project_rename_to_icccer_outside_moves_pending.md) — v0.4.0 renamed; repo/folders/channel still iccce until Ken OKs
