//! The identity of a profile: `Identity.profileId` of `src/Identity.lex.tex`, in Rust.
//!
//! The model is the definition. This is the one encoder of the host, and it is held to the model
//! three ways: the tests reproduce the model's own vectors (`profilePreimageVector`,
//! `profileIdVector`, `profileDigestVector`, `changedFileProfileId` and
//! `profileFramingReviewsVector`, read from the source of the model at test time), an independent
//! recomputation in `gateway/vectors/generate.py` agrees on a real project, and a byte of any
//! profile file changes the ID. The preimage is
//!
//! ```text
//! frame("domain", DOMAIN) ++ list("files", "file", path, bytes, ...) ++
//! list("reviews", "review", record, ...) ++ frame("toolchain", toolchain)
//! ```
//!
//! where a frame is `be32(len(label)) ++ label ++ be64(len(payload)) ++ payload` and a list is
//! its label, `be64` of the number of items and the framed items. The files are sorted by path and
//! the review records by their bytes, both bytewise, so that neither the order in which the
//! operator gives them nor the file system's order is part of the ID.

use sha2::{Digest, Sha256};

/// The domain tag of a profile: the 22 octets of `lexlean-mcp-profile-v1`.
pub const DOMAIN: &[u8] = b"lexlean-mcp-profile-v1";

/// Why no identity could be made: the three refusals of `Identity.Framing`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// Two files have one path.
    DuplicatePath,
    /// Two review records are the same bytes.
    DuplicateReview,
    /// A field is as long as the 64-bit length field cannot say.
    FieldTooLong,
}

/// A length as the model's `be64 (satF n)` says it: a refusal when it does not fit, which the
/// model calls `Fit.Over` and decides before it frames anything.
fn length(count: usize) -> Result<[u8; 8], Refused> {
    let count = u64::try_from(count).map_err(|_| Refused::FieldTooLong)?;
    if count == u64::MAX {
        return Err(Refused::FieldTooLong);
    }
    Ok(count.to_be_bytes())
}

fn label(out: &mut Vec<u8>, name: &str) {
    out.extend_from_slice(&(name.len() as u32).to_be_bytes());
    out.extend_from_slice(name.as_bytes());
}

fn frame(out: &mut Vec<u8>, name: &str, payload: &[u8]) -> Result<(), Refused> {
    label(out, name);
    out.extend_from_slice(&length(payload.len())?);
    out.extend_from_slice(payload);
    Ok(())
}

/// The preimage of the profile whose files are the (path, bytes) rows `files`, whose review
/// records are `reviews` and whose toolchain is `toolchain`.
pub fn preimage(
    files: &[(Vec<u8>, Vec<u8>)],
    reviews: &[Vec<u8>],
    toolchain: &[u8],
) -> Result<Vec<u8>, Refused> {
    let mut rows: Vec<&(Vec<u8>, Vec<u8>)> = files.iter().collect();
    rows.sort_by(|left, right| left.0.cmp(&right.0));
    if rows.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Err(Refused::DuplicatePath);
    }
    let mut records: Vec<&Vec<u8>> = reviews.iter().collect();
    records.sort();
    if records.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(Refused::DuplicateReview);
    }
    let mut out = Vec::new();
    frame(&mut out, "domain", DOMAIN)?;
    label(&mut out, "files");
    out.extend_from_slice(&length(rows.len().saturating_mul(2))?);
    for (path, bytes) in rows {
        frame(&mut out, "file", path)?;
        frame(&mut out, "file", bytes)?;
    }
    label(&mut out, "reviews");
    out.extend_from_slice(&length(records.len())?);
    for record in records {
        frame(&mut out, "review", record)?;
    }
    frame(&mut out, "toolchain", toolchain)?;
    Ok(out)
}

/// The profile ID: the SHA-256 of the preimage.
pub fn profile_id(
    files: &[(Vec<u8>, Vec<u8>)],
    reviews: &[Vec<u8>],
    toolchain: &[u8],
) -> Result<[u8; 32], Refused> {
    Ok(Sha256::digest(preimage(files, reviews, toolchain)?).into())
}
