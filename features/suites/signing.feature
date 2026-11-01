Feature: The signer of the verification records

  The host half of issue #11 that stands without the verifier process: the DSSE envelope over an in-toto statement whose subject is attestation.json, the Ed25519 key held as a file secret, the key history with rotation and revocation, and the validator that refuses what it should. The verify task and the store record are not claimed here (SIGNING.md, "Not met").

  @SG-01 @build
  Scenario: The signer writes the DSSE envelope of a verification: the payload type is application/vnd.in-toto+json, the payload is the base64 of an in-toto Statement v1 whose one subject is attestation.json with the SHA-256 of the bytes the signer was given and whose predicate type is https://uor.foundation/lexlean-mcp/verification/v1 with the predicate the caller supplied, and there is one signature, whose keyid is the SHA-256 in hexadecimal of the public key and whose sig is the Ed25519 signature over PAE(payloadType, payload).
    Given a signer with a fixed seed, the bytes of an attestation and a predicate
    When the envelope is signed
    Then the envelope, its statement and its one signature are exactly those the statement of the row describes

  @SG-02 @build
  Scenario: The primitives are the standards': PAE reproduces the example of the DSSE protocol (DSSEv1 29 http://example.com/HelloWorld 11 hello world), the Ed25519 key that the signer derives from the seed of RFC 8032 section 7.1 test 1 has that test's public key and signs that test's empty message with that test's signature, and two envelopes made from equal inputs are equal to the byte because Ed25519 is deterministic.
    Given the DSSE example, the seed of RFC 8032 test 1 and one set of inputs
    When PAE is computed, the empty message is signed and the envelope is made twice
    Then PAE, the public key and the signature are the published bytes, and the two envelopes are equal

  @SG-03 @build
  Scenario: validate accepts the envelope the signer wrote, given a key history that holds its key and the bytes of the subject, and returns the statement: it checks the signature with the key the history names by the keyid (strict verification), the payload type, the statement type, the predicate type and that the subject digest is the SHA-256 of the bytes it was given.
    Given an envelope written by the signer and the history the signer wrote
    When the envelope is validated with the bytes of the attestation
    Then the statement is returned and equals the one that was signed

  @SG-04 @build
  Scenario: Every refusal of an envelope is typed and has a negative test: the constructors of EnvelopeError (Malformed, PayloadType, Payload, Statement, NoSignature, UnknownKey, BadSignature, KeyRevoked, SubjectMismatch) are each reached by a planted envelope --- not JSON, a payload type changed, a payload that is not base64, a statement of another type or predicate, no signatures, a keyid the history lacks, a flipped signature byte or payload byte, a revoked key and a subject of other bytes --- and none of them is accepted.
    Given an envelope the signer wrote and a planted change for each constructor
    When each planted envelope is validated
    Then each is refused with its own constructor

  @SG-05 @build
  Scenario: The key history is appended by the signer and read back: the first signature with a key adds {keyid, algorithm Ed25519, public_key, not_before}, further signatures with it add nothing, a second key adds its own entry and the envelopes of the first still validate (rotation), revoke sets revoked_at so that the envelopes of that key fail with KeyRevoked while those of the other key still validate and the revoked key itself signs nothing (SignError::KeyRevoked), and a history that is not a list of well-formed entries is refused with HistoryError::Malformed.
    Given two signers with different seeds sharing one history file
    When each signs, the first key is revoked and the history is damaged in turn
    Then the history lists each key once, rotation keeps old envelopes valid, a revoked key's envelopes are refused and a damaged history is refused

  @SG-06 @build
  Scenario: The key is a file secret: Signer::load reads 64 hexadecimal digits, with surrounding whitespace allowed, as the Ed25519 seed; a missing file is KeyError::Missing, an unreadable one KeyError::Unreadable and a file of any other content KeyError::Malformed; and neither the seed nor any part of it appears in the Debug output of the signer or of a KeyError, in an envelope or in the history.
    Given key files that are missing, unreadable, too short, not hexadecimal and valid
    When each is loaded, and a signature is made with the valid one
    Then each fault has its own error, the valid key signs, and the seed appears nowhere that is written or printed

  @SG-07 @build
  Scenario: A signature that cannot be recorded is not returned: when the history cannot be written, sign returns SignError::History and no envelope; the history is written to a temporary file and renamed over the old one, so a reader sees the old list or the new one and no partial list, and no temporary file is left after a success.
    Given a history path that cannot be written and one that can
    When a signature is made with each
    Then the unwritable path gives SignError::History and no envelope, and the writable one leaves a complete history and no temporary file

  @SG-08 @build
  Scenario: Each gate of the signer rejects its planted defect: the lengths of PAE swapped, the keyid taken from the seed instead of the public key, the digest of the subject not checked, a revoked key accepted, and the history not appended on the first use of a key; SIGNING.md records the command, the diagnostic and the restore of each.
    Given the signer with one defect planted at a time
    When the tests of SG-01 to SG-07 run
    Then each defect makes the test that answers it fail, with the diagnostic SIGNING.md records

  @SG-09 @build
  Scenario: A verification is not started without a signer whose key loads: the host reads the socket of the signer from LEXLEAN_MCP_SIGNER (an empty value is no path), reads the key history from the keys area of the store, and verify.start fails with verifier-unavailable, starts no task, runs no verification and leaves nothing in the builds area when no signer is configured, when none answers at the socket and when the key file of the signer is missing, unreadable or malformed, while a project that is not one is refused as project-refused whatever the state of the key.
    Given a host with no signer, with a socket nothing answers, and with a signer whose key is a missing path, a directory, or a file that is not a key
    When verify.start is asked for a project that exists
    Then each is verifier-unavailable, the next task is still task-1, nothing is in the builds area and an absent project is project-refused

  @SG-10 @build
  Scenario: A verification that ends verified leaves a record that is a signed envelope: verifications/<attestation ID>.json is the DSSE envelope, verifications/<attestation ID>/attestation.json is the attestation.json of the verified directory byte for byte, the envelope validates against keys/history with those bytes, its predicate carries the attestation, build, source and semantic IDs, the status verified, the modules and the SHA-256 of each source file, and the result of the task names the key.
    Given the facts of a verification that ended verified, and a host with a key
    When the verification is concluded
    Then the envelope, the subject and the history are on disk and validate, and the predicate says what the row lists

  @SG-11 @build
  Scenario: attestation.lookup returns the signed envelope: by attestation ID and by the SHA-256 of a source file the verification covered, the lookup of a concluded verification returns exactly the bytes of the envelope in the store, and those bytes validate against keys/history and the stored attestation.json.
    Given a concluded verification and a lookup by its ID and by the digest of a source file
    When attestation.lookup is asked both ways
    Then both return the envelope, which validates

  @SG-12 @build
  Scenario: A verification that cannot be signed leaves no record: when the key's history cannot be written, or the attestation.json of the verified directory cannot be read, the task ends failed with class 5, and the store holds no envelope, no attestation.json and no index entry for it; a verification that failed leaves none either.
    Given a host whose history is blocked, a verified directory without attestation.json, and a draft that does not verify
    When each is concluded or run
    Then each ends failed and the verifications and index areas are empty

  @SG-13 @build
  Scenario: Each gate of the signed record rejects its planted defect: verify.start accepting a verification without a key, the failure to sign ignored and an unsigned record written, the envelope stored without its attestation.json, the predicate without the digests of the source files, and the lookup answering the unsigned record again; SIGNING.md records the command, the diagnostic and the restore of each.
    Given the host with one defect planted at a time
    When the tests of SG-09 to SG-12 run
    Then each defect makes the test that answers it fail, with the diagnostic SIGNING.md records

  @SG-14 @build
  Scenario: A verified record keeps the evidence its attestation.json binds: below verifications/<attestation ID>/ the store holds build-manifest.json, the manifest whose SHA-256 attestation.json names as build_manifest, each output of kind tex that the manifest names (the canonical LaTeX of a module) at its path, and each input of kind source at sources/<its path>, every file byte for byte with the digest the manifest gives, written after attestation.json and before the envelope; a real verification of the tiny project leaves such a record.
    Given a verification that ended verified, whose verified directory holds the manifest that attestation.json names and the rendering it names, and whose project holds the source it names
    When the host concludes it
    Then the record holds the manifest, the rendering and the source byte for byte, and the digests lead from attestation.json to each

  @SG-15 @build
  Scenario: A verification whose evidence cannot be kept whole leaves no record: when the build manifest is missing or is not the one attestation.json names, attestation.json names no manifest, a rendering or a source file is missing or differs from the digest the manifest gives, the manifest names no rendering or no source, or a path the manifest names is not a key of the store, the task ends failed with class 5 and the store holds no envelope, no attestation.json, no evidence, no index entry and no key entry for it.
    Given verified directories and projects each spoiled in one way
    When the host concludes each
    Then each task fails with class 5 and the store holds nothing for it

  @SG-16 @build
  Scenario: The signer is a program of its own and the only holder of the key: gateway-signer reads the key file from LEXLEAN_MCP_SIGNING_KEY at each request and answers on the Unix socket LEXLEAN_MCP_SIGNER with mode 0600, it writes the key history below LEXLEAN_MCP_STORE, the crate that holds it links no compiler and no generated package, and the host has no configuration for a key, loads none and links no signature library outside its tests, so that a key named in the environment of the host signs nothing.
    Given the program gateway-signer with a key file, and a host whose environment names a key file and no signer
    When the program is asked for its key and the host is asked to start a verification
    Then the program answers on a socket of mode 0600, and the host is verifier-unavailable and names no signer

  @SG-17 @build
  Scenario: An envelope made through the socket is the envelope the signer makes: its bytes equal those of the signer with the same key, subject and predicate, it validates against the history the program wrote, and a key file put right or rotated while the program runs is used at the next request without a restart.
    Given the program gateway-signer running with a key file
    When a subject is signed, the key file is replaced, and the subject is signed again
    Then the first envelope equals that of the signer, both validate against the history, and the second names the new key

  @SG-18 @build
  Scenario: Every failure of the signer service is typed and has a negative test: each refusal travels by its name and comes back as the same SignError, a request that is not one or is over MAX_REQUEST_BYTES is RequestRefused, no answer at the socket and an answer that is not one are Unavailable, a verification whose signer is lost before it concludes leaves no record, and each constructor of StartError stops the program with exit status 2 and a message that holds nothing of the key.
    Given a service spoiled in each way, a socket that answers nonsense, a verification whose signer has gone, and the program started without each of its settings
    When each is asked to sign or to start
    Then each gives its own typed failure, the store holds no record, and the program exits with status 2

  @SG-19 @build
  Scenario: Each gate of the signer service rejects its planted defect: the host loading the key itself, the socket left open to other users, the key cached at the start of the program, a refusal name that comes back as another error, and the bound on a request removed; SIGNING.md records the command, the diagnostic and the restore of each.
    Given the signer service with one defect planted at a time
    When the tests of SG-16 to SG-18 run
    Then each defect makes the test that answers it fail, with the diagnostic SIGNING.md records
