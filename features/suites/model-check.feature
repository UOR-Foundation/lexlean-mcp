Feature: prismpm check on the root model

  The SDK of the development binding checks this language-1.2 model, and the model has the form that check requires.

  @MC-01 @build
  Scenario: The root is a PrismPM project over the root model and just vv runs prismpm check: prismpm.toml names lexlean.toml, the recipe vv depends on model-check and model-check is prismpm check, so a model the check refuses does not pass the acceptance gate.
    Given the root prismpm.toml and the Justfile
    When just vv runs
    Then prismpm check runs inside the SDK image, and a model it refuses fails the gate

  @MC-02 @build
  Scenario: The facet records have the form the check requires: for each record type of Foundation.Arch, Foundation.Sec and Foundation.Qual with an index, the index of each declaration is its position in the byte order of the IDs, and each standard reference names its entry of the standards profile with the position of that entry and edition index 0.
    Given the facet records of the modules of the model
    When each is read from its source
    Then each index is its position in the byte order of the IDs, and each standard reference has the form of the profile

  @MC-03 @build
  Scenario: Each rule of the check that the model relies on rejects its planted defect, and what it does not reject is named: an index out of order, a standard of another edition or outside the profile, a feature with no call or no acceptance row, a call that names no declaration, a journey step that is no feature, a journey actor that is no stakeholder, and an unresolved viewpoint, risk, control, requirement or measure link are each refused with their code; a concern framed by no viewpoint, a risk with no control, a requirement with no measure and a view of the wrong viewpoint pass the check, and MODEL-CHECK.md records what rejects each of them.
    Given the model with one defect planted at a time
    When prismpm check runs on each
    Then each defect its rules cover is refused with its code, and each it lets through is named in MODEL-CHECK.md
