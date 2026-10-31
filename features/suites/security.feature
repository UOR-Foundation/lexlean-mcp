Feature: Executed measurements of the threat model

  The threat model of src/Security.lex.tex gives each risk one measurement, which is Executed, Pending or Residual. A measurement is Executed only when registered rows whose tests pass observe the check it states, and the model cites no row that is not registered.

  @SM-01 @build
  Scenario: The measurement of R5 (path escape) is executed by NF-09, NF-12, DL-01, DL-02, DL-03 and DL-04, whose tests pass, and Security records it as Executed; the symlink clause of K5 is executed by DR-08, so Security records no clause as pending.
    Given the register, the test names of the workspace, src/Security.lex.tex and src/Admission.lex.tex
    When the rows that observe a planted path escape and the measurement of R5 are read
    Then each cited row is registered at the build level with a test, Security records the measurements of R5 and R2 as its Executed ones, the symlink clause of K5 is executed by the registered row DR-08 and no clause is recorded as unbuilt, and the same checks reject a measurement of R5 flipped back to unexecuted and a cited row that is missing

  @SM-02 @build
  Scenario: The measurement of R2 (a lexicon entry renders words that misstate its denotation) is executed by VO-06, VO-07 and VO-15, whose tests pass, and Security records it as Executed: no profile is installed, and so none served, whose path or Git package has no review record naming its ID, version and tree digest.
    Given the register, the test names of the workspace and src/Security.lex.tex
    When the rows that observe the refusal of a profile without a review record and the measurement of R2 are read
    Then each cited row is registered at the build level with a test, Security records the measurement of R2 as Executed with a status that names the review record and the refusals, and the same checks reject a measurement of R2 flipped back to unexecuted and a cited row that is missing

  @SM-03 @build
  Scenario: The measurement of R1 (a forged attestation) is executed by SG-03, SG-10 and SG-11, whose tests pass, and Security records it as Executed: an envelope of a verification, or one the lookup returns, validates only under the key history and the bytes of attestation.json.
    Given the register, the test names of the workspace and src/Security.lex.tex
    When the rows that observe the signed envelope and its validation under the key history and the measurement of R1 are read
    Then each cited row is registered at the build level with a test, Security records the measurement of R1 as Executed with a status that names its rows, and the same checks reject a measurement of R1 flipped back to unexecuted and a cited row that is missing

  @SM-04 @build
  Scenario: The measurement of R3 (a web origin or another host acting on the loopback gateway) is executed by GW-12 and GW-13, whose tests pass, and Security records it as Executed: dns-rebinding-protection passes against the served gateway and the adapter refuses a foreign Origin before the core is called.
    Given the register, the test names of the workspace and src/Security.lex.tex
    When the rows that observe the refusal of a foreign Host or Origin and the measurement of R3 are read
    Then each cited row is registered at the build level with a test, Security records the measurement of R3 as Executed with a status that names the scenario and its rows, and the same checks reject a measurement of R3 flipped back to unexecuted and a cited row that is missing

  @SM-05 @build
  Scenario: The measurement of R8 (key compromise) is executed by SG-04, SG-05 and SG-07, whose tests pass, and Security records it as Executed: an envelope under a revoked key fails with KeyRevoked, a revoked key signs nothing, and a history that cannot be written gives no envelope; the measurement of R9 stays Pending, naming #45, because no tool of the gateway issues a requestState.
    Given the register, the test names of the workspace and src/Security.lex.tex
    When the rows that observe the revocation of a key and the measurements of R8 and R9 are read
    Then each cited row is registered at the build level with a test, Security records the measurement of R8 as Executed with a status that names KeyRevoked and its rows, the measurement of R9 is still unexecuted and names #45, and the same checks reject a measurement of R8 flipped back to unexecuted and a cited row that is missing
