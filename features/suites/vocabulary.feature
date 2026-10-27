Feature: Profiles, vocabulary and provisioning

  The gateway side of issue #8: the operator provisions a profile with commands of the binding and no effect, the profile ID covers every profile file, and the agent reads the profiles and their vocabulary through two tools, a resource template and completion, as far as the language rule and the review records allow.

  @VO-01 @build
  Scenario: The profile ID that the host computes is Identity.profileId: the host's encoder reproduces the model's profilePreimageVector, profileIdVector and profileFramingReviewsVector byte for byte, gateway/vectors/generate.py computes the ID of the vectors' profile again in Python and the host agrees, and the toolchain of the preimage is the compiler-semantics ID and the Lean toolchain of the lock that lock --check has just accepted.
    Given the model's profile vectors and the host's provisioning module
    When the preimage and the ID of the vectors' files, and of a provisioned project, are computed
    Then the host's bytes equal the model's, the Python recomputation equals the host's, and the toolchain is the compiler's

  @VO-02 @build
  Scenario: Changing any file of a provisioned profile changes the profile ID the host records: a byte of lexlean.toml, of lexlean.lock, of a file of a local lexicon package or of an artifact_source file, a renamed profile file, an added one, a removed one and a review record each give another ID, while a file under a source root leaves it alone.
    Given a project with a local lexicon package, an artifact source and a review record
    When each profile file is changed in turn, renamed, added and removed, and a source file is added
    Then every change but the source file gives another ID, and the source file gives the same ID

  @VO-03 @build
  Scenario: The language rule of the gateway is the model's and not a second copy: gateway/src/ProfileLanguage.lex.tex holds PackageCheck, PackageDecl, languageMismatchCode, checkPackage and checkPackages byte for byte as src/Profiles.lex.tex declares them, the model carries them as executable, and a test compares the text of each declaration.
    Given src/Profiles.lex.tex and gateway/src/ProfileLanguage.lex.tex
    When the five declarations are extracted from both modules and compared
    Then each declaration is byte for byte the model's, and the model marks each as executable

  @VO-04 @build
  Scenario: A package of another language is refused with LLC0103 at every place it could enter: install refuses a project whose lexicon package has another language with the compiler's own LLC0103 and installs nothing, and a catalogue or a vocabulary that holds such a package is not served, lexlean_profiles lists the profile under refused with LLC0103 and not among profiles, lexlean_lexicon_search and the read of its resources are refused with LanguageMismatch and the code LLC0103, and none of its entries is returned.
    Given a project that locks a language 1.2 path package and a profile that declares a language 1.0 package in its vocabulary
    When the project is installed, and the tools and the resource template are asked for the profile
    Then install is refused with LLC0103 and no profile is left, and each tool and the resource read refuse the other store with LLC0103 and return no entry

  @VO-05 @build
  Scenario: Provisioning is an operator command of the binding and not an effect: gateway-provision init and lock run lexlean init and lexlean lock in process, --allow-network is accepted by lock alone, install never writes the lock (a stale lock is the compiler's LLC0102 and a missing lock is refused), no tool of the gateway locks, initialises or installs, and the set of effects is still the model's ten.
    Given an empty directory, a project whose lock is stale and the gateway-provision binary
    When the binary initialises, locks and installs, and --allow-network is given to each command
    Then init and lock produce what the lexlean commands produce, only lock takes the flag, install leaves the lock as it found it and refuses a stale one, and the gateway's effects are E1 to E10

  @VO-06 @build
  Scenario: A package that enters from a path or from Git needs a review record before a profile is installed (K2, and the measurement of risk 2 that SECURITY.md executes): a record is four lines (lexlean-mcp-review-v1, package ID@VERSION, tree_sha256 and the digest, reviewer and a name), install refuses a locked path or Git package that no record names with ReviewMissing, a record that is not four such lines with ReviewMalformed, a record that names no locked package or another tree digest than the lock's with ReviewStale and a record given twice with ReviewDuplicate, a builtin package needs none, and the bytes of every record enter the profile ID and are stored as reviews/<SHA-256 of the record>.
    Given projects with a path package, a Git package and builtin packages, and review records that are valid, missing, malformed, stale and repeated
    When each project is installed with each set of records
    Then a project is installed only when every path or Git package has a valid record, each failure is its typed refusal, and the records are in the ID and in the store

  @VO-07 @build
  Scenario: install lays out the profile as one rename into the profiles area and leaves the store as it was when it refuses: profiles/<name> holds the project as locked (not .git, .lake, or .lexlean but its Git package cache), the record profile, the derived vocabulary and the reviews, the catalogue index/profiles lists it, a name that is not a valid key segment is NameInvalid, a name already installed is ProfileExists, a project with a symbolic link is LinkInBundle, a top-level entry named profile, vocabulary or reviews is ReservedEntry, a project with source packages is SourcePackagesUnsupported, and materialization (DR-07) copies the bundle without the record, the vocabulary and the reviews.
    Given a store and projects that install, that hold a link, a reserved entry or source packages, and a name already installed
    When each project is installed, and a draft of an installed profile is materialized
    Then the layout is as stated, each refusal is its constructor and leaves the store byte for byte as it was, and the materialized project holds none of the three entries

  @VO-08 @build
  Scenario: The vocabulary is the entries of the packages that the compiler resolved for the project, as lexlean::lock::resolve_packages loads them: the document lists each locked lexicon package with its version, its language and its number of entries, and the entries in package and local ID order each with its package, version, category, frame, arity, denotation and forms, an entry that denotes a module the source roots of the profile lack is left out, and a vocabulary over 1 MiB (the bound of lexicon.read) is VocabularyTooLarge.
    Given a project with a local package of three entries, one of which denotes a module that the project lacks
    When the project is installed and its vocabulary is read from the store
    Then the vocabulary lists every package and the two served entries, the entry into the missing module is absent, and a vocabulary over the bound is refused at install

  @VO-09 @build
  Scenario: lexlean_profiles takes an optional profile and asks one store.get of the key profiles of the index area: it answers every installed profile with its name, ID, language, source roots and packages (package, version, language, entries), a named profile that is not installed is ProfileUnknown, a catalogue that is not the layout is CatalogueMalformed, a profile whose packages are not all of its language is under refused with the code LLC0103, an absent catalogue is no profile, and the input schema is closed.
    Given stores with two installed profiles, with none and with a catalogue that is not the layout
    When lexlean_profiles is called without arguments, with each name and with a bad argument
    Then the profiles are listed as installed, one store.get is asked for, and each failure is its refusal or its invalid-arguments error

  @VO-10 @build
  Scenario: lexlean_lexicon_search takes a profile, an optional query (matched against the entry ID and the spelling of every form without regard to ASCII case), an optional package, an offset and a limit of at most 100, asks one lexicon.read of the entry vocabulary and answers total, offset and the entries of the page in vocabulary order; an unknown profile is the host failure profile-unknown, a profile without a vocabulary is VocabularyMissing, a vocabulary that is not the layout is VocabularyMalformed, and the input schema is closed.
    Given a profile with a vocabulary of known entries
    When lexlean_lexicon_search is called with queries that match an ID, a spelling in another case and nothing, with a package, with pages, and with bad arguments
    Then the page and the total are those of the vocabulary, one lexicon.read is asked for, and each failure is its refusal, failure or invalid-arguments error

  @VO-11 @build
  Scenario: The resource template lexlean://profiles/{profile}/lexicon/{package}@{version} is served with cacheScope public and the owner's cache.ttl_ms: resources/templates/list answers it with no effect, resources/list answers one resource for every package of every installed profile with one store.get, and resources/read of a URI of the template asks one lexicon.read of the vocabulary and answers the entries of that package as application/json text; a URI that is not of the template, a profile that is not installed and a package or version the profile lacks are the protocol error -32602 with the URI, and a package of another language is refused as in VO-04.
    Given stores with installed profiles and the template of the gateway
    When resources/templates/list, resources/list and resources/read are called for URIs of the template and for URIs that are not
    Then the answers carry the public cache hint, the effects are those stated, and the resource of a package holds its entries

  @VO-12 @build
  Scenario: completion/complete of the reference ref/resource of the template completes the argument profile from the installed profiles and the argument package from the packages of the profile that context.arguments.profile names (from all installed profiles when it names none), by prefix and at most 100 values with total and hasMore as MCP's Resources.completionOf computes them; another reference, another argument name and a profile that is not installed in the context are the protocol error -32602 or no value as stated by the tests, and the call asks one store.get.
    Given stores with installed profiles and packages
    When completion/complete is called for each argument with prefixes and contexts
    Then the values are those of the catalogue by prefix, and every refusal is -32602

  @VO-13 @build
  Scenario: The capabilities the gateway declares are those it serves and no others: server/discover declares tools, resources and completions, a method of each is answered, prompts and the tasks methods are not declared and are answered as before, gateway/waiting.json names exactly the scenarios of MCP's server suite that still fail with a reason that is true of this gateway, and GATEWAY-CONFORMANCE.md is that table.
    Given the core and the server suite over the served gateway
    When server/discover and every method of the declared capabilities are called, and the suite runs
    Then each declared capability is served, an undeclared one is refused, and the failing scenarios are the waiting list

  @VO-14 @build
  Scenario: An Atlas profile serves all 293 entries through the tools and the resource template: the project examples/uor-atlas of the pinned LexLean installs as a profile of language 1.0 with its 86 core modules in its source root, lexlean_profiles lists lexlean.uor.atlas 1.0.0 with 293 entries, lexlean_lexicon_search pages through exactly the 293 entry IDs of the model's atlasEntries, and resources/read of the Atlas package returns the same 293 entries; a profile without the module Atlas serves none of them.
    Given the Atlas project of the pinned LexLean, installed as a profile, and the same project without the module Atlas
    When the Atlas profile is listed, searched in pages and read as a resource
    Then 293 entries are served, their IDs are the model's, and the profile without the module Atlas serves none

  @VO-15 @build
  Scenario: Every refusal of provisioning and of the vocabulary tools is typed and has a negative test: the constructors of provision::Refusal (NameInvalid, ProfileExists, ProjectUnreadable, Compiler, LinkInBundle, ReservedEntry, SourcePackagesUnsupported, ReviewMissing, ReviewMalformed, ReviewStale, ReviewDuplicate, VocabularyTooLarge, IdentityRefused, StoreUnwritable), the tool refusals ProfileUnknown, CatalogueMalformed, VocabularyMissing, VocabularyMalformed and LanguageMismatch, the host failures profile-unknown and key-refused of lexicon.read, the invalid arguments of the two tools, and the protocol error -32602 of a resource that is not found and of a completion that is not served.
    Given a negative test for each refusal
    When each refusal is provoked
    Then each is the constructor or the code stated, and nothing is served

  @VO-16 @build
  Scenario: Each gate of the profile work rejects its planted defect: a profile file excluded from the identity, a mixed-language package admitted, an Atlas entry missing from the served lexicon, a capability declared that is not served, a profile installed without a review record and a reserved entry copied into a project; PROFILES.md records the command, the diagnostic and the restore of each.
    Given the sources with one defect planted
    When the gate of the defect is run
    Then the gate fails with its diagnostic and the restored sources pass
