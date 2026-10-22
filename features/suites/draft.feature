Feature: Draft admission as loops

  Draft admission decides, for the rows of a draft, whether every path is admitted and the draft is within its limits. Every stage that runs over the rows is a loop in the exported root, so a draft of any number of rows gets a typed verdict and never exhausts the host stack.

  @DL-01 @build
  Scenario: The exported root admitDraftRun declares no recursion effect, its package carries the recorded digests and the compiler semantics ID of lexlean.lock, and the stages of draft admission that run over the rows (rowsStage, the row count, the byte total and the duplicate scan) are executable definitions of src/Admission.lex.tex.
    Given the provenance of the exported draft package, lexlean.lock and src/Admission.lex.tex
    When the effects, the recorded digests, the compiler semantics ID and the stage definitions are read
    Then the effects are allocation and overflow only, each recorded digest is the digest of the committed file, the semantics ID is the lock's and each stage is executable

  @DL-02 @build
  Scenario: The exported root admitDraftRun runs on a two MiB thread stack with a typed result for a draft of one million rows: it refuses the draft as TooManyFiles under a small maxFiles, admits a draft whose row count equals maxFiles and whose bytes equal maxBytes, refuses a draft one byte past maxBytes as TooManyBytes, and refuses a draft with a duplicate path as DuplicatePath.
    Given the exported draft root and a thread with a two MiB stack
    When the root judges a draft of one million rows, drafts at and one past the limits, and a draft with a duplicate path
    Then each verdict is the typed one: TooManyFiles, Admitted, TooManyBytes and DuplicatePath

  @DL-03 @build
  Scenario: Draft admission through the exported root admitDraftRun on the full tables gives the verdicts of the kernel-checked instances of admitDraft, with the refusal named by the constructor order of Refusal, and decides in this order: the path refusals of the rows in row order, then TooManyFiles, then TooManyBytes, then DuplicatePath.
    Given the exported draft root, the full tables and the instances of admitDraft in src/Admission.lex.tex
    When it judges the drafts of the instances and drafts that fail several stages at once
    Then each verdict is the one the instance states, and a draft that fails several stages is refused by the earliest in the order
