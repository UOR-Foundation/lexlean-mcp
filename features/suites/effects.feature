Feature: Host effects

  Issue #7 closes the set of effects through which the tool layer acts on its host. The model declares the ten effects E1 to E10 as data with a total codec, proves that no other wire decodes, and carries one acceptance vector per effect for the gateway that PrismPM#216 and MCP#14 will generate.

  @EF-01 @build
  Scenario: The effect set is the ten effects of issue #7, E1 to E10, and no other: the Effect inductive of src/Effects.lex.tex has exactly those ten constructors in that order, allEffects lists them, their wire names are the dotted names of the issue, and their wire tags are their positions 0 to 9, which EFFECTS.md lists.
    Given src/Effects.lex.tex, EFFECTS.md and the ten effects issue #7 names
    When the constructors, the list, the names, the tags and the table are read
    Then they are the ten effects of the issue in order, with names and tags equal in the model and in the table

  @EF-02 @build
  Scenario: decode is a total function from a wire to Result Effect Refusal with the typed refusals UnknownId, UnknownName and MalformedFrame, and decode inverts encode: for every effect, the frame and the name it encodes to decode back to it, proved by cases on the effect.
    Given the Wire, Refusal and Effect types and the theorems decodeEncode and decodeEncodeName
    When their statements and proofs are read
    Then each is a round trip proved by cases on the effect, decode handles both forms of a wire and EFFECTS.md states the frame

  @EF-03 @build
  Scenario: No wire outside the encodings of E1 to E10 decodes: decodeSound states over every Wire, not a list of examples, that a wire which decodes to an effect is that effect's frame or name, proved for every input by cases and induction, and the acceptance vectors refuse with each of the three refusals.
    Given the theorem decodeSound, its chain decodeFrameSound, decodeWordSound and decodeNameInSound, and the acceptance vectors
    When the statement quantifies over a wire and no proof of the chain is decide
    Then the negative theorem holds for every wire and each refusal is exercised by a vector

  @EF-04 @build
  Scenario: The wire tags of E1 to E10 are the pairwise distinct numbers 0 to 9 and their wire names are pairwise distinct, decided by the kernel.
    Given the tag and the name of each effect
    When the tags and the names are compared pairwise
    Then no two effects share a tag or a name

  @EF-05 @build
  Scenario: The Engine-call effects E3 to E6 correspond one to one to the four Engine operations check, snapshot, format and build: engineEffect and effectEngine are inverse on them, exactly four effects are Engine calls, and their numbers are 3 to 6.
    Given the EngineCall type and the maps engineEffect and effectEngine
    When the round trips, the count and the numbers are read
    Then the four calls and the four effects correspond one to one

  @EF-06 @build
  Scenario: No effect name contains a tool name or an MCP method name (Q6): every name is lower case letters and dots with no empty segment and avoids the reserved words lexlean_, tools/, resources/, prompts/ and the other method prefixes, decided by the kernel over the name list.
    Given the names of the ten effects and the reserved words of the model
    When each name is held against the alphabet and the reserved words
    Then no name contains a reserved word and every name is lower case letters and dots

  @EF-07 @build
  Scenario: Every effect is exercised by an acceptance vector by frame and by name, the vectors decode as they state, and EFFECTS.md lists the vectors of the model, in order, and no other.
    Given the acceptance vectors of the model and the table of EFFECTS.md
    When each effect is looked up among the vectors and the table is compared with the model
    Then every effect has a vector by frame and by name and the table equals the model

  @EF-08 @build
  Scenario: Each gate of the effect set rejects its planted defect: an eleventh effect, a duplicated id, a decode that accepts an unknown wire, an effect name containing a tool name and a mismapped Engine call; EFFECTS.md records the command, the diagnostic and the restore of each.
    Given EFFECTS.md and the committed source
    When the planted defects are read
    Then each has a command that runs lexlean verify, a diagnostic and a restore, and the source holds none of them
