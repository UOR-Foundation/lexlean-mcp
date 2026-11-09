#!/usr/bin/env python3
"""What users need from LexLean and Prism, ranked by demand (OB-18, OBSERVABILITY.md).

Reads the OTLP/JSON lines the gateway writes (`LEXLEAN_MCP_TELEMETRY`) or a collector's file
exporter writes, and ranks:

- the gaps prism-code reported (OB-16), one row per fingerprint (kind, construct, code), by
  distinct installs x times recorded x 2 when it blocks a build; a property Lean did not prove,
  a model the toolchain refused, and a round trip a refusal stopped each block one;
- the failed tool outcomes (S1, OB-17) by profile and diagnostic code. S1 is a cumulative counter
  since the gateway started, so each series counts its largest value; a restart between two
  files undercounts.

    python3 -I deploy/demand.py report FILE...            markdown, most demanded first
    python3 -I deploy/demand.py report --json FILE...     the same as JSON
    python3 -I deploy/demand.py issues --repo OWNER/NAME FILE...           print the plan
    python3 -I deploy/demand.py issues --repo OWNER/NAME --apply FILE...   open or update issues

`issues` keeps one GitHub issue per fingerprint, found by its title, and changes nothing unless
`--apply` is given: opening issues is outward-facing. It needs `gh`, signed in.

The input holds no user text (OB-13, OB-16), and every value read is held to the grammar the
gateway emits, so a hand-edited file cannot put text in a report or an issue either.
"""

import argparse
import json
import re
import subprocess
import sys

KINDS = ("refusal", "roundtrip", "unproved", "model-refused")
CONSTRUCT = re.compile(r"^[a-z][a-z+:.-]{0,127}$")
CODE = re.compile(r"^(?:LL[A-Z]|PP)[0-9]{4}$")
INSTALL = re.compile(r"^[0-9a-f]{32}$")
PROFILE = re.compile(r"^[a-z0-9._-]{1,64}$")
VERSION = re.compile(r"^\d{1,4}\.\d{1,4}\.\d{1,4}(?:-(?:alpha|beta|rc)\.\d{1,4})?$")
TOOL_OUTCOMES = "lexlean_mcp.tool.outcomes"
LABEL = "demand"


def attributes(record):
    """The string attributes of an OTLP record, by key."""
    found = {}
    for attribute in record.get("attributes") or []:
        value = (attribute.get("value") or {}).get("stringValue")
        if isinstance(attribute.get("key"), str) and isinstance(value, str):
            found[attribute["key"]] = value
    return found


def lines(paths):
    for path in paths:
        with open(path, encoding="utf-8") as stream:
            for line in stream:
                line = line.strip()
                if not line:
                    continue
                try:
                    yield json.loads(line)
                except ValueError:
                    continue


def blocks(kind, construct):
    """Whether a gap stopped a build: Lean or the toolchain refused, or a valid spec could not be taken."""
    return kind in ("unproved", "model-refused") or (kind == "roundtrip" and ":" in construct)


def read(paths):
    """The gap records and the largest value of each failed S1 series."""
    gaps = {}
    outcomes = {}
    for line in lines(paths):
        for resource in line.get("resourceLogs") or []:
            for scope in resource.get("scopeLogs") or []:
                for record in scope.get("logRecords") or []:
                    a = attributes(record)
                    if a.get("event") != "gap":
                        continue
                    kind, construct, code = a.get("gap.kind"), a.get("gap.construct", ""), a.get("gap.code")
                    install, version = a.get("gap.install", ""), a.get("gap.version", "")
                    count = a.get("gap.count", "")
                    if kind not in KINDS or not CONSTRUCT.match(construct) or not INSTALL.match(install):
                        continue
                    if not (code == "none" or CODE.match(code or "")) or not VERSION.match(version):
                        continue
                    if not count.isdigit() or not 1 <= int(count) <= 10000:
                        continue
                    row = gaps.setdefault((kind, construct, code), {"installs": set(), "count": 0, "versions": set()})
                    row["installs"].add(install)
                    row["count"] += int(count)
                    row["versions"].add(version)
        for resource in line.get("resourceMetrics") or []:
            for scope in resource.get("scopeMetrics") or []:
                for metric in scope.get("metrics") or []:
                    if metric.get("name") != TOOL_OUTCOMES:
                        continue
                    for point in (metric.get("sum") or {}).get("dataPoints") or []:
                        a = attributes(point)
                        klass, profile, code = a.get("class"), a.get("profile", "other"), a.get("code", "none")
                        if klass in (None, "ok") or not PROFILE.match(profile):
                            continue
                        if not (code in ("none", "other") or CODE.match(code)) or not str(klass).isdigit():
                            continue
                        key = (klass, profile, code)
                        value = int(point.get("asInt", 0))
                        outcomes[key] = max(outcomes.get(key, 0), value)
    return gaps, outcomes


