#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::str_to_string,
    clippy::bool_assert_comparison,
    dead_code
)]

use gts::{GtsInstanceId, GtsSchema, GtsSchemaId};
use gts_macros::GtsSchema;
use schemars::JsonSchema;
use uuid::Uuid;

// =============================================================================
// Test structs
// =============================================================================

/// A base event type with a type field (anonymous instance pattern).
///
/// Note: In Phase 2 we do NOT derive Serialize/Deserialize on generic structs
/// because serde bound injection (#[serde(bound(...))]) is a Phase 3 concern.
/// The struct still gets the `GtsSchema` trait impl and runtime API.
#[derive(Debug, Clone, JsonSchema, GtsSchema)]
#[gts(
    dir_path = "schemas",
    schema_id = "gts.x.core.events.type.v1~",
    description = "Base event type"
)]
pub struct BaseEventV1<P: GtsSchema + JsonSchema> {
    #[gts(type_field)]
    #[serde(rename = "type")]
    pub event_type: GtsSchemaId,
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub sequence_id: u64,
    pub payload: P,
}

/// A topic with an instance ID (well-known instance pattern).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, GtsSchema)]
#[gts(
    dir_path = "schemas",
    schema_id = "gts.x.core.events.topic.v1~",
    description = "Event topic definition"
)]
pub struct EventTopicV1 {
    #[gts(instance_id)]
    pub id: GtsInstanceId,
    pub name: String,
    pub description: Option<String>,
    pub retention: String,
}

/// A data entity without any GTS identity field (fixes Issue #72).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, GtsSchema)]
#[gts(
    dir_path = "schemas",
    schema_id = "gts.x.commerce.orders.order.v1.0~",
    description = "Order data entity"
)]
pub struct OrderV1_0 {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity: u32,
    pub total: f64,
}

/// A struct with #[gts(skip)] field.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, GtsSchema)]
#[gts(
    dir_path = "schemas",
    schema_id = "gts.x.test.entities.product.v1~",
    description = "Product entity"
)]
pub struct ProductV1 {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    #[gts(skip)]
    pub internal_cache: Option<String>,
}

/// A struct with a `GtsSchemaId` field that is NOT annotated with `#[gts(type_field)]`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, GtsSchema)]
#[gts(
    dir_path = "schemas",
    schema_id = "gts.x.test.entities.ref_holder.v1~",
    description = "Holds a reference to another GTS schema"
)]
pub struct RefHolderV1 {
    pub id: Uuid,
    pub subject_type: GtsSchemaId,
}

// =============================================================================
// Tests: Schema ID and constants
// =============================================================================

#[test]
fn base_struct_schema_id() {
    let id = BaseEventV1::<()>::gts_schema_id();
    assert_eq!(id.as_ref(), "gts.x.core.events.type.v1~");
}

#[test]
fn base_struct_schema_constants() {
    assert_eq!(
        <BaseEventV1<()> as GtsSchema>::SCHEMA_ID,
        "gts.x.core.events.type.v1~"
    );
    assert_eq!(
        BaseEventV1::<()>::GTS_SCHEMA_FILE_PATH,
        "schemas/gts.x.core.events.type.v1~.schema.json"
    );
    assert_eq!(BaseEventV1::<()>::GTS_SCHEMA_DESCRIPTION, "Base event type");
    // Properties should include all fields (no skipped fields)
    let props = BaseEventV1::<()>::GTS_SCHEMA_PROPERTIES;
    assert!(props.contains("type")); // serde rename
    assert!(props.contains("id"));
    assert!(props.contains("tenant_id"));
    assert!(props.contains("sequence_id"));
    assert!(props.contains("payload"));
}

#[test]
fn base_struct_instance_id() {
    let instance_id =
        BaseEventV1::<()>::gts_make_instance_id("x.commerce.orders.order_placed.v1.0");
    assert_eq!(
        instance_id.as_ref(),
        "gts.x.core.events.type.v1~x.commerce.orders.order_placed.v1.0"
    );
}

#[test]
fn base_struct_base_schema_id_none() {
    assert!(BaseEventV1::<()>::gts_base_schema_id().is_none());
}

#[test]
fn base_struct_no_gts_identity_field() {
    // Issue #72: struct without id/type field should compile and produce valid schema
    let id = OrderV1_0::gts_schema_id();
    assert_eq!(id.as_ref(), "gts.x.commerce.orders.order.v1.0~");

    let schema_str = OrderV1_0::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    assert_eq!(schema["$id"], "gts://gts.x.commerce.orders.order.v1.0~");
    assert_eq!(schema["type"], "object");

    let props = schema["properties"].as_object().unwrap();
    assert!(props.contains_key("id"));
    assert!(props.contains_key("product_id"));
    assert!(props.contains_key("quantity"));
    assert!(props.contains_key("total"));
}

// =============================================================================
// Tests: Schema structure
// =============================================================================

