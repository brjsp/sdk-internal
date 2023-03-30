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
pub struct EnvironmentConfigResponseModel {
    #[serde(rename = "vault", skip_serializing_if = "Option::is_none")]
    pub vault: Option<String>,
    #[serde(rename = "api", skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "notifications", skip_serializing_if = "Option::is_none")]
    pub notifications: Option<String>,
    #[serde(rename = "sso", skip_serializing_if = "Option::is_none")]
    pub sso: Option<String>,
}

impl EnvironmentConfigResponseModel {
    pub fn new() -> EnvironmentConfigResponseModel {
        EnvironmentConfigResponseModel {
            vault: None,
            api: None,
            identity: None,
            notifications: None,
            sso: None,
        }
    }
}