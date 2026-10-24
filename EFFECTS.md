# Host effects

Issue #7 closes the set of effects through which the tool layer acts on its host.
This repository declares the set in the model, `src/Effects.lex.tex`, as data with
a total codec, a negative theorem and one acceptance vector per effect. The
registered rows are `EF-01` to `EF-08` (suite `effects`); `lexlean verify` checks
the kernel side, and `cargo test -p repo-model --test effects` holds the source,
this document and issue #7 to one statement.

What this declares, and what it does not: the **declaration half** of issue #7.
The generated host adapter and the `gateway` binary are not here. They need the
host effect protocol of PrismPM#216 (afflom's open drafts #246 and #339) and the
crates.io core of UOR-Foundation/MCP#14, and nothing under `crates/` generates
either. The issue stays open.

## The effects

| Effect | Wire name | Tag | Constructor | Stands for |
| --- | --- | --- | --- | --- |
| `E1` | `store.get` | `0` | `StoreGet` | the store: read an entry |
| `E2` | `store.put` | `1` | `StorePut` | the store: write an entry |
| `E3` | `engine.check` | `2` | `EngineCheck` | `Engine` call `check` |
| `E4` | `engine.snapshot` | `3` | `EngineSnapshot` | `Engine` call `snapshot` |
| `E5` | `engine.format` | `4` | `EngineFormat` | `Engine` call `format` |
| `E6` | `engine.build` | `5` | `EngineBuild` | `Engine` call `build` |
| `E7` | `verify.start` | `6` | `VerifyStart` | the verifier: start a verification |
| `E8` | `verify.control` | `7` | `VerifyControl` | the verifier: control a running verification |
| `E9` | `lexicon.read` | `8` | `LexiconRead` | the lexicon: read an entry |
| `E10` | `attestation.lookup` | `9` | `AttestationLookup` | the store: look an attestation up |

The tags are the positions of the effects in the list, and the position is the
order of the table. Issue #7 gives the dotted names of E1, E2 and E7 to E10
and says "the `Engine` calls check, snapshot, format and build" for E3 to E6;
`engine.check`, `engine.snapshot`, `engine.format` and `engine.build` are this
model's names for them, in the same grammar.

## The wire

A wire is one of two forms, `Wire.Frame` of a list of octets or `Wire.Named` of a
text, and `decode : Wire -> Result Effect Refusal` is total over both.

- **The frame** is the tag word of the effect in four octets, big-endian: the
  frame of `E<n>` is `00 00 00 <n - 1>`. This is the `u32` tag that PrismPM#246
  puts after `perform` in a step, and the position of the effect in the list, so
  the two agree by construction. A frame whose length is not four is
  `MalformedFrame`. A frame of four values that is not the frame of an effect is
  `MalformedFrame` when a value is not an octet (above 255) and `UnknownId`
  otherwise.
- **The name** is the dotted wire name of the table above. A name that is not one
  of the ten is `UnknownName`. Names are case-sensitive and exact: no trimming,
  no folding, no prefix match.
- `encode e` is the frame of `e` and `encodeName e` its name. `decode (encode e)`
  and `decode (encodeName e)` are `ok e` for every effect (`decodeEncode`,
  `decodeEncodeName`, by cases on the effect).
- **A refusal is typed** (R5): `UnknownId`, `UnknownName`, `MalformedFrame`.
  There is no fourth outcome and no default effect.

Octets: the frame is a list of natural numbers, because the proofs split it with
`cases`. A value above 255 can never arrive from a byte reader, and the model does
not rely on it: `decode` accepts exactly the four-element list `0 0 0 tag` with
`tag` in 0 to 9, and no value above 255 can be that.

## The negative theorem

"No effect outside E1 to E10 can be encoded" is `decodeSound`, stated for every
`wire : Wire`:

    soundDecode wire (decode wire) = true

where a decode result is sound when it is a refusal, or an effect `e` such that
the wire is the frame of `e` or the name of `e` (`accepts wire e`). So a wire
that decodes is the encoding of one of E1 to E10, and every other wire, of either
form and of any length or content, is a refusal. It is proved for every input, not
decided on examples:

- `decodeWordSound a b c d`: by cases on the four octets and then on the tag, the
  frame `a b c d` decodes only to the effect whose frame it is.
