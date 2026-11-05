//! DP-01 to DP-04: the loopback deployment of issue #17 as a PrismPM host-effect system model.
//!
//! `lexlean verify` checks the kernel side of `src/Deployment.lex.tex`. These tests read the
//! sources as text and check the same facts a second way, so that a theorem deleted or weakened
//! in source fails here too, and they hold the model against what it names outside itself: the
//! PrismPM commit its profile is copied from, the gateway's application declaration and the
//! gateway's host protocol.

use std::path::PathBuf;

use sha2::{Digest, Sha256};

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// The source with runs of white space collapsed, so the formatter's one-line and multi-line
/// spellings of a declaration read the same.
fn tight(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace("{ ", "{")
        .replace(" }", "}")
        .replace("[ ", "[")
        .replace(" ]", "]")
}

/// The tight text of the declaration `kind{name: NAME, ...}`, up to the brace that closes it.
fn declaration(source: &str, kind: &str, name: &str) -> String {
    let head = format!("{kind}{{name: {name},");
    let start = source
        .find(&head)
        .unwrap_or_else(|| panic!("no {kind} named {name}"));
    let mut depth = 0usize;
    let mut in_string = false;
    for (offset, c) in source[start..].char_indices() {
        match c {
            '"' => in_string = !in_string,
            '{' if !in_string => depth += 1,
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return source[start..=start + offset].to_string();
                }
            }
            _ => {}
        }
    }
    panic!("the {kind} {name} is not closed");
}

fn deployment() -> String {
    tight(&read("src/Deployment.lex.tex"))
}

/// The quoted strings of `text`, in order.
fn quoted(text: &str) -> Vec<String> {
    text.split('"')
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, part)| part.to_string())
        .collect()
}

/// The value of the field `field := VALUE` of a record literal, up to the comma or brace that
/// ends it at the same depth.
fn field(record: &str, field: &str) -> String {
    let head = format!("{field} := ");
    let start = record
        .find(&head)
        .unwrap_or_else(|| panic!("no field {field} in {record}"))
        + head.len();
    let mut depth = 0i32;
    let mut in_string = false;
    for (offset, c) in record[start..].char_indices() {
        match c {
            '"' => in_string = !in_string,
            '(' | '{' if !in_string => depth += 1,
            ')' | '}' if !in_string && depth == 0 => {
                return record[start..start + offset].to_string()
            }
            ')' | '}' if !in_string => depth -= 1,
            ',' if !in_string && depth == 0 => return record[start..start + offset].to_string(),
            _ => {}
        }
    }
    panic!("the field {field} is not closed");
}

/// DP-01: the profile is PrismPM's, byte for byte.
#[test]
fn the_profile_is_the_prismpm_copy_dp_01() {
    let model = repo_model::Model::load_from_repo_root().expect("the model loads");
    let authority = model
        .authorities
        .authority
        .iter()
        .find(|a| a.id == "A-PRISMPM-SYSTEM")
        .expect("model/authorities.toml names A-PRISMPM-SYSTEM");
    let mut files = vec![
        "src/Production/Core.lex.tex",
        "src/Production/HostEffectSystem.lex.tex",
    ];
    files.sort_unstable();
    let mut manifest = String::new();
    for file in &files {
        let bytes = std::fs::read(root().join(file)).expect("a copied module");
        manifest.push_str(&format!("{}  {file}\n", hex(&Sha256::digest(&bytes))));
    }
    assert_eq!(
        hex(&Sha256::digest(manifest.as_bytes())),
        authority.checksum,
        "the profile differs from PrismPM ecb594bd: a copy is edited here, or the authority moved"
    );
    assert!(
        authority
            .citation
            .contains("ecb594bd5640f3cb4bdc567c096c20f03209bd0d"),
        "the authority names the PrismPM commit"
    );
    let entries = read("lexlean.toml");
    assert!(
        entries.contains("\"src/Deployment.lex.tex\""),
        "the deployment is an entrypoint, so lexlean verify reaches the profile"
    );
}

/// DP-02: the placement of the services, and the validity theorem, read from source.
#[test]
fn the_gateway_alone_is_on_the_host_network_dp_02() {
    let source = deployment();
    let components = declaration(&source, "definition", "components");
    let order: Vec<&str> = ["collector", "verifier", "signer", "engineHost", "gateway"].to_vec();
    let listed: String = order
        .iter()
        .map(|name| format!("(cons ({name}) "))
        .collect();
    assert!(
        components.contains(&listed),
        "components lists collector, verifier, signer, engine host and gateway, in that order"
    );
    for name in &order {
        let body = declaration(&source, "definition", name);
        let kind = field(&body, "kind");
        let networks = field(&body, "networks");
        let secrets = field(&body, "secrets");
        if *name == "gateway" {
            assert_eq!(kind, "\"api\"", "the gateway is the api");
            assert_eq!(networks, "(nil String)", "the gateway declares no network");
        } else {
            assert_ne!(kind, "\"api\"", "{name} is not an api");
            assert_eq!(
                networks, "(cons \"backend\" (nil String))",
                "{name} is on backend alone"
            );
        }
        if *name == "signer" {
            assert_eq!(
                secrets, "(cons \"lexlean-mcp-signing-key\" (nil String))",
                "the signer holds the signing key"
            );
        } else {
            assert_eq!(secrets, "(nil String)", "{name} holds no secret");
        }
    }
    let networks = declaration(&source, "definition", "networks");
    assert!(
        networks.contains("{id := \"backend\", internal := true}")
            && networks
                .matches("record Production.HostEffectSystem::Network")
                .count()
                == 1,
        "backend is the one network and it is internal"
    );
    let listeners = declaration(&source, "definition", "listeners");
    assert_eq!(
        listeners
            .matches("record Production.HostEffectSystem::Listener")
            .count(),
        1,
        "one listener"
    );
    assert_eq!(field(&listeners, "component"), "\"gateway\"");
    assert_eq!(field(&listeners, "port"), "(OwnerValues::listenPort)");
    assert_eq!(field(&listeners, "protocol"), "\"tcp\"");
    assert_eq!(
        field(&listeners, "bindAddress"),
        "(Production.HostEffectSystem::loopbackAddress)"
    );
    let profile = tight(&read("src/Production/HostEffectSystem.lex.tex"));
    assert!(
        profile
            .contains("name: loopbackAddress, parameters: [], result: String, body: \"127.0.0.1\""),
        "PrismPM's loopback address is 127.0.0.1"
    );
    let secrets = declaration(&source, "definition", "secrets");
    assert_eq!(
        field(&secrets, "consumers"),
        "(cons \"signer\" (nil String))"
    );

    let valid = declaration(&source, "theorem", "deploymentIsValidForEveryNamedBuild");
    assert!(
        valid.contains("parameters: [images : Images]")
            && valid.contains(
                "statement: ((Production.HostEffectSystem::validateManifest (systemModelFor images) (systemManifestFor)) = (imagesNamed images))"
            )
            && valid.contains("axioms: [propext]"),
        "the validity theorem is for every build and is an equation, not an implication: {valid}"
    );
    let named = declaration(&source, "definition", "imagesNamed");
    for image in ["collector", "verifier", "signer", "engineHost", "gateway"] {
        assert!(
            named.contains(&format!(
                "(Production.HostEffectSystem::nonEmpty (project images {image}))"
            )),
            "imagesNamed requires the {image} image to be named"
        );
    }
    assert_eq!(
        named.matches("nonEmpty").count(),
        5,
        "imagesNamed asks for the five images and nothing else"
    );
    let images = declaration(&source, "structure", "Images");
    assert_eq!(images.matches("type: String").count(), 5, "five images");
    for theorem in [
        "theGatewayIsTheOnlyApi",
        "theListenerIsTheOwnerSetPortOnLoopback",
        "everyOtherServiceIsOnTheInternalNetworkAlone",
        "theSigningKeyIsTheSignersAlone",
    ] {
        let body = declaration(&source, "theorem", theorem);
        assert!(
            body.contains("proof: decide{}") && body.contains("axioms: []"),
            "{theorem} is decided with no axiom"
        );
    }
}

