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
pub struct ProviderUserConfirmRequestModel {
    #[serde(rename = "key")]
    pub key: String,
}

impl ProviderUserConfirmRequestModel {
    pub fn new(key: String) -> ProviderUserConfirmRequestModel {
        ProviderUserConfirmRequestModel { key }
    }
}