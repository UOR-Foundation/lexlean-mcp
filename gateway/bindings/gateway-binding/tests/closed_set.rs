//! The closed set of effects: what the generated decoder admits, and what the core may ask.

use gateway_host_api::protocol::{
    attestation_lookup, engine_build, engine_check, engine_format, engine_snapshot, lexicon_read,
    rate_calls, store_get, store_put, verify_control, verify_start, Request, CORE, EFFECTS,
    REQUEST_MAXIMA, RESULT_MAXIMA,
};
use gateway_host_api::wire::Refusal;

/// One request of each effect, in the order of the declaration.
fn one_of_each() -> Vec<Request> {
    let engine = |project: &str| (project.to_string(), 0u32, String::new());
    let (project, selection, files) = engine("tiny");
    vec![
        Request::StoreGet(store_get::Request {
            area: 1,
            key: "tiny/lexlean.toml".into(),
        }),
        Request::StorePut(store_put::Request {
            area: 1,
            key: "tiny/lexlean.toml".into(),
            content: b"x".to_vec(),
        }),
        Request::EngineCheck(engine_check::Request {
            project: project.clone(),
            selection,
            files: files.clone(),
        }),
        Request::EngineSnapshot(engine_snapshot::Request {
            project: project.clone(),
            selection,
            files: files.clone(),
        }),
        Request::EngineFormat(engine_format::Request {
            project: project.clone(),
            selection,
            files: files.clone(),
            check_only: true,
        }),
        Request::EngineBuild(engine_build::Request {
            project: project.clone(),
            selection,
            files: files.clone(),
        }),
        Request::VerifyStart(verify_start::Request {
            project,
            selection,
            files,
        }),
        Request::VerifyControl(verify_control::Request {
            task: "task-1".into(),
            action: 0,
        }),
        Request::LexiconRead(lexicon_read::Request {
            profile: "p".into(),
            entry: "e".into(),
        }),
        Request::AttestationLookup(attestation_lookup::Request {
            by: 0,
            value: "0".repeat(64),
        }),
        Request::RateCalls(rate_calls::Request { counted: true }),
    ]
}

/// The tag of each variant; exhaustive, so an effect added to the declaration without an
/// arm here does not compile.
fn tag_of(request: &Request) -> u32 {
    match request {
        Request::StoreGet(_) => 0,
        Request::StorePut(_) => 1,
        Request::EngineCheck(_) => 2,
        Request::EngineSnapshot(_) => 3,
        Request::EngineFormat(_) => 4,
        Request::EngineBuild(_) => 5,
        Request::VerifyStart(_) => 6,
        Request::VerifyControl(_) => 7,
        Request::LexiconRead(_) => 8,
        Request::AttestationLookup(_) => 9,
        Request::RateCalls(_) => 10,
    }
}

#[test]
fn the_first_tag_past_the_set_is_undeclared() {
    let first = u32::try_from(EFFECTS.len()).expect("a count");
    assert_eq!(
        Request::decode(first, &[]).unwrap_err(),
        Refusal::Undeclared
    );
    assert_eq!(
        first, 11,
        "the set is the ten effects of the core and the frame effect"
    );
}

#[test]
fn the_largest_tag_is_undeclared() {
    assert_eq!(
        Request::decode(u32::MAX, &[]).unwrap_err(),
        Refusal::Undeclared
    );
    assert_eq!(
        Request::decode(u32::MAX - 1, &[0; 8]).unwrap_err(),
        Refusal::Undeclared
    );
}

#[test]
fn the_decoder_has_one_variant_per_effect() {
    let requests = one_of_each();
    assert_eq!(
        requests.len(),
        EFFECTS.len(),
        "one request for each declared effect"
    );
    assert_eq!(REQUEST_MAXIMA.len(), EFFECTS.len());
    assert_eq!(RESULT_MAXIMA.len(), EFFECTS.len());
    for (position, request) in requests.iter().enumerate() {
        let tag = u32::try_from(position).expect("a tag");
        assert_eq!(
            tag_of(request),
            tag,
            "the variant at {position} has that tag"
        );
        assert_eq!(request.tag(), tag);
        let payload = request.encode().expect("encodes");
        assert_eq!(
            Request::decode(tag, &payload).as_ref(),
            Ok(request),
            "{} decodes back to itself",
            EFFECTS[position]
        );
    }
    let names: Vec<&str> = EFFECTS.to_vec();
    assert_eq!(
        names,
        [
            "store.get",
            "store.put",
            "engine.check",
            "engine.snapshot",
            "engine.format",
            "engine.build",
            "verify.start",
            "verify.control",
            "lexicon.read",
            "attestation.lookup",
            "rate.calls",
        ],
        "the tag of an effect is its position, in the order of src/Effects.lex.tex"
    );
}

#[test]
fn the_frame_effect_is_not_the_cores() {
    assert_eq!(CORE.len(), EFFECTS.len());
    for (tag, core) in CORE.iter().enumerate() {
        assert_eq!(
            *core,
            tag < 10,
            "{}: only the tags 0 to 9 are effects the core may ask for",
            EFFECTS[tag]
        );
    }
}

#[test]
fn a_payload_with_bytes_to_spare_is_not_a_request() {
    let mut payload = one_of_each()[0].encode().expect("encodes");
    payload.push(0);
    assert_eq!(Request::decode(0, &payload).unwrap_err(), Refusal::Trailing);
}
