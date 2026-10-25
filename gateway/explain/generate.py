#!/usr/bin/env python3
"""Render gateway/src/Explain.lex.tex, the explanations of the diagnostic codes.

`lexlean explain <CODE>` prints the entry of one registered code and needs no project: the
text is a function of the code (SPEC 23.4), so `lexlean_explain` (F11, issue #10) asks the host
for nothing and the entries are compiled into the core. The registry is LexLean's
`model/errors.toml` at the commit of `gateway/mcp/lexlean.rev`, the one source (R1); this
script renders it and holds no text of its own. An entry is exactly what `cli::explain_text`
makes of a row:

    ## `CODE` --- title

    statement

    Class: `class`. Exit code: N.

usage:
  generate.py render [SOURCES]   write gateway/src/Explain.lex.tex from SOURCES/model/errors.toml
  generate.py check [SOURCES]    exit 1 when gateway/src/Explain.lex.tex is not that rendering
  generate.py codes [SOURCES]    print the registered codes, one per line

SOURCES is a checkout of LexLean at the pin, as gateway/lay-out-lexlean.sh lays it out; without
the argument it is the environment variable LEXLEAN_SOURCES.
"""

import os
import pathlib
import sys
import tomllib

HERE = pathlib.Path(__file__).resolve().parent
MODULE = HERE.parent / "src" / "Explain.lex.tex"

# A group holds this many entries. LexLean lowers a list literal to nested terms and Lean stops
# at its recursion depth when one is too deep, so the table is a chain of short lists.
GROUP = 8


def rows(sources: pathlib.Path) -> list:
    registry = tomllib.loads((sources / "model" / "errors.toml").read_text(encoding="utf-8"))
    errors = registry["error"]
    codes = [row["code"] for row in errors]
    assert codes == sorted(set(codes)), "the registry's rows are sorted and unique"
    return errors


def entry(row: dict) -> str:
    """What `explain_text` renders from a row."""
    return (
        f"## `{row['code']}` --- {row['title']}\n\n{row['statement']}\n\n"
        f"Class: `{row['class']}`. Exit code: {row['exit']}.\n"
    )


def text(value: str) -> str:
    """A LexLean string literal."""
    out = ['"']
    for c in value:
        out.append({'"': '\\"', "\\": "\\\\", "\n": "\\n"}.get(c, c))
    out.append('"')
    return "".join(out)


def render(errors: list) -> str:
    groups = [errors[i:i + GROUP] for i in range(0, len(errors), GROUP)]
    out = []
    out.append(
        "\\begin{lexlean}{Explain}\n"
        "\\useglossary{lexlean.std.bool@1.2.0}\n"
        "\\useglossary{lexlean.std.nat@1.2.0}\n"
        "\\useglossary{lexleanmcp.titles@1.0.0}\n"
        "\\title{LexLean MCP diagnostic explanations}\n"
        "\\begin{semanticmodule}\n"
    )
    for index, group in enumerate(groups):
        pairs = "".join(f"(cons ({text(r['code'])}, {text(entry(r))}) " for r in group)
        body = f"{pairs}(nil (String * String))" + ")" * len(group)
        out.append(
            "definition{\n"
            f"  name: group{index:02d},\n"
            "  parameters: [],\n"
            "  result: (List (String * String)),\n"
            f"  body: {body},\n"
            "  executable: true\n"
            "}\n\n"
        )
    out.append(
        "definition{\n"
        "  name: find,\n"
        "  parameters: [code : String, rows : (List (String * String))],\n"
        "  result: (Option String),\n"
        "  body: (match rows\n"
        "    | List.nil => (ctor Option.none [String])\n"
        "    | List.cons head rest => (if (prim equal code (first head) : Bool) then (ctor Option.some [String] (second head)) else (find code rest))),\n"
        "  executable: true,\n"
        "  recursive_argument: rows\n"
        "}\n\n"
    )
    # `lexlean fmt` indents the arms of a nested match two columns further for each level.
    chain = "(ctor Option.none [String])"
    for index in reversed(range(len(groups))):
        indent = " " * (4 + 2 * index)
        chain = (
            f"(match (find code (group{index:02d}))\n"
            f"{indent}| Option.some found => (ctor Option.some [String] found)\n"
            f"{indent}| Option.none => {chain})"
        )
    out.append(
        "definition{\n"
        "  name: entry,\n"
        "  parameters: [code : String],\n"
        "  result: (Option String),\n"
        f"  body: {chain},\n"
        "  executable: true\n"
        "}\n"
    )
    out.append("\\end{semanticmodule}\n\\end{lexlean}\n")
    return "".join(out)


def sources_from(argv: list) -> pathlib.Path:
    given = argv[2] if len(argv) > 2 else os.environ.get("LEXLEAN_SOURCES")
    if not given:
        sys.exit("generate: name the checkout of LexLean at the pin (gateway/lay-out-lexlean.sh), as an argument or LEXLEAN_SOURCES")
    return pathlib.Path(given)


def main(argv: list) -> int:
    command = argv[1] if len(argv) > 1 else ""
    if command == "render":
        MODULE.write_text(render(rows(sources_from(argv))), encoding="utf-8")
        print(f"generate: wrote {MODULE}")
        return 0
    if command == "check":
        if MODULE.read_text(encoding="utf-8") != render(rows(sources_from(argv))):
            print(f"generate: {MODULE} is not the rendering of the registry", file=sys.stderr)
            return 1
        print("generate: the module is the rendering of the registry")
        return 0
    if command == "codes":
        for row in rows(sources_from(argv)):
            print(row["code"])
        return 0
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