def ranked(gaps):
    rows = []
    for (kind, construct, code), row in gaps.items():
        users = len(row["installs"])
        blocking = blocks(kind, construct)
        rows.append({
            "kind": kind,
            "construct": construct,
            "code": None if code == "none" else code,
            "users": users,
            "count": row["count"],
            "blocks": blocking,
            "score": users * row["count"] * (2 if blocking else 1),
            "versions": sorted(row["versions"]),
        })
    rows.sort(key=lambda r: (-r["score"], -r["users"], -r["count"], r["kind"], r["construct"], r["code"] or ""))
    return rows


def failures(outcomes):
    rows = [{"class": int(k[0]), "profile": k[1], "code": k[2], "count": n} for k, n in outcomes.items() if n > 0]
    rows.sort(key=lambda r: (-r["count"], r["profile"], r["code"], r["class"]))
    return rows


def title(row):
    code = f" {row['code']}" if row["code"] else ""
    return f"demand: {row['kind']} {row['construct']}{code}"


def body(row):
    return "\n".join([
        f"Users of prism-code asked for something Prism or LexLean could not do: `{row['kind']}` `{row['construct']}`"
        + (f" with `{row['code']}`." if row["code"] else "."),
        "",
        "| Installs | Times | Blocks a build | Score | Versions |",
        "| --- | --- | --- | --- | --- |",
        f"| {row['users']} | {row['count']} | {'yes' if row['blocks'] else 'no'} | {row['score']} | {', '.join(row['versions'])} |",
        "",
        "Counted from the gateway's gap records (OBSERVABILITY.md, OB-16) by `deploy/demand.py`, which",
        "updates this issue. The records hold fingerprints only, never a user's text.",
    ])


def markdown(gaps, outcomes):
    out = ["# Demand", "", "## Gaps prism-code reported", ""]
    out += ["| Rank | Kind | Construct | Code | Installs | Times | Blocks a build | Score |", "| --- | --- | --- | --- | --- | --- | --- | --- |"]
    for i, r in enumerate(gaps, 1):
        out.append(f"| {i} | {r['kind']} | `{r['construct']}` | {r['code'] or ''} | {r['users']} | {r['count']} | {'yes' if r['blocks'] else 'no'} | {r['score']} |")
    out += ["", "## Failed tools by profile and code (S1)", "", "| Profile | Code | Class | Count |", "| --- | --- | --- | --- |"]
    for r in outcomes:
        out.append(f"| {r['profile']} | {r['code']} | {r['class']} | {r['count']} |")
    return "\n".join(out) + "\n"


def gh(*args):
    done = subprocess.run(["gh", *args], capture_output=True, text=True, check=False)
    if done.returncode != 0:
        sys.exit(f"gh {' '.join(args[:2])}: {done.stderr.strip()}")
    return done.stdout


def issues(rows, repo, apply):
    existing = {}
    if apply:
        listed = json.loads(gh("issue", "list", "--repo", repo, "--label", LABEL, "--state", "all", "--limit", "1000", "--json", "number,title"))
        existing = {issue["title"]: issue["number"] for issue in listed}
    for row in rows:
        name = title(row)
        number = existing.get(name)
        if not apply:
            print(f"would open or update: {name} (score {row['score']})")
        elif number is None:
            print(gh("issue", "create", "--repo", repo, "--title", name, "--label", LABEL, "--body", body(row)).strip())
        else:
            gh("issue", "edit", str(number), "--repo", repo, "--body", body(row))
            print(f"updated #{number}: {name}")


def main(argv):
    parser = argparse.ArgumentParser(description="Rank what users need from LexLean and Prism.")
    sub = parser.add_subparsers(dest="command", required=True)
    report = sub.add_parser("report")
    report.add_argument("--json", action="store_true")
    report.add_argument("files", nargs="+")
    plan = sub.add_parser("issues")
    plan.add_argument("--repo", required=True)
    plan.add_argument("--apply", action="store_true", help="open and update issues on GitHub")
    plan.add_argument("files", nargs="+")
    args = parser.parse_args(argv)
    if args.command == "issues" and not re.match(r"^[A-Za-z0-9-]+/[A-Za-z0-9._-]+$", args.repo):
        parser.error("--repo must be OWNER/NAME")
    gaps, outcomes = read(args.files)
    rows = ranked(gaps)
    if args.command == "report":
        if args.json:
            print(json.dumps({"gaps": rows, "failures": failures(outcomes)}, indent=2))
        else:
            sys.stdout.write(markdown(rows, failures(outcomes)))
    else:
        issues(rows, args.repo, args.apply)


if __name__ == "__main__":
    main(sys.argv[1:])