- `decodeFrameSound octets`: by cases on the length of the list.
- `decodeNameInSound text effects`: by induction on the list of effects, with
  `nameStepSound` and `chooseSame`, so that the name search never returns an
  effect whose name differs from the text.
- `decodeSound`: by cases on the form of the wire.

The closed set itself is the `Effect` inductive: it has ten constructors, and a
`match` over it without an eleventh arm is refused by LexLean (the first planted
defect below). `encode` has no other target, so there is no wire to write for an
effect that is not in the type.

## The Engine calls

`EngineCall` has the four operations `Check`, `Snapshot`, `Format` and `Build`.
`engineEffect` maps them to E3 to E6 and `effectEngine` maps an effect back
(`engineCallRoundTrip`, `everyEngineEffectIsACall`, `fourEngineEffects`,
`engineEffectsAreE3ToE6`). Exactly four effects are Engine calls, and verify and
the store have effects of their own (E1, E2, E7 to E10).

## Tool and method names (Q6)

No effect name contains a tool name or an MCP method name, so the generated
adapter never has one to spell. The kernel cannot reduce the UTF-8 encoding of a
text, so the model holds the octets of each name as a byte literal
(`effectNameBytes`) and decides that they spell the name (`nameBytesSpellTheNames`,
by decoding them and comparing with `effectName`); the same holds for the
reserved words (`reservedBytesSpellTheWords`). `effectNamesCarryNoToolOrMethodName`
then decides, over the octets of the ten names, that each is lower case letters and dots with no
empty segment (a tool name has an underscore and a method name a slash, so
neither can be one) and that none contains a reserved word: `lexlean_`, the
method prefixes `tools/`, `resources/`, `prompts/`, `completion/`, `tasks/`,
`server/`, `logging/`, `sampling/`, `elicitation/`, `roots/`, `notifications/` and
the bare methods `initialize` and `ping`. `reservedWordsCoverTheTools` shows the
list catches `lexlean_check`, `lexlean_verify` and `tools/call`, and
`aToolNameIsNotAnEffectName` that the check refuses them as effect names.

## The acceptance vectors

| Label | Wire | Expected |
| --- | --- | --- |
| E1 store.get by frame | `frame 00 00 00 00` | `E1 store.get` |
| E2 store.put by frame | `frame 00 00 00 01` | `E2 store.put` |
| E3 engine.check by frame | `frame 00 00 00 02` | `E3 engine.check` |
| E4 engine.snapshot by frame | `frame 00 00 00 03` | `E4 engine.snapshot` |
| E5 engine.format by frame | `frame 00 00 00 04` | `E5 engine.format` |
| E6 engine.build by frame | `frame 00 00 00 05` | `E6 engine.build` |
| E7 verify.start by frame | `frame 00 00 00 06` | `E7 verify.start` |
| E8 verify.control by frame | `frame 00 00 00 07` | `E8 verify.control` |
| E9 lexicon.read by frame | `frame 00 00 00 08` | `E9 lexicon.read` |
| E10 attestation.lookup by frame | `frame 00 00 00 09` | `E10 attestation.lookup` |
| E1 store.get by name | `name "store.get"` | `E1 store.get` |
| E2 store.put by name | `name "store.put"` | `E2 store.put` |
| E3 engine.check by name | `name "engine.check"` | `E3 engine.check` |
| E4 engine.snapshot by name | `name "engine.snapshot"` | `E4 engine.snapshot` |
| E5 engine.format by name | `name "engine.format"` | `E5 engine.format` |
| E6 engine.build by name | `name "engine.build"` | `E6 engine.build` |
| E7 verify.start by name | `name "verify.start"` | `E7 verify.start` |
| E8 verify.control by name | `name "verify.control"` | `E8 verify.control` |
| E9 lexicon.read by name | `name "lexicon.read"` | `E9 lexicon.read` |
| E10 attestation.lookup by name | `name "attestation.lookup"` | `E10 attestation.lookup` |
| the first unassigned id | `frame 00 00 00 0a` | `refused UnknownId` |
| the largest octet id | `frame 00 00 00 ff` | `refused UnknownId` |
| a high octet set | `frame 00 00 01 00` | `refused UnknownId` |
| the highest octet set | `frame ff 00 00 00` | `refused UnknownId` |
| a name no effect has | `name "store.delete"` | `refused UnknownName` |
| the empty name | `name ""` | `refused UnknownName` |
| an MCP method name | `name "tools/call"` | `refused UnknownName` |
| a tool name | `name "lexlean_check"` | `refused UnknownName` |
| a name in capitals | `name "Store.Get"` | `refused UnknownName` |
| a name with a trailing space | `name "store.get "` | `refused UnknownName` |
| the empty frame | `frame ()` | `refused MalformedFrame` |
| three octets | `frame 00 00 00` | `refused MalformedFrame` |
| five octets | `frame 00 00 00 00 00` | `refused MalformedFrame` |
| a value that is not an octet | `frame 00 00 00 256` | `refused MalformedFrame` |
| a value that is not an octet in the first place | `frame 300 00 00 01` | `refused MalformedFrame` |

