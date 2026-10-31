Feature: The present and export tools of the gateway

  Issue #12 (F9, F12): lexlean_present returns the attested canonical LaTeX of a record as an embedded resource, with the attestation ID and the envelope as structured content, and lexlean_export writes the bundle a reader checks offline. Both read the record that a verification leaves (SIGNING.md, SG-14), through attestation.lookup (PRESENT.md).

  @PT-01 @build
  Scenario: lexlean_present by attestation ID returns the attested canonical LaTeX of the record as one embedded resource whose text is the rendering in the store byte for byte, at the uri lexlean://attestation/<ID>/<path of the rendering> with the type application/x-tex, and its structured content is found true, the attestation ID, the path of the rendering and the envelope as JSON, which validates against keys/history and the stored attestation.json; the tool asks attestation.lookup once.
    Given a store that holds the signed record of a concluded verification
    When lexlean_present is called with the attestation ID
    Then the host is asked once, the content is one embedded resource whose text is the rendering in the store, and the structured content names the ID, the path and the envelope, which validates

  @PT-02 @build
  Scenario: AC-06: for the record of nat-add-zero, J1 ends in an embedded resource whose bytes equal the expected canonical LaTeX of the example, and the SHA-256 of those bytes is the digest the attested build manifest names for the rendering.
    Given the record of the nat-add-zero example whose rendering is its expected canonical LaTeX
    When lexlean_present is called with the attestation ID
    Then the embedded resource equals the expected canonical LaTeX and its digest is the one the build manifest names

  @PT-03 @build
  Scenario: lexlean_present by the SHA-256 of a source file the verification covered returns the same resource as by ID; the argument module, a path of the build manifest, selects one rendering of a record that has several, an empty module is the only rendering of a record that has one, and an empty module on several or a module the manifest does not name is the host's refusal module-unknown.
    Given a record with one rendering, a record with two, and the digest of a source file
    When lexlean_present is called by digest, with a module and without
    Then the same resource comes by digest as by ID, a module selects its rendering, and an ambiguous or unnamed module is module-unknown

  @PT-04 @build
  Scenario: A record that does not exist is found false with the text not found, a value that is not 64 lowercase hexadecimal digits is value-refused, an empty value is refused by the core before the host is asked, and a present writes nothing to the store.
    Given a store with a record, an empty store, and values that name nothing, that are malformed or that are empty
    When lexlean_present is called with each
    Then a miss is found false, a malformed value is value-refused, an empty value is refused before the host is asked, and the store is unchanged

  @PT-05 @build
  Scenario: The tool presents only what is attested: a rendering of the record that differs from the digest the build manifest names, or a manifest that differs from the digest attestation.json names, is not presented; the host faults and the tool answers an error without the bytes.
    Given records whose rendering or manifest differs from the digest that names it
    When lexlean_present is called
    Then no bytes are presented and the tool answers an error

  @PT-06 @build
  Scenario: Each gate of lexlean_present rejects its planted defect: the rendering read from the wrong file, the digest of the rendering not checked, the module not selected, and a present that writes to the store; PRESENT.md records the command, the diagnostic and the restore of each.
    Given the host with one defect planted at a time
    When the tests of PT-01 to PT-05 run
    Then each defect makes the test that answers it fail, with the diagnostic PRESENT.md records

  @EX-01 @build
  Scenario: lexlean_export by attestation ID writes the bundle of the record below exports/<ID>/ and answers found true with the attestation ID and the bundle key exports/<ID>: attestation.json, envelope.json (the envelope in the store), keys.json (the key history), build-manifest.json, each rendering at its path and each source file at sources/<its path>, every file byte for byte as the record holds it; the tool asks attestation.lookup once.
    Given a store that holds the signed record of a concluded verification
    When lexlean_export is called with the attestation ID
    Then the host is asked once and exports/<ID> holds the files of the record byte for byte

  @EX-02 @build
  Scenario: J4: a reader checks the bundle offline from its files alone, with the store gone: envelope.json validates against keys.json and attestation.json, the digest of build-manifest.json is the one attestation.json names, and every rendering and source file has the digest the manifest names.
    Given an exported bundle copied away from the store
    When its files are checked with the key history of the bundle alone
    Then the envelope validates and every digest leads from attestation.json to the manifest, the renderings and the sources

  @EX-03 @build
  Scenario: The renderings digest of an exported bundle matches the attested build manifest: the SHA-256 of each rendering in the bundle equals the digest of its row of the manifest of the bundle, the bundle holds exactly the renderings and sources the manifest names, and its renderings equal what lexlean_present returns for the record.
    Given an exported bundle and the record it came from
    When the renderings of the bundle are compared with the manifest and with lexlean_present
    Then each digest equals its row, the sets of files equal the manifest, and the renderings equal the presented ones

  @EX-04 @build
  Scenario: A record that does not exist, a malformed value and an empty value are answered as for lexlean_present and write nothing; a record whose evidence differs from its digests is refused with a host fault and no file of its bundle is written.
    Given a store with a record, an empty store, malformed and empty values, and a record whose evidence was altered
    When lexlean_export is called with each
    Then misses and refusals are as for present, an altered record is a host fault, and nothing is written for any of them

  @EX-05 @build
  Scenario: An export is repeatable: exporting a record again writes the same bytes, the record itself is not changed, and the bundles of two records are in directories of their own.
    Given an exported record
    When it is exported again, and a second record is exported
    Then the bytes are the same, the record is unchanged, and the two bundles are apart

  @EX-06 @build
  Scenario: Each gate of lexlean_export rejects its planted defect: a rendering left out of the bundle, the key history left out, the envelope replaced by attestation.json, and the evidence not checked before the first write; PRESENT.md records the command, the diagnostic and the restore of each.
    Given the host with one defect planted at a time
    When the tests of EX-01 to EX-05 run
    Then each defect makes the test that answers it fail, with the diagnostic PRESENT.md records
