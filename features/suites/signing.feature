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
