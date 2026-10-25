Feature: Profiles and vocabulary

  Issue #8 asks for profile identity, provisioning, the language rule and the Atlas profiles. Provisioning, the tools, the resource template and completion need the generated gateway of #7, so this suite holds the model-side slice: the file set that a profile ID covers, the language rule and the Atlas profile data. `lexlean verify` checks the kernel side.

  @PF-01 @build
  Scenario: The profile identity is Identity.profileId over the profile files and the domain tag is the issue's: the 22 octets of the tag lexlean-mcp-profile-v1 are the profile domain that Identity frames first, and the identity of a project is the SHA-256 of the framed domain, the sorted (path, bytes) rows of the files outside the source roots, the review records and the toolchain.
    Given the profile identity of src/Identity.lex.tex and the domain tag of issue #8
    When the domain tag and the identity of a project built from the profile files are read
    Then the tag is the issue's 22 octets and the identity is Identity.profileId over the profile file set

  @PF-02 @build
  Scenario: A path is under a source root exactly when it is the root, one separator and more: roots match whole components, so src/Main.lex.tex and src/deep/Other.lex.tex are under the root src, while srcs/Main.lex.tex, the path src itself and lexlean.toml are not.
    Given the root test underRoot of src/Profiles.lex.tex
    When five paths are tested against the root src
    Then two paths under the root are under it and a lookalike directory, the bare root and a configuration file are not

  @PF-03 @build
  Scenario: Changing the bytes of any profile file, one of lexlean.toml, lexlean.lock, a file of a local lexicon package or an artifact_source file, renaming a profile file, adding one and removing one each give another profile ID, as do a file under a directory that only looks like a source root and a file named like the root.
    Given the four profile file kinds of issue #8 and the identity function idOf of src/Profiles.lex.tex
    When each file kind is changed, a file is renamed, added and removed, and two lookalikes are added
    Then every one of the nine variants has an identity other than the base's

  @PF-04 @build
  Scenario: A file under a source root is not a profile file: the profile file set drops it, and for every other file list, review list and toolchain the profile ID with that file equals the profile ID without it.
    Given the profile file set profileFiles of src/Profiles.lex.tex
    When a source file is put in front of any file list
    Then the file set and the profile ID equal those without the file, for every list, review list and toolchain

  @PF-05 @build
  Scenario: The order of the rows does not change the profile ID: four reorderings of the four profile files, one of them with a source file added, give the ID of the original order, and a repeated path is refused with DuplicatePath.
    Given the profile identity with four profile files in five orders
    When the identities are compared with the identity of the first order
    Then all are the same and a repeated path is refused with DuplicatePath

  @PF-06 @build
  Scenario: A profile has one language: a package of the profile's language is admitted and a package of another language is refused with LLC0103, for each of the nine pairs of the languages 1.0, 1.1 and 1.2 that the owner values admit; the first refusal of a package list is the answer, and the code is the one the pinned LexLean returns for a lexicon whose language does not match the project's.
    Given the language rule checkPackage and checkPackages of src/Profiles.lex.tex
    When the nine pairs of the languages 1.0, 1.1 and 1.2 are checked
    Then equal languages are admitted and every other pair is refused with LLC0103

  @PF-07 @some-true
  Scenario: The Atlas lexicon lexlean.uor.atlas@1.0.0 of the pinned LexLean is language 1.0 and holds 293 entry files with distinct ids, each a document denotation into the module Atlas, and the project examples/uor-atlas holds 86 source modules, all core modules, 80 at the top and 6 under Prelude, that include the module Atlas; the model's package, version, language, counts, entry ids and module names equal that copy.
    Given the lexicon package and example of the pinned LexLean copied under crates/model/tests/atlas
    When the copy is recomputed against the authority and read against src/Profiles.lex.tex
    Then the package, version, language, 293 entry ids and 86 module names equal the model's

  @PF-08 @build
  Scenario: An Atlas profile serves all 293 entries: every entry denotes the module Atlas, the module Atlas is among the 86 modules of the profile, and the number of served entries is 293; without the module Atlas no entry is served, an entry into a module the profile lacks is not served, and the owner values give the uor-atlas profile the language 1.0, so a package of language 1.2 is refused with LLC0103.
    Given the Atlas data and the served count of src/Profiles.lex.tex
    When the entries are served against the modules of the profile, with and without the module Atlas
    Then all 293 are served, none without the module Atlas and 293 of 294 with an entry into a missing module

  @PF-09 @build
  Scenario: Each law of the profile model rejects its planted defect: a root that matches a prefix instead of a whole component, a profile file set that keeps source files, a serve check that counts an entry into a missing module and a language rule that admits another language each fail lexlean verify; PROFILES.md records the command, the diagnostic and the restore of each.
    Given PROFILES.md and the laws of src/Profiles.lex.tex
    When the planted-defect table is read
    Then four defects are recorded with a command, a diagnostic and a restore, each against a law that exists
