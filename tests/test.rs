use schemafy_core;

use serde_json;

use serde_derive::{Deserialize, Serialize};

schemafy::schemafy!(
    root: Schema
    "src/schema.json"
);

fn schema_exists(schema: Option<&Schema>) {
    if let Some(schema) = schema {
        let _ = &schema.type_;
    }
}

fn types_exists(_: Option<(&SimpleTypes, PositiveInteger)>) {}

#[test]
fn test() {
    schema_exists(None);
    types_exists(None);
}

schemafy::schemafy!("tests/debugserver-schema.json");

#[test]
fn debugserver_types() {
    let request: Option<SourceRequest> = None;
    if let Some(r) = request {
        let _: &SourceArguments = &r.arguments;
    }
}

schemafy::schemafy!("tests/nested.json");

#[test]
fn nested() {
    let _: Option<Defnested> = None;
}

schemafy::schemafy!("tests/vega/vega.json");

schemafy::schemafy!(
    root: OptionType
    "tests/option-type.json"
);

#[test]
fn option_type() {
    let o: Option<OptionType> = None;
    if let Some(o) = o {
        let _: Option<i64> = o.optional;
    }
}

schemafy::schemafy!(
    root: EmptyStruct
    "tests/empty-struct.json"
);

#[test]
fn empty_struct() {
    let EmptyStruct {} = EmptyStruct {};
}

schemafy::schemafy!(
    root: AnyProperties
    "tests/any-properties.json"
);

#[test]
fn any_properties() {
    let _: ::std::collections::BTreeMap<String, serde_json::Value> = AnyProperties::default();
}

schemafy::schemafy!(
    root: RootArray
    "tests/root-array.json"
);

#[test]
fn root_array() {
    let a = RootArray::default();
    let _: Option<&RootArrayItem> = a.get(0);
}
