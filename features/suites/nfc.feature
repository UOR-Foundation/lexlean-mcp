Feature: Unicode normalization decision

  The LexLean module decides whether a scalar sequence is in NFC as Unicode 17.0.0 defines it, and is measured against the pinned Unicode Character Database 17.0.0.

  @NF-01 @build
  Scenario: The pinned Unicode Character Database files (UnicodeData.txt, CompositionExclusions.txt, DerivedNormalizationProps.txt, NormalizationTest.txt) are the Unicode 17.0.0 files whose SHA-256 digests the authorities record, and 17.0.0 is the Unicode version of unicode-normalization 0.1.25, the crate LexLean v0.4.0 at `e0b74b8` pins.
    Given the four pinned Unicode Character Database files and the dev-dependency unicode-normalization 0.1.25
    When the SHA-256 of each file, the version named in its first line and the crate's UNICODE_VERSION are read
    Then each digest equals the recorded authority, each first line names 17.0.0 and the crate's version is (17, 0, 0)

  @NF-02 @build
  Scenario: The committed artifacts/nfc-tables-low.bin, artifacts/nfc-tables-high.bin, artifacts/nfc-sample.bin and src/NfcTables.lex.tex equal, byte for byte, a fresh generation from the pinned files, and the SHA-256 of each artifact equals the digest in its artifact declaration and in lexlean.toml.
    Given the pinned files
    When the generator runs and its outputs are compared with the committed files
    Then the two halves of the tables, the sample image and the module are equal byte for byte, and the digest of each artifact is the digest declared in the module and pinned in lexlean.toml

  @NF-03 @build
  Scenario: The LexLean decision answers every line of the pinned NormalizationTest.txt as the file's own invariants state: the c2 and c4 columns are NFC, each of the c1 and c3 columns is NFC exactly when it equals c2, and the c5 column is NFC exactly when it equals c4.
    Given every line of the pinned NormalizationTest.txt
    When the exported LexLean decision judges each of c1 to c5
    Then c2 and c4 are NFC, c1 and c3 are NFC exactly when they equal c2, and c5 is NFC exactly when it equals c4

  @NF-04 @build
  Scenario: For every line of the pinned NormalizationTest.txt, the LexLean canonical decomposition with canonical ordering maps c1, c2 and c3 to c3 and c4 and c5 to c5, and the LexLean NFC maps c1, c2 and c3 to c2 and c4 and c5 to c4.
    Given every line of the pinned NormalizationTest.txt
    When the exported LexLean normalizations are applied to c1 to c5
    Then the canonical decomposition of c1, c2 and c3 is c3 and of c4 and c5 is c5, and the NFC of c1, c2 and c3 is c2 and of c4 and c5 is c4

  @NF-05 @build
  Scenario: Every code point assigned in the pinned UnicodeData.txt that is not a c1 of Part 1 of NormalizationTest.txt is NFC and is its own NFC and its own canonical decomposition, as the file's conformance clause 2 requires.
    Given every assigned code point of the pinned UnicodeData.txt that is not a c1 of Part 1
    When the exported LexLean decision judges the code point on its own
    Then it is NFC and its canonical decomposition and its NFC are itself

  @NF-06 @build
  Scenario: The LexLean UTF-8 reader returns the scalar value of every Unicode scalar value from its standard UTF-8 encoding.
    Given every Unicode scalar value, encoded to UTF-8 by the standard library
    When the exported LexLean reader decodes the octets
    Then it returns the scalar value

  @NF-07 @build
  Scenario: The oracle executes the packages exported from the LexLean roots nfcRun and admitRun, whose recorded digests and compiler semantics ID match the committed files and lexlean.lock.
    Given the committed packages and their provenance records
    When the recorded digests, the compiler semantics ID and the lock are compared
    Then each equals the committed file and lexlean.lock, and the oracle compiled exactly the recorded source

  @NF-08 @build
  Scenario: The LexLean decision agrees with unicode-normalization 0.1.25 on every line of the pinned NormalizationTest.txt and on every scalar value; the crate is a cross-check and never the authority.
    Given every line of the pinned NormalizationTest.txt and every scalar value
    When the exported LexLean decision and unicode-normalization 0.1.25 judge each string
    Then their verdicts are equal

  @NF-09 @build
  Scenario: Path admission refuses a path that is valid UTF-8 and not NFC with the typed refusal NotNfc after UTF-8 validation and before the root and suffix checks, and admits an NFC path; the instances are kernel-checked by lexlean verify.
    Given the admission module
    When it is read for the NotNfc refusal, the order of its stages and the kernel-checked instances
    Then NotNfc is a constructor of Refusal, the NFC stage follows the UTF-8 stage and precedes the root and suffix stages, and the instances e followed by U+0301 and U+00E9 are stated

  @NF-10 @build
  Scenario: The fast bound the generator derives from the pinned files is the least code point that has a nonzero combining class, is in Full_Composition_Exclusion or is the second code point of a canonical pair; the LexLean decision judges every string of two code points below it to be NFC, as unicode-normalization 0.1.25 does, and decomposesFrom is the least code point with a canonical decomposition.
    Given the pinned files, the exported LexLean decision and unicode-normalization 0.1.25
    When the bounds are derived and every pair of code points below the fast bound is judged by both
    Then the bounds are the least such code points and each pair is NFC under both

  @NF-11 @build
  Scenario: Every kernel-checked instance stated in src/Nfc.lex.tex on the sample image holds, with the same value, when the exported LexLean decision runs it on the full tables.
    Given the instance theorems of src/Nfc.lex.tex
    When the exported LexLean decision runs each on the sample image and on the full tables
    Then each returns the value the theorem states on both

  @NF-12 @build
  Scenario: Path admission through the exported LexLean root admitRun on the full tables refuses e followed by U+0301 as NotNfc and admits U+00E9, refuses invalid UTF-8 as NotUtf8 before NFC, refuses a non-NFC path as NotNfc before the root and suffix checks, and agrees with the order of the kernel-checked instances of src/Admission.lex.tex.
    Given the exported admission root and the full tables
    When it judges the paths of the kernel-checked instances and the ordering vectors
    Then each verdict is the one the instance states, with the refusal named by the constructor order of Refusal

  @NF-13 @build
  Scenario: The exported roots nfcRun and admitRun declare no recursion effect, and run a path of one MiB of scalars on a two MiB thread stack with a typed result: no stage recurses to a depth bounded by the input.
    Given the provenance of the exported package and a thread with a two MiB stack
    When the roots judge paths of one MiB of scalars of several shapes
    Then no root declares recursion and each verdict is typed

  @NF-14 @build
  Scenario: Canonical ordering sorts each run of non-starters by combining class with a stable merge sort, so the cost of the NFC decision in a run of n combining marks is O(n log n): through the exported roots nfcRun and admitRun on a two MiB thread stack, a run of 10^6 combining marks of cyclically descending class, a run of 10^6 marks in ascending class order and a run of 10^6 marks of one class are each decided within a stated time bound, with the NFD, the NFC, the NFC verdict and the path verdict that unicode-normalization 0.1.25 gives, and the NFD of the run of one class keeps the order of its marks.
    Given the exported roots, a thread with a two MiB stack and a starter followed by 10^6 marks of each shape
    When the roots decide the NFD, the NFC, the NFC verdict and the path verdict, each against the time bound
    Then each result is the one unicode-normalization 0.1.25 gives, within the bound, and the NFD of the run of one class equals the input

  @NF-15 @build
  Scenario: Canonical ordering exchanges only marks whose combining classes are strictly out of order, so marks of equal class keep their order: the kernel-checked instances of src/Nfc.lex.tex state it, and the exported decision gives the NFD and the NFC of unicode-normalization 0.1.25 on every one of 20000 pseudo-random strings of starters and marks drawn from few classes with several marks in each.
    Given the instances of src/Nfc.lex.tex that keep marks of equal class in order and pseudo-random strings of starters and marks
    When the exported decision runs the instances and decides each string
    Then each instance holds and each NFD and NFC equals the one unicode-normalization 0.1.25 gives
