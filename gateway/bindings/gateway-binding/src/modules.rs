//! The closed `view` of `store.get` and `form` of `store.put` (issue #9, EF-09), and the four
//! functions of `lexlean::sdk` they reach.
//!
//! A view reads bytes the store holds: `0` as stored, `1` as the module of `sdk::read_module`
//! (JSON), `2` as the byte ranges of `sdk::declaration_spans` (JSON, `null` for a module written
//! as data). A form says how a put stores its content: `0` as given, `1` as the text
//! `sdk::assemble` makes of `{kind, header, data}`, `2` as the text `sdk::write_module` makes of
//! `{text, data}`. The numbers are those of `View`, `Form` and `Class` in `src/Effects.lex.tex`;
//! a number outside them is not a view or a form, and the host refuses it before it reads or
//! stores anything, so the sdk is reached for a value of the model only.
//!
//! The sdk is for semantic, tooling and core modules. Bytes that are not one (a prose document,
//! text that is not UTF-8) are refused by the sdk, and the refusal travels as the class `1` of
//! the answer with the message of the sdk; the host never converts them.

use lexlean::sdk::{self, ModuleKind, SourceHeader};
use serde_json::Value;

/// The class of an answer whose bytes the sdk took.
pub const ACCEPTED: u32 = 0;
/// The class of an answer the sdk refused: its content is the message of the sdk.
pub const REFUSED: u32 = 1;
/// The class of an answer for a draft that has no such path.
pub const ABSENT: u32 = 2;

/// How `store.get` reads the bytes it found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    /// The bytes as stored.
    Stored,
    /// The module as JSON, as `sdk::read_module` reads it.
    Data,
    /// The byte ranges of the declarations, as `sdk::declaration_spans` gives them.
    Spans,
}

impl View {
    /// The view that has this number, if there is one.
    pub fn from_number(number: u32) -> Option<View> {
        match number {
            0 => Some(View::Stored),
            1 => Some(View::Data),
            2 => Some(View::Spans),
            _ => None,
        }
    }
}

/// How `store.put` makes the bytes it stores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    /// The content as given.
    Plain,
    /// The text of `sdk::assemble` for `{kind, header, data}`.
    Assemble,
    /// The text of `sdk::write_module` for `{text, data}`.
    Rewrite,
}

impl Form {
    /// The form that has this number, if there is one.
    pub fn from_number(number: u32) -> Option<Form> {
        match number {
            0 => Some(Form::Plain),
            1 => Some(Form::Assemble),
            2 => Some(Form::Rewrite),
            _ => None,
        }
    }
}

fn utf8(bytes: &[u8]) -> Result<&str, String> {
    std::str::from_utf8(bytes).map_err(|_| "the bytes are not UTF-8".to_string())
}

/// The bytes of a view of `bytes` (`Data` and `Spans`), or the message of the sdk. A result
/// longer than `limit` is refused with a message, as the host returns no more than it stores.
pub fn view(view: View, bytes: &[u8], limit: usize) -> Result<Vec<u8>, String> {
    let produced = match view {
        View::Stored => bytes.to_vec(),
        View::Data => {
            let module = sdk::read_module(utf8(bytes)?).map_err(|error| error.to_string())?;
            serde_json::to_vec(&module).map_err(|error| error.to_string())?
        }
        View::Spans => {
            let spans = sdk::declaration_spans(utf8(bytes)?).map_err(|error| error.to_string())?;
            serde_json::to_vec(&spans).map_err(|error| error.to_string())?
        }
    };
    if produced.len() > limit {
        return Err(format!(
            "the result of the view is {} bytes, more than the {limit} the host returns",
            produced.len()
        ));
    }
    Ok(produced)
}

fn object<'a>(
    value: &'a Value,
    what: &str,
    keys: &[&str],
) -> Result<&'a serde_json::Map<String, Value>, String> {
    let map = value
        .as_object()
        .ok_or_else(|| format!("{what} is not a JSON object"))?;
    for key in map.keys() {
        if !keys.contains(&key.as_str()) {
            return Err(format!(
                "{what} has the member {key}, which is not one of {keys:?}"
            ));
        }
    }
    for key in keys {
        if !map.contains_key(*key) {
            return Err(format!("{what} lacks the member {key}"));
        }
    }
    Ok(map)
}

fn text<'a>(map: &'a serde_json::Map<String, Value>, key: &str) -> Result<&'a str, String> {
    map[key]
        .as_str()
        .ok_or_else(|| format!("the member {key} is not a string"))
}

fn words(map: &serde_json::Map<String, Value>, key: &str) -> Result<Vec<String>, String> {
    map[key]
        .as_array()
        .ok_or_else(|| format!("the member {key} is not a list"))?
        .iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .ok_or_else(|| format!("the member {key} holds an item that is not a string"))
        })
        .collect()
}

fn kind_of(name: &str) -> Result<ModuleKind, String> {
    match name {
        "semantic" => Ok(ModuleKind::Semantic),
        "tooling" => Ok(ModuleKind::Tooling),
        "core" => Ok(ModuleKind::Core),
        other => Err(format!(
            "the kind {other:?} is not semantic, tooling or core"
        )),
    }
}

/// The text a form makes of its content (`Assemble` and `Rewrite`), or why it cannot: the
/// request is not the JSON the form states, or the sdk refuses the module. A text longer than
/// `limit` is refused too.
pub fn render(form: Form, request: &[u8], limit: usize) -> Result<String, String> {
    let value: Value = serde_json::from_slice(request)
        .map_err(|error| format!("the content is not JSON: {error}"))?;
    let made = match form {
        Form::Plain => return Err("the form Plain makes nothing".to_string()),
        Form::Assemble => {
            let top = object(&value, "the content", &["kind", "header", "data"])?;
            let kind = kind_of(text(top, "kind")?)?;
            let header = object(
                &top["header"],
                "the header",
                &["name", "glossaries", "imports", "title"],
            )?;
            let glossaries = words(header, "glossaries")?;
            let imports = words(header, "imports")?;
            let glossaries: Vec<&str> = glossaries.iter().map(String::as_str).collect();
            let imports: Vec<&str> = imports.iter().map(String::as_str).collect();
            sdk::assemble(
                &SourceHeader {
                    name: text(header, "name")?,
                    glossaries: &glossaries,
                    imports: &imports,
                    title: text(header, "title")?,
                },
                kind,
                &top["data"],
            )
            .map_err(|error| error.to_string())?
        }
        Form::Rewrite => {
            let top = object(&value, "the content", &["text", "data"])?;
            sdk::write_module(text(top, "text")?, &top["data"])
                .map_err(|error| error.to_string())?
        }
    };
    if made.len() > limit {
        return Err(format!(
            "the text is {} bytes, more than the {limit} the host stores",
            made.len()
        ));
    }
    Ok(made)
}
