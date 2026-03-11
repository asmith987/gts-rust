//! Test: allow_direct_serde without extends is an error

use gts_macros::GtsSchema;
use schemars::JsonSchema;

#[derive(Debug, JsonSchema, GtsSchema)]
#[gts(
    dir_path = "schemas",
    schema_id = "gts.x.app.entities.thing.v1~",
    description = "Root type with allow_direct_serde (meaningless)",
    allow_direct_serde
)]
pub struct ThingV1 {
    pub name: String,
}

fn main() {}
