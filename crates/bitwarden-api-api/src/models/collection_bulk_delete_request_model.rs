/*
 * Bitwarden Internal API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CollectionBulkDeleteRequestModel {
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
    #[serde(rename = "organizationId", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
}

impl CollectionBulkDeleteRequestModel {
    pub fn new(ids: Vec<String>) -> CollectionBulkDeleteRequestModel {
        CollectionBulkDeleteRequestModel {
            ids,
            organization_id: None,
        }
    }
}