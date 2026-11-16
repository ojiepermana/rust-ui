#!/usr/bin/env python3
"""Validate local package boundaries and the active application runtime graph."""
import json
import hashlib
import pathlib
import subprocess

root = pathlib.Path(__file__).resolve().parent.parent
metadata = json.loads(subprocess.check_output(
    ["cargo", "metadata", "--locked", "--format-version", "1"], cwd=root))
packages = {p["id"]: p for p in metadata["packages"]}
members = {packages[i]["name"]: packages[i] for i in metadata["workspace_members"]}
required = {"ui-core", "ui-components", "ui-theme", "ui-layout", "ui-icons", "ui-kit", "showcase"}
assert required <= set(members), "required workspace members are missing"
libraries = required - {"showcase"}
allowed = {
    "ui-core": set(), "ui-components": set(), "ui-theme": {"ui-core"},
    "ui-icons": set(), "ui-layout": {"ui-components"},
    "ui-kit": libraries - {"ui-kit"},
}
graph = {name: set() for name in members}
for name, package in members.items():
    for dep in package["dependencies"]:
        if dep["name"] in members:
            target = members[dep["name"]]
            assert dep.get("path"), f"{name}: internal dependency is not a local path"
            assert dep["req"] != "*", f"{name}: internal dependency has no registry alternative"
            assert pathlib.Path(dep["path"]).resolve() == pathlib.Path(target["manifest_path"]).parent.resolve()
            graph[name].add(dep["name"])
            if name in allowed:
                assert dep["name"] in allowed[name], f"unexpected boundary: {name} -> {dep['name']}"
        assert dep["name"] != "gpui", f"{name}: depend on the kit runtime re-export"
        if dep["name"] == "gpui-kit":
            assert dep["req"] == "=0.9.1", f"{name}: unpinned GPUI Kit"
            assert dep.get("source", "").startswith("registry+"), "unexpected GPUI source"
    if name in libraries:
        assert package["license"] and package["description"] and package["readme"]

visiting, visited = set(), set()
def visit(name):
    assert name not in visiting, "cycle in workspace dependencies"
    if name in visited:
        return
    visiting.add(name)
    for child in graph[name]:
        visit(child)
    visiting.remove(name)
    visited.add(name)
for name in graph:
    visit(name)
assert graph["showcase"] == {"ui-kit"}, "Showcase must consume the local facade"
normal_tree = subprocess.check_output(
    ["cargo", "tree", "-p", "showcase", "--edges", "normal", "--prefix", "none"],
    cwd=root, text=True)
assert "gpui-pre v0.3.12" in normal_tree
assert "gpui-fast v" not in normal_tree, "alternate runtime unexpectedly active"
for line in normal_tree.splitlines():
    if line.startswith("gpui-pre v"):
        assert line.startswith("gpui-pre v0.3.12"), "mixed GPUI versions"
print(f"Validated {len(members)} workspace packages: local paths, version alternatives, boundaries, acyclic graph and one GPUI runtime.")

# Keep the temporary upstream patch reviewable and prevent accidental vendor edits.
base = [p for p in metadata["packages"] if p["name"] == "gpui-base"]
assert len(base) == 1 and base[0]["version"] == "0.9.1"
vendor = root / "vendor" / "gpui-base"
assert pathlib.Path(base[0]["manifest_path"]).resolve() == (vendor / "Cargo.toml").resolve()
integrity = json.loads((vendor / "source-integrity.json").read_text())
for path, original in integrity["original_files"].items():
    expected = integrity["patched_files"].get(path, original)
    assert hashlib.sha256((vendor / path).read_bytes()).hexdigest() == expected, f"unexpected vendor change: {path}"
assert set(integrity["patched_files"]) == {
    "src/resizable/panel.rs", "src/text_selection.rs",
}
print("Verified pinned GPUI Base compatibility patch and original source integrity.")