/// DP-03: the application digest is the gateway's declaration.
#[test]
fn the_application_digest_is_the_gateway_declaration_dp_03() {
    let declared = std::fs::read(root().join("gateway/src/Application.lex.tex"))
        .expect("the gateway's application declaration");
    let expected = format!("\"sha256:{}\"", hex(&Sha256::digest(&declared)));
    let source = deployment();
    let digest = declaration(&source, "definition", "gatewayApplicationDigest");
    assert!(
        digest.ends_with(&format!("body: {expected}}}")),
        "gatewayApplicationDigest is not {expected}: the gateway's application changed, so the deployment binds another application"
    );
    for name in ["profileFor", "systemManifestFor"] {
        let body = declaration(&source, "definition", name);
        assert_eq!(
            field(&body, "applicationModelDigest"),
            "(gatewayApplicationDigest)",
            "{name} binds gatewayApplicationDigest"
        );
    }
}

/// DP-04: the effects are the gateway's host protocol.
#[test]
fn the_effects_are_the_gateway_host_protocol_dp_04() {
    let application = read("gateway/src/Application.lex.tex");
    let marker = "record Foundation.Server.V1.Model::HostEffect {name := \"";
    let gateway: Vec<String> = application
        .split(marker)
        .skip(1)
        .map(|rest| rest.split_once('"').expect("a closed name").0.to_string())
        .collect();
    assert_eq!(gateway.len(), 15, "the gateway declares 15 host effects");

    let source = deployment();
    let stated = declaration(&source, "theorem", "theEffectsAreTheGatewaysHostProtocol");
    let statement = stated
        .split_once("statement: ((hostEffects) = ")
        .expect("the theorem states hostEffects")
        .1
        .split_once("proof:")
        .expect("a proof")
        .0;
    assert_eq!(
        quoted(statement),
        gateway,
        "the deployment's effects are not the gateway's, in order"
    );
    assert!(
        stated.contains("proof: decide{}"),
        "the statement is decided"
    );

    let effects = tight(&read("src/Effects.lex.tex"));
    let names = declaration(&effects, "definition", "effectName");
    let all = declaration(&effects, "definition", "allEffects");
    let by_constructor: Vec<(String, String)> = names
        .split("| Effect.")
        .skip(1)
        .map(|arm| {
            let (constructor, rest) = arm.split_once(" => \"").expect("an arm");
            (
                constructor.to_string(),
                rest.split_once('"').expect("a name").0.to_string(),
            )
        })
        .collect();
    let in_order: Vec<String> = all
        .split("(ctor Effect.")
        .skip(1)
        .map(|rest| {
            let constructor = rest.split_once(')').expect("a constructor").0;
            by_constructor
                .iter()
                .find(|(c, _)| c == constructor)
                .unwrap_or_else(|| panic!("effectName has no arm for {constructor}"))
                .1
                .clone()
        })
        .collect();
    assert_eq!(in_order.len(), 10, "allEffects is E1 to E10");
    assert_eq!(
        &gateway[..10],
        &in_order[..],
        "the gateway's first ten effects are not E1 to E10 in the order of allEffects"
    );
    let rest = declaration(&source, "definition", "frameAndTaskEffects");
    assert_eq!(quoted(&rest), gateway[10..].to_vec());
    let host = declaration(&source, "definition", "hostEffects");
    assert!(
        host.contains("(effectNamesOf (Effects::allEffects)) (frameAndTaskEffects)"),
        "hostEffects is allEffects' names, then the frame and task effects"
    );
}
