# Portable project knowledge

This directory is committed so a normal clone includes the graph, its audit trail,
and the source investigations completed in this repository.

## What to keep

* `graph.json`, `graph.html`, and `GRAPH_REPORT.md`: the generated graph snapshot,
  interactive community view, and report.
* `memory/` and `reflections/`: saved questions, source traces, verification results,
  and lessons used by later graph queries.
* `.graphify_labels.json`: curated community names.
* `manifest.json`, `cache/ast/`, and `cache/semantic/`: incremental extraction state.
  Semantic cache locations containing `$graphify-root$` are intentionally portable
  placeholders understood by graphify.
* `graph_health.json`, `graph_questions.json`, `benchmark.json`, and `cost.json`:
  diagnostic and accounting evidence. Zero token counters in this snapshot mean
  host usage was unavailable, as explained in the report.

Source paths in the exported graph are relative to the repository root. External
runtime source references identify the pinned Cargo package and source file,
not a developer's Cargo cache directory. Resolve that package's `manifest_path`
with `cargo metadata --locked --format-version 1` on the current machine.

## Use after cloning

The report, saved investigations, and HTML can be read without rebuilding.
Run CLI commands from the repository root after installing the same graphify
version used by this snapshot:

```sh
uv tool install graphifyy==0.9.48
graphify query "selection participant rendered marker"
graphify reflect --if-stale
```

`.graphify_python`, `.graphify_root`, extraction scratch files, vocabulary, stat
indexes, and query timestamps are ignored because they bind to the current
machine or can be regenerated. The graphify skill recreates the interpreter
binding when needed. Do not copy these local bindings from another checkout.

The graph is a dated snapshot, not proof of current source behavior. The initial
snapshot predates the selection virtualization regression and navigation crate;
their later source traces and verification evidence are saved under `memory/`.
See also the [navigation delivery record](../docs/agent-team/checkpoints/navigation-delivery.md).
Refresh with
`/graphify . --update` after source changes. Keep the updated knowledge files in
the same commit as the source change when practical. Native platform and
accessibility verification remain separate from headless test results.
