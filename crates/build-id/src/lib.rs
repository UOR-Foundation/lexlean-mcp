//! The measure of Q7 (`QM-09`): the same `build_id` on every supported host.
//!
//! The build ID hashes only the source and semantic IDs (LexLean SPEC 21.5), so it is a
//! function of the bytes of a project and of the compiler, never of the host. `fixture/` is
//! a project whose lock was made by `lexlean` 0.4.0, and `fixture/expected` records the IDs of
//! its build. The test builds it with the `Engine` the repository pins and compares: `just vv`
//! runs that on `linux/amd64` for a pull request and on both architectures for `main`
//! (`bootstrap.yml`), so both match one recorded value, and so each other.
