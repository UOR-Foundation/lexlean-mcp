Feature: The draft tools

  The agent writes sources as immutable drafts: a draft is identified by its profile and its files, admitted by the exported path admission, stored as one key, read back after a digest check, and materialized by copying, never by linking, into a project the Engine runs.

  @DR-01 @build
  Scenario: The draft identity of the core is Identity.draftId: the SHA-256 of the framed preimage of the domain lexlean-mcp-draft-v1, the 32 octets of the profile ID and the (path, bytes) rows sorted by path; the core's encoder reproduces the model's draftPreimageVector and draftIdVector byte for byte, and the rows of a draft in any order give the same identity.
    Given the core exported from the gateway, the model's draftPreimageVector and draftIdVector, and a profile record
    When a draft whose files are the rows of the vector is written, in two orders
    Then the identity the tool answers is the model's, the stored preimage is the model's preimage, and both orders give the same handle

  @DR-02 @build
  Scenario: The identity of a draft depends on its profile and on every path and byte: the same files under two profile IDs, a file whose bytes differ by one and a renamed file each give another identity, and the handle of a draft is its profile name and its identity, so a draft written twice is the same key of the drafts area.
    Given the core, a store with two profiles and a draft of two files
    When the same files are written under each profile, one byte is changed, and a file is renamed
    Then each variation gives another identity, and a draft written twice is one key of the drafts area

  @DR-03 @build
  Scenario: Path admission in lexlean_draft_write is the exported admission: a path with a .. component, a path that is not .lex.tex, a path that is not NFC, an absolute, empty, dot or empty-component path and a path outside the source roots are each refused with the constructor of Refusal named for it, before store.put and before any Engine call, and the only effect the call asks for is the lexicon.read of the profile record.
    Given the core, the real host and a profile record whose source root is src
    When lexlean_draft_write is called with each path that admission refuses
    Then each call is an error result naming its Refusal, and the effects of the call are the lexicon.read of the profile alone

  @DR-04 @build
  Scenario: A draft is within draft.max_files 256 and draft.max_bytes 8388608 and has no repeated path: a draft at both limits is written, and one file more, one byte more and a repeated path are refused with TooManyFiles, TooManyBytes and DuplicatePath.
    Given the core, the real host and drafts at and one past each limit
    When lexlean_draft_write is called with each draft
    Then the drafts at the limits are written and the others are refused with their Refusal

  @DR-05 @build
  Scenario: lexlean_draft_write stores the preimage as the one key <profile>/<identity> of the drafts area, and lexlean_draft_read returns the text of a file of the draft, or the paths and sizes of its files, after checking that the SHA-256 of the stored bytes is the identity of the handle: an unknown draft or a handle that is not <profile>/<64 lower case hexadecimal digits>, a path the draft lacks and a stored draft whose bytes changed or are not a preimage are refused with DraftUnknown, PathUnknown and DraftCorrupt.
    Given a draft written by lexlean_draft_write and the same draft with one byte of its stored key changed
    When lexlean_draft_read is called for a file, for the listing, for an unknown draft and for a path the draft lacks
    Then the file is the text written, the listing is its paths and sizes, and each failure is its typed refusal

  @DR-06 @build
  Scenario: The profile record is read by the first effect of a write: it holds the 64 lower case hexadecimal digits of the profile ID and one source root to a line; an unknown profile is the host failure profile-unknown, and a profile without a record, or a record that is short, long, not lower case hexadecimal, without a root or with a root that is not a relative path of whole components, is refused as ProfileRecordMalformed, with no store.put.
    Given profile records that are valid and records that are short, not hexadecimal, without a root or with a bad root
    When lexlean_draft_write is called for each
    Then a valid record gives a write, an unknown profile is profile-unknown, each bad record is ProfileRecordMalformed, and no store.put is asked for

  @DR-07 @build
  Scenario: Materialization copies: an Engine tool whose project is the handle of a draft runs on a directory of the builds area that holds the files of the profile bundle and of the draft as regular files of their own and links nothing, the result equals the result of the same tool on a directory project holding the same files, and a handle whose draft or profile is absent is the host failure project-refused.
    Given a profile bundle, a draft written under it and the same files as a directory project
    When lexlean_check is called with the handle and with the directory
    Then the results are equal, the materialized directory holds regular files of its own, and an absent draft is project-refused

  @DR-08 @build
  Scenario: A profile bundle that contains a symbolic link is refused, wherever the link lies (a file, a directory, a dangling link, below a subdirectory) and whatever it points at, as the host failure project-refused, before the Engine is called, and no byte is read through the link.
    Given profile bundles that hold a link to a file, to a directory, a dangling link and a link below a subdirectory
    When an Engine tool is called with the handle of a draft of each profile
    Then each call is project-refused, the Engine is not called, and the target of the link is not read

  @DR-09 @build
  Scenario: The draft tools are the two the host can serve within E1 to E10: lexlean_draft_write asks lexicon.read and then store.put, lexlean_draft_read asks store.get, both have closed input schemas, the set of effects is the model's ten, and the core declares no tool for the four functions of lexlean::sdk that no effect reaches.
    Given the tool list of the gateway and the effects of each call
    When tools/list is read and each draft tool is called
    Then the draft tools are lexlean_draft_write and lexlean_draft_read with closed schemas, their effects are the ones stated, and the core has no lexlean_draft_assemble

  @DR-10 @build
  Scenario: Every refusal of the draft tools is typed and has a negative test: ten constructors of Admission.Refusal, ProfileRecordMalformed, DraftUnknown, PathUnknown and DraftCorrupt are each the refusal of one test, the eleventh constructor NotUtf8 is reached only by octets that the JSON reader refuses before any tool runs, which a test shows (the constructor itself is DL-03's), and the host failures profile-unknown, key-refused and project-refused are each the failure of one.
    Given the Refusal type of Admission and the draft tools
    When each refusal is provoked by a call
    Then each reachable constructor and each draft refusal has a negative test that provokes it and names it, and NotUtf8 is shown to be refused by the reader of JSON

  @DR-11 @build
  Scenario: Each gate of the draft tools rejects its planted defect: a .. path admitted, a symbolic link followed in a profile bundle and a draft identity that ignores the profile; DRAFTS.md records the command, the diagnostic and the restore of each.
    Given DRAFTS.md and the planted defects of the draft tools
    When the defects are read
    Then each names a command, a diagnostic and a restore, and the three kinds the statement names are among them