Each vector is one request to the decoder and the result it must give. The model
decides all of them (`acceptanceVectorsPass`); every effect has one by frame and
one by name (`everyEffectHasAVectorByFrameAndByName`), and every refusal has one
(`everyRefusalHasAVector`). The gateway work consumes this list unchanged. A
vector is a decode request, not yet an effect performed: exercising an effect end
to end needs the generated adapter and the host (see below).

## Mapping to PrismPM#216

The upstream form is not stable. At the time of this change PrismPM#246 (slice 1)
and #339 (slice 2) are drafts, so this model does not mirror them. It follows
what is shared, and the table says what changes when #216 lands.

| Subject | PrismPM#246 | This model | When #216 merges |
| --- | --- | --- | --- |
| Declaration | `ServerApplication.host : HostProtocol`, a closed `effects : List HostEffect` in `Foundation.Server.V1.Model` | `Effect`, an inductive of ten constructors, and `allEffects` | the ten become the `effects` of the application's `HostProtocol`, in this order; `tagsAreZeroToNine` pins the order, and `Effects` stays as the reference the generated list is compared with |
| Name | unique dotted lowercase `name` | `effectName`, the same grammar | unchanged; `namesAreDistinct` and the Q6 theorems hold the list to it |
| Tag | the position in the list, a `u32` after `perform` | `effectTag`, the position, four octets big-endian | unchanged; the frame is the generated `Request` tag |
| Fields | typed request and result fields (`U32`, `U64`, `Flag`, `Bytes`, `Text`), closed failures, byte maxima | none: the effect set is names and tags only | each effect gets its request and result fields and failures when the store, the Engine and the verifier slices define their payloads; this model gains a field list per effect and no tag changes |
| Undeclared effect | `EffectUndeclared`: no constructor, no decoder arm | `UnknownId` | `UnknownId` maps to `EffectUndeclared` and `MalformedFrame` to `EffectMalformed`; the generated decoder is checked against `decode` on the vectors |
| Names on the wire | not on the wire | `UnknownName`, `Wire.Named` | names are a model-side key (documents, diagnostics, vectors); the wire form stays the tag, and `Named` is not generated |
| Acceptance | `transcripts` of steps, per vector | `acceptanceVectors` of decode requests | the vectors become the first requests of the transcripts; `acceptanceVectorsPass` stays |
| Core | `core` is a LexLean production root exported as `run(Bytes)` | not declared here | the tool layer's root and the `bindings` crate are declared with the gateway (MCP#14) |

If the declaration form of #216 differs from this one when it merges, the remap is
the rows above; the ten names, the ten tags, the three refusals and the vectors
are what must survive it, and the theorems say so.

## Not met

