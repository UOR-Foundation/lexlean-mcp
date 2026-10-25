Feature: Executed measurements of the threat model

  The threat model of src/Security.lex.tex gives each risk one measurement, which is Executed, Pending or Residual. A measurement is Executed only when registered rows whose tests pass observe the check it states, and the model cites no row that is not registered.

  @SM-01 @build
  Scenario: The measurement of R5 (path escape) is executed by NF-09, NF-12, DL-01, DL-02, DL-03 and DL-04, whose tests pass, and Security records it as Executed; the symlink clause of K5 is executed by DR-08, so Security records no clause as pending.
    Given the register, the test names of the workspace, src/Security.lex.tex and src/Admission.lex.tex
    When the rows that observe a planted path escape and the measurement of R5 are read
    Then each cited row is registered at the build level with a test, Security records the measurement of R5 as its only Executed one, the symlink clause of K5 is executed by the registered row DR-08 and no clause is recorded as pending, and the same checks reject a measurement of R5 flipped back to unexecuted and a cited row that is missing