#[test]
fn schema_has_id() {
    let schema_str = EventTopicV1::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    assert_eq!(schema["$id"], "gts://gts.x.core.events.topic.v1~");
}

#[test]
fn schema_has_json_schema_ref() {
    let schema_str = EventTopicV1::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    assert_eq!(schema["$schema"], "http://json-schema.org/draft-07/schema#");
}

#[test]
fn schema_type_object() {
    let schema_str = EventTopicV1::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    assert_eq!(schema["type"], "object");
}

#[test]
fn schema_additional_properties_false() {
    let schema_str = EventTopicV1::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    assert_eq!(schema["additionalProperties"], false);
}

#[test]
fn schema_required_fields() {
    let schema_str = EventTopicV1::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    let required = schema["required"].as_array().unwrap();
    // Non-Option fields should be required
    assert!(required.contains(&serde_json::json!("id")));
    assert!(required.contains(&serde_json::json!("name")));
    assert!(required.contains(&serde_json::json!("retention")));
}

#[test]
fn schema_optional_fields_not_required() {
    let schema_str = EventTopicV1::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    let required = schema["required"].as_array().unwrap();
    // Option<T> fields should NOT be required
    assert!(!required.contains(&serde_json::json!("description")));
}

#[test]
fn base_struct_schema_has_description() {
    let schema_str = EventTopicV1::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    assert_eq!(schema["description"], "Event topic definition");
}

#[test]
fn base_struct_schema_pretty() {
    let pretty = EventTopicV1::gts_schema_with_refs_as_string_pretty();
    // Should be valid JSON and contain newlines
    let _: serde_json::Value = serde_json::from_str(&pretty).unwrap();
    assert!(pretty.contains('\n'));
}

// =============================================================================
// Tests: x-gts-ref behavior
// =============================================================================

#[test]
fn base_struct_with_type_field() {
    // #[gts(type_field)] should produce x-gts-ref: "/$id"
    let schema_str = BaseEventV1::<()>::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    let type_prop = &schema["properties"]["type"];
    assert_eq!(
        type_prop["x-gts-ref"], "/$id",
        "type_field should have x-gts-ref: /$id"
    );
}

#[test]
fn base_struct_with_instance_id() {
    // #[gts(instance_id)] should produce x-gts-ref: "/$id"
    let schema_str = EventTopicV1::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    let id_prop = &schema["properties"]["id"];
    assert_eq!(
        id_prop["x-gts-ref"], "/$id",
        "instance_id should have x-gts-ref: /$id"
    );
}

#[test]
fn base_struct_other_gts_fields() {
    // Non-annotated GtsSchemaId fields should retain x-gts-ref: "gts.*"
    let schema_str = RefHolderV1::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    let subject_type_prop = &schema["properties"]["subject_type"];
    assert_eq!(
        subject_type_prop["x-gts-ref"], "gts.*",
        "Non-annotated GtsSchemaId should retain x-gts-ref: gts.*"
    );
}

// =============================================================================
// Tests: #[gts(skip)] and #[serde(skip)]
// =============================================================================

#[test]
fn base_struct_gts_skip() {
    // #[gts(skip)] field should NOT appear in GTS_SCHEMA_PROPERTIES
    let props = ProductV1::GTS_SCHEMA_PROPERTIES;
    assert!(
        !props.contains("internal_cache"),
        "gts(skip) field should be excluded from properties"
    );
    assert!(props.contains("id"));
    assert!(props.contains("name"));
    assert!(props.contains("price"));
}

// =============================================================================
// Tests: Generic field detection
// =============================================================================

#[test]
fn base_struct_with_generic() {
    assert_eq!(
        <BaseEventV1<()> as GtsSchema>::GENERIC_FIELD,
        Some("payload")
    );
}

#[test]
fn base_struct_no_generic() {
    assert_eq!(<EventTopicV1 as GtsSchema>::GENERIC_FIELD, None);
}

// =============================================================================
// Tests: GtsSchemaId / GtsInstanceId format in schema
// =============================================================================

#[test]
fn schema_gts_schema_id_format() {
    let schema_str = RefHolderV1::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    let subject_type_prop = &schema["properties"]["subject_type"];
    assert_eq!(subject_type_prop["type"], "string");
    assert_eq!(subject_type_prop["format"], "gts-schema-id");
}

#[test]
fn schema_gts_instance_id_format() {
    let schema_str = EventTopicV1::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    let id_prop = &schema["properties"]["id"];
    assert_eq!(id_prop["type"], "string");
    assert_eq!(id_prop["format"], "gts-instance-id");
}

#[test]
fn schema_uuid_format() {
    let schema_str = OrderV1_0::gts_schema_with_refs_as_string();
    let schema: serde_json::Value = serde_json::from_str(&schema_str).unwrap();
    let id_prop = &schema["properties"]["id"];
    assert_eq!(id_prop["format"], "uuid");
}
