//! Slice-1 conformance suite — First Runnable Slice Contract §8, obligation 8.
//!
//! The acceptance test for the slice is that two independently written
//! conforming implementations produce the same verdict, defect set, defect
//! identities, defect ordering and not-evaluated metadata for the same input.
//!
//! **Phase 0 asserts none of that yet.** No contract behaviour exists to
//! exercise. This target proves only that the library builds and links into an
//! integration test binary, which is what bootstrap verification is for. The
//! real suite arrives with the checks it covers, in Phases 4 through 7.

use ordina as _;

#[test]
fn library_links_into_integration_test_target() {
    // Deliberately empty: linking is the assertion at Phase 0.
}