- The generated host adapter and the `gateway` binary: they need PrismPM#216
  (#246, #339) and MCP#14. Nothing here generates either, and nothing under
  `crates/` changed for one.
- The vectors exercised end to end through a gateway and a host: here they are
  decode requests that the model decides.
- The remap to afflom's declaration form, if it differs (above).
- `Architecture` and `Product` are untouched. `pendingInProduct` lists the
  components with no anchor yet, which is not a statement about effects, so a tie
  would be a change of anchors and not a one-line addition.

## Planted defects

A gate is evidence only after a planted defect shows that it can fail (AGENTS.md). Each defect below was applied to a copy of `src/Effects.lex.tex` in a scratch project with the repository's `lexlean.toml` and `lexlean.lock`, `lexlean verify src/Effects.lex.tex` was run with the compiler pinned in `lexlean.lock`, and the copy was restored from the committed file. The unplanted file verifies (`verified 1 module`). Each run is refused with exit status 1.

| Defect | Command | Gate | Diagnostic | Restore |
| --- | --- | --- | --- | --- |
| an eleventh effect: a constructor `Rogue` added to `Effect` | `lexlean verify src/Effects.lex.tex` | the closed `Effect` inductive (every `match` over it must be exhaustive) | `LLT4001`: nonexhaustive match branches (the link phase refuses the eleventh constructor before Lean runs) | the committed `src/Effects.lex.tex`: `lexlean verify src/Effects.lex.tex` prints `verified 1 module` again |
| an eleventh effect in the list: `allEffects` gains a second `StoreGet` | `lexlean verify src/Effects.lex.tex` | `tenEffects`, the tag and name distinctness theorems | `LLV7002`: `decide` proved the proposition false at `tenEffects`, `tagsAreZeroToNine`, `tagsAreDistinct`, `namesAreDistinct`, `namesAreTheIssueNames` | the committed `src/Effects.lex.tex`: `lexlean verify src/Effects.lex.tex` prints `verified 1 module` again |
| a duplicated id: `effectTag` gives `EngineFormat` the tag 3 of `EngineSnapshot` | `lexlean verify src/Effects.lex.tex` | `tagsAreZeroToNine`, `tagsAreDistinct`, `decodeEncode` | `LLV7002`: `rfl` failed; `decide` proved the proposition false at `decodeEncode`, `decodeWordSound`, `tagsAreZeroToNine`, `tagsAreDistinct`, `engineEffectsAreE3ToE6` | the committed `src/Effects.lex.tex`: `lexlean verify src/Effects.lex.tex` prints `verified 1 module` again |
| a decode that accepts an unknown id: the unassigned tag of `decodeId` yields `StoreGet` | `lexlean verify src/Effects.lex.tex` | the negative theorem (`decodeWordSound`) and the vectors | `LLV7002`: `rfl` failed; `decide` proved the proposition false at `decodeWordSound`, `acceptanceVectorsPass` | the committed `src/Effects.lex.tex`: `lexlean verify src/Effects.lex.tex` prints `verified 1 module` again |
| a decode that accepts an unknown name: `decodeNameIn` yields `StoreGet` when no name matches | `lexlean verify src/Effects.lex.tex` | the negative theorem (`decodeNameInSound`) and the vectors | `LLV7002`: unsolved goals; `decide` proved the proposition false at `decodeNameInSound`, `acceptanceVectorsPass` | the committed `src/Effects.lex.tex`: `lexlean verify src/Effects.lex.tex` prints `verified 1 module` again |
| an effect name that contains a tool name: `engine.check` becomes `lexlean_check` in `effectName` and `effectNameBytes` | `lexlean verify src/Effects.lex.tex` | Q6 (`effectNamesCarryNoToolOrMethodName`) and the issue's names | `LLV7002`: `decide` proved the proposition false at `namesAreTheIssueNames`, `effectNamesCarryNoToolOrMethodName`, `acceptanceVectorsPass` | the committed `src/Effects.lex.tex`: `lexlean verify src/Effects.lex.tex` prints `verified 1 module` again |
| an Engine call mapped to the wrong effect: `Check` maps to `EngineSnapshot` | `lexlean verify src/Effects.lex.tex` | `engineCallRoundTrip`, `everyEngineEffectIsACall`, `engineEffectsAreE3ToE6` | `LLV7002`: `rfl` failed; `decide` proved the proposition false at `engineCallRoundTrip`, `everyEngineEffectIsACall`, `engineEffectsAreE3ToE6` | the committed `src/Effects.lex.tex`: `lexlean verify src/Effects.lex.tex` prints `verified 1 module` again |
| a vector that states the wrong refusal: the first unassigned id expects `UnknownName` | `lexlean verify src/Effects.lex.tex` | `acceptanceVectorsPass` | `LLV7002`: `decide` proved the proposition false at `acceptanceVectorsPass` | the committed `src/Effects.lex.tex`: `lexlean verify src/Effects.lex.tex` prints `verified 1 module` again |
