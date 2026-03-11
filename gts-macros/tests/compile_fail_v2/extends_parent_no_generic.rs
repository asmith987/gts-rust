//! Test: extends = Parent requires parent to have a generic field.
//! A derived struct cannot extend a parent that has no generic field.

use gts_macros::GtsSchema;
use schemars::JsonSchema;

// Parent struct with NO generic field (leaf/terminal type)
#[derive(Debug, JsonSchema, GtsSchema)]
#[gts(
    dir_path = "schemas",
    schema_id = "gts.x.app.entities.leaf.v1~",
    description = "Leaf type with no generic field"
)]
pub struct LeafTypeV1 {
    pub name: String,
}

// This should fail: trying to extend a parent with no generic field
#[derive(Debug, JsonSchema, GtsSchema)]
#[gts(
    dir_path = "schemas",
    schema_id = "gts.x.app.entities.leaf.v1~x.app.entities.child.v1~",
    description = "Child trying to extend leaf type (invalid)",
    extends = LeafTypeV1
)]
pub struct ChildOfLeafV1 {
    pub extra_field: String,
}

fn main() {}
