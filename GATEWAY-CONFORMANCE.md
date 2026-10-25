# The conformance run of the gateway

AC-03, part b (issue #20, decided in its comment on AC-03): the gateway passes the server
conformance suite that UOR-Foundation/MCP pins, with the gateway's own exact waiting list.
This file says what runs, what the run says today, why each scenario that does not pass is
listed, and what a planted defect looks like. The row is `GW-12` (suite `gateway`) in
`CONFORMANCE.md`; a passing scenario is evidence for what it checks, not a proof about the
gateway.

## What runs

The `gateway` job of `.github/workflows/lexlean.yml` (steps 7k to 7p), after `prismpm build`
has generated the host packages and the core is shown to be the committed export:

| Step | What it does |
| --- | --- |
| 7k | builds `gateway-host` (`cargo build --release -p gateway-host` in the gateway workspace, against the registry of the pins) |
| 7l | node (`^20.19.0 || >=22.12.0`), then the MCP checkout at `.github/mcp.rev` and the suite, installed by MCP's own `conformance/oracle/fetch-suite.sh` (commit `232a9014457eaf4c59559916e3d616d2f7f28f05`, vendored files compared, `npm ci`) |
| 7m | `cargo test -p repo-model --test gateway_conformance -- --ignored`: the tests that need node and the installed suite; the job passes only when as many passed as the source ignores (the pattern of MCP's CF-08) |
| 7n | MCP's `server-suite.mjs` over the served `gateway-host`, with `--waiting gateway/waiting.json` |
| 7o | MCP's `schema-oracle.mjs` over the transcript of 7n, against the `2026-07-28` `schema.json` MCP vendors at the pin |
| 7p | the per-scenario table, to the step summary |

MCP's oracle is not copied: `gateway_conformance_checkout` (`.github/lexlean-ci/lib.sh`) fetches
UOR-Foundation/MCP at the commit `.github/mcp.rev` names, refuses any other HEAD, and the
scripts run from that checkout (`conformance/oracle/server-suite.mjs`, `schema-oracle.mjs`,
`verdict.mjs`, `recorder.mjs`, `fetch-suite.sh`). A test (`GW-12`) refuses a copy of any of them
in this repository. The gateway's sources of the core are the same MCP commit
(`gateway/lay-out-mcp.sh`, `gateway/mcp-identities.sh`, `GW-11`).

The workspace gate (`cargo test --workspace`, and `just vv` in the locked SDK image) has neither
node nor the suite. The two tests that need them are `#[ignore]`d with a reason that names the
job; the pure checks of the same claims (the list is well formed, each entry names an issue or a
pull request, this table agrees with the list, the job and the library are wired, no copy of the
oracle) stay in the gate.

## The verdict

MCP's runner exits 1 for a failing scenario and for a harness error alike, so the verdict is the
oracle's, decided from the per-scenario checks of the frozen requirements
(`requirements/2026-07-28.yaml`, SHA-256 `ae2f4f6210fd729e2e318edd5bbfa31a43cee0bc608e48052fa26dbf1d939b57`):

- a scenario passes when it produced a check, no check failed and at least one succeeded;
- the job passes (exit 0) only when the set of failing scenarios equals the set
  `gateway/waiting.json` names, each entry naming its `issue` or `pr`;
- a failing scenario that is not listed (`FAILING and not named in waiting.json`), a listed
  scenario that does not fail (`named as waiting but not failing`) and an entry that names
  neither an issue nor a pull request each fail the job;
- a waiting scenario is still a failure: it is `fail` in the report, which says `valid: false`,
  and the scenarios passing are counted without it. The list classifies; it never turns a failure
  into a pass;
- `tasks-status-notifications` sends no request at this pin and reports one `SKIPPED` check on any
  server; it is named under `suite_skipped`, reported as `skipped by the suite`, not counted as
  passing, and held to the rule above as soon as the suite stops skipping it.

## The run, per scenario

Run of the gateway of this commit (`gateway-host serve`, the core of `gateway/generated`) through MCP's oracle at the pin 39a7d8e (the squash merge of MCP#40; its oracle and the run before it, at 59bfc29, gave the same table), spec
2026-07-28, 106 HTTP exchanges. Required: 8 of 37 pass. Not scored: 1 of 13 passes, 1 is skipped by
the suite. 40 scenarios fail and are exactly the list of `gateway/waiting.json`. The schema oracle
validated every JSON-RPC message of the transcript: 211 messages, 88 refused probes, 1 transport
refusal, all valid.

The gateway's catalogue is lexlean-mcp's ten tools, one per host effect. The suite's scenarios
were written against a fixture server, and most of them call the fixture's own tools, prompts and
resources. Those scenarios cannot pass here, and the product does not add fixture tools, prompts
or resources to make them pass (R4, and the decision recorded in #45). Each is an entry of the
list, with its reason and the issue that records it or delivers the capability.

| # | scenario | kind | result | waits on | checks | notes |
|---|---|---|---|---|---|---|
| 1 | `server-stateless` | required | waiting | #45 | 21 of 30 (5 skipped) | 4 checks call fixture diagnostic tools; 5 skipped by the suite |
| 2 | `completion-complete` | required | waiting | #45 | 1 of 2 | completes an argument of a fixture prompt |
| 3 | `tools-list` | required | pass |  | 3 of 3 | the catalogue is lexlean-mcp's ten tools |
| 4 | `tools-call-simple-text` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_simple_text` |
| 5 | `tools-call-image` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_image_content` |
| 6 | `tools-call-audio` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_audio_content` |
| 7 | `tools-call-embedded-resource` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_embedded_resource` |
| 8 | `tools-call-mixed-content` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_multiple_content_types` |
| 9 | `tools-call-error` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_error_handling` |
| 10 | `tools-call-with-progress` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_tool_with_progress` |
| 11 | `server-sse-multiple-streams` | required | pass |  | 1 of 2 (1 info) | 1 info: the gateway answers JSON, event streams are optional there |
| 12 | `resources-list` | required | waiting | #45 | 1 of 2 | fixture resource; none offered (#8 adds another template) |
| 13 | `resources-read-text` | required | waiting | #45 | 1 of 2 | fixture resource; none offered (#8 adds another template) |
| 14 | `resources-read-binary` | required | waiting | #45 | 1 of 2 | fixture resource; none offered (#8 adds another template) |
| 15 | `resources-templates-read` | required | waiting | #45 | 1 of 2 | fixture resource; none offered (#8 adds another template) |
| 16 | `sep-2164-resource-not-found` | required | pass |  | 2 of 4 (2 warning) | 2 SHOULD warnings: the gateway offers no resources, so it answers -32601 where -32602 with data.uri is recommended |
| 17 | `prompts-list` | required | waiting | #45 | 1 of 2 | fixture prompts; no prompts capability |
| 18 | `prompts-get-simple` | required | waiting | #45 | 1 of 2 | fixture prompts; no prompts capability |
| 19 | `prompts-get-with-args` | required | waiting | #45 | 1 of 2 | fixture prompts; no prompts capability |
| 20 | `prompts-get-embedded-resource` | required | waiting | #45 | 1 of 2 | fixture prompts; no prompts capability |
| 21 | `prompts-get-with-image` | required | waiting | #45 | 1 of 2 | fixture prompts; no prompts capability |
| 22 | `dns-rebinding-protection` | required | pass |  | 2 of 2 | a foreign Host or Origin is refused with 403 |
| 23 | `caching` | required | waiting | #45 | 4 of 8 (1 skipped) | tools/list hints pass; prompts/list, resources/list, resources/templates/list answer Method not found |
| 24 | `input-required-result-basic-elicitation` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_input_required_result_elicitation`; no tool of this product asks the client for input |
| 25 | `input-required-result-basic-sampling` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_input_required_result_sampling`; no tool of this product asks the client for input |
| 26 | `input-required-result-basic-list-roots` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_input_required_result_list_roots`; no tool of this product asks the client for input |
| 27 | `input-required-result-request-state` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_input_required_result_request_state`; no tool of this product asks the client for input |
| 28 | `input-required-result-multiple-input-requests` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_input_required_result_multiple_inputs`; no tool of this product asks the client for input |
| 29 | `input-required-result-multi-round` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_input_required_result_multi_round`; no tool of this product asks the client for input |
| 30 | `input-required-result-missing-input-response` | required | pass |  | 1 of 2 (1 warning) | 1 warning: the fixture tool is unknown, so the gateway answers a JSON-RPC error; a weak pass |
| 31 | `input-required-result-non-tool-request` | required | waiting | #45 | 1 of 2 | gets a fixture prompt; no prompts capability |
| 32 | `input-required-result-result-type` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_input_required_result_elicitation`; no tool of this product asks the client for input |
| 33 | `input-required-result-unsupported-methods` | required | pass |  | 2 of 2 |  |
| 34 | `input-required-result-tampered-state` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_input_required_result_tampered_state`; no tool of this product asks the client for input |
| 35 | `input-required-result-capability-check` | required | waiting | #45 | 1 of 2 | calls the fixture tool `test_input_required_result_capabilities`; no tool of this product asks the client for input |
| 36 | `input-required-result-ignore-extra-params` | required | pass |  | 1 of 2 (1 warning) | 1 warning: as above, the fixture tool is unknown; a weak pass |
| 37 | `input-required-result-validate-input` | required | pass |  | 3 of 3 |  |
| 38 | `tasks-lifecycle` | not scored | waiting | #11 | 1 of 9 | tasks extension arrives with #11 and MCP#12; fixture task tools |
| 39 | `tasks-capability-negotiation` | not scored | waiting | #11 | 1 of 5 | tasks extension arrives with #11 and MCP#12; fixture task tools |
| 40 | `tasks-wire-fields` | not scored | waiting | #11 | 1 of 4 | tasks extension arrives with #11 and MCP#12; fixture task tools |
| 41 | `tasks-request-state-removal` | not scored | waiting | #11 | 1 of 2 | tasks extension arrives with #11 and MCP#12; fixture task tools |
| 42 | `tasks-mrtr-input` | not scored | waiting | #11 | 1 of 4 | tasks extension arrives with #11 and MCP#12; fixture task tools |
| 43 | `tasks-request-headers` | not scored | waiting | #11 | 2 of 5 | tasks extension arrives with #11 and MCP#12; fixture task tools |
| 44 | `tasks-dispatch-and-envelope` | not scored | waiting | #11 | 3 of 9 | tasks extension arrives with #11 and MCP#12; fixture task tools |
| 45 | `tasks-status-notifications` | not scored | skipped by the suite |  | 0 of 1 (1 skipped) | sends no request, one SKIPPED check, whatever the server does |
| 46 | `tasks-required-task-error` | not scored | waiting | #11 | 1 of 2 | tasks extension arrives with #11 and MCP#12; fixture task tools |
| 47 | `tasks-mrtr-composition` | not scored | waiting | #11 | 1 of 2 | tasks extension arrives with #11 and MCP#12; fixture task tools |
| 48 | `json-schema-2020-12` | not scored | waiting | #45 | 1 of 2 | needs the fixture tool `json_schema_2020_12_tool` |
| 49 | `http-header-validation` | not scored | pass |  | 14 of 14 | standard routing headers (SEP-2243) |
| 50 | `http-custom-header-server-validation` | not scored | waiting | #45 | 1 of 6 | needs a tool with an x-mcp-header annotation; none has one |

### What the passes mean

- `tools-list` (3 of 3) lists the gateway's own catalogue and validates it against the suite's
  rules for tool names and schemas.
- `dns-rebinding-protection` (2 of 2) sends a foreign `Host` and `Origin` (`evil.example.com`)
  and requires a refusal; the allow-list is the declaration's `origins` (`localhost`,
  `127.0.0.1` and `[::1]`, each over `http` and `https`).
- `http-header-validation` (14 of 14), `input-required-result-unsupported-methods` and
  `input-required-result-validate-input` exercise the served headers, the typed refusal of a
  method the core does not register and of an invalid argument.
- `server-sse-multiple-streams` passes with an info: the gateway answers JSON, and event
  streams are optional there. `sep-2164-resource-not-found` passes with two SHOULD warnings: the
  gateway offers no resources, so it answers `-32601` where `-32602` with `data.uri` is recommended.
- `input-required-result-missing-input-response` and `input-required-result-ignore-extra-params`
  pass with a warning each: the fixture tool they call is not in the catalogue, so the gateway
  answers a JSON-RPC error and the SHOULD checks about re-requesting input are warnings. These are
  weak passes; they say the gateway does not crash on the call, nothing about input requests.

### Why the scenarios wait

| Group | Scenarios | Reason | Delivered by |
| --- | --- | --- | --- |
| fixture tools | `tools-call-*` (7), `input-required-result-*` that call a `test_input_required_result_*` tool (9), `server-stateless` (4 checks), `json-schema-2020-12` | the scenario calls a tool of the suite's fixture, which the catalogue does not have | no one: #45 |
| prompts | `prompts-*` (5), `input-required-result-non-tool-request`, `completion-complete` | no prompts: `ServerCapabilities.prompts` is "Present if the server offers any prompt templates." | no one: #45 (#8 adds completion of its own template's arguments, a different `ref`) |
| resources | `resources-list`, `resources-read-text`, `resources-read-binary`, `resources-templates-read` | the fixture's `test://` resources; `ServerCapabilities.resources` is "Present if the server offers any resources to read." | no one: #45 (#8 adds the template `lexlean://profiles/{profile}/lexicon/{package}@{version}`, not the fixture's) |
| caching | `caching` | 4 of 8 checks pass (`tools/list`); the others need `prompts/list`, `resources/list` and `resources/templates/list` | #45; `resources/templates/list` arrives with #8 |
| custom headers | `http-custom-header-server-validation` | needs a tool whose schema carries `x-mcp-header`, which is optional ("Property schemas may carry an `x-mcp-header` annotation") | #45: a product decision for #10 |
| tasks | `tasks-*` (9) | the tasks extension (`tasks/get`, `tasks/update`, `tasks/cancel`, `io.modelcontextprotocol/tasks` in `ServerCapabilities.extensions`, "Optional MCP extensions that the server supports") arrives with #11 and MCP#12; the scenarios also call fixture tools (`slow_compute`, `greet`, `confirm_delete`, `failing_job`) | #11 for the methods; the fixture tools: #45 |

## Reproduce

```sh
gateway/lay-out-mcp.sh && gateway/lay-out-sdk.sh && gateway/registry.sh   # as the gateway job
(cd gateway && prismpm build) && gateway/lay-out.sh gateway/.prism/build/<id>
(. .github/lexlean-ci/lib.sh && gateway_conformance_build && gateway_conformance_install)
(. .github/lexlean-ci/lib.sh && gateway_conformance_suite)        # exit 0 only if failing = gateway/waiting.json
(. .github/lexlean-ci/lib.sh && gateway_conformance_schema)
```

`GATEWAY_CONFORMANCE_DIR` names where the checkout, the suite and the reports are kept
(default `$RUNNER_TEMP/gateway-conformance`).

## Planted defects

A gate is evidence only after a planted defect shows it can fail (AGENTS.md). Each was planted
alone, against the green state above (exit 0), on the development machine; the command ran and the
change was reverted. "Green" is the exit status of `server-suite.mjs`, which is the status of step 7n.

| Defect | Planted in | Run | Diagnostic |
| --- | --- | --- | --- |
| planted: stale. A scenario that passes is listed as waiting | `gateway/waiting.json` with `tools-list` added | `server-suite.mjs --waiting <it>`: exit 1 | `named as waiting but not failing (1): tools-list`, then `the failing scenarios are not the ones .../waiting.json names` |
| planted: unlisted. A failing scenario is not listed | `gateway/waiting.json` without `caching` | exit 1 | `FAILING and not named in waiting.json (1): caching`, then the suite's check messages (`caching: sep-2549-prompts-list-caching-hints: prompts/list returned JSON-RPC error -32601: Method not found`, ...) |
| planted: unnamed. An entry names neither an issue nor a pull request | `caching` with only a reason | exit 1 | `named as waiting without an issue or a pull request (1): caching` |
| planted: served (not caught). The adapter's origin allow-list admits a foreign host | the `origins` of the declaration (`gateway/vectors/generate.py`, rendered into `gateway/src/Application.lex.tex`) with `evil.example`, then with `evil.example.com` (the host the suite probes) added; `prismpm build`, `gateway-host` rebuilt | exit 0, the scenarios unchanged | not caught, and by design of the core: the core refuses the foreign `Origin` itself (`curl` with `Host` and `Origin` of `evil.example.com` against the rebuilt host: `403`, `{"jsonrpc":"2.0","error":{"code":-32000,"message":"Forbidden: invalid Origin"}}`), so the adapter's list is a second check that this suite cannot see. The declaration's `origins` are held by the declaration and by `GW-13`, which observes the adapter's own refusal (GATEWAY.md, "The adapter's refusal of a foreign `Origin`"), not by this suite |
| planted: served. The `tools/list` answer carries a tool whose name is not a valid tool name | `gateway/src/GatewayTools.lex.tex`: the catalogue entry `lexlean_check` renamed `lexlean check`; `prismpm build`, `gateway-host` rebuilt | `server-suite.mjs --waiting gateway/waiting.json`: exit 1 | `FAILING and not named in waiting.json (3): tools-list, server-sse-multiple-streams, http-header-validation`, then `tools-list: tools-list: Failed: Internal error` |
| the same tests as automation | `cargo test -p repo-model --test gateway_conformance -- --ignored` (step 7m) | the verdict of MCP's `verdict.mjs`, run over synthetic results derived from the real list | the stale, delivered, unlisted and unnamed cases are each refused, and the exact case is accepted |

A change to the core or to the declaration is rejected earlier in the job, by step 7f: the core
that `prismpm build` exports no longer equals `gateway/generated/lexlean_mcp_gateway_core`
(`diff -r`, observed for each of the three plants above). The suite is the gate for what the job
cannot read from the sources: the bytes the running gateway serves.

## Not claimed

- The tools do what their names say only as far as the vectors of `GATEWAY.md` show; the
  semantics of each tool belong to #8 to #12. The suite does not exercise them.
- That the gateway passes the scenarios that wait is not claimed: they are failures and are
  counted as such. Eight of 37 required scenarios pass today.
- The suite is pinned at `232a9014`; a later suite, or a later MCP pin, can change this table.
  `.github/mcp.rev` is a development pin (MCP#40) and moves to a release before the release of
  this repository, together with the waiting list.
