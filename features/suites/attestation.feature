Feature: The attestation tool of the gateway

  Issue #12 (F10): lexlean_attestation resolves an attestation ID, or the SHA-256 of a source file, to the signed envelope of a verification through the store index. The core was already wired to the lookup effect; these rows test the tool against a real signed record (ATTESTATION.md).

  @AT-01 @build
  Scenario: lexlean_attestation by attestation ID returns the signed envelope: for the record of a concluded verification the tool asks attestation.lookup once and answers found true with the envelope as its text, byte for byte the envelope in the store, and those bytes validate against keys/history and the stored attestation.json.
    Given a store that holds the signed record of a concluded verification
    When lexlean_attestation is called with the attestation ID
    Then the host is asked once, found is true, the text is the envelope in the store byte for byte, and it validates against the key history and the stored attestation.json

  @AT-02 @build
  Scenario: lexlean_attestation by the SHA-256 of a source file that the verification covered returns the same envelope as by the attestation ID.
    Given the same record and the digest of the source file it covered
    When lexlean_attestation is called by that digest and by the ID
    Then both answer the one envelope

  @AT-03 @build
  Scenario: A lookup that finds nothing is not an error and a malformed value is refused: an attestation ID or a digest that names no record, and an empty store, answer found false with the text not found; a value that is not 64 lowercase hexadecimal digits is the host's refusal value-refused; an empty value is refused by the core as bad arguments and the host is not asked.
    Given a store with a record, an empty store, and values that name nothing, that are malformed, or that are empty
    When lexlean_attestation is called with each
    Then a missing record is found false with the text not found, a malformed value is value-refused, and an empty value is refused before the host is asked

  @AT-04 @build
  Scenario: A lookup writes nothing: the store holds the same files with the same bytes after lookups that find, that miss and that are refused.
    Given the files of the store and their bytes
    When lookups that find, miss and are refused are made
    Then the store holds the same files with the same bytes

  @AT-05 @build
  Scenario: Each gate of the tool rejects its planted defect: the lookup reading the file that the envelope signs instead of the envelope, the digest read as an attestation ID, the check of the value dropped, and a lookup that writes to the store; ATTESTATION.md records the command, the diagnostic and the restore of each.
    Given the lookup of the host with one defect planted at a time
    When the tests of AT-01 to AT-04 run
    Then each defect makes the test that answers it fail, with the diagnostic ATTESTATION.md records
