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
pub struct RegisterResponseModel {
    #[serde(rename = "object", skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "captchaBypassToken", skip_serializing_if = "Option::is_none")]
    pub captcha_bypass_token: Option<String>,
}

impl RegisterResponseModel {
    pub fn new() -> RegisterResponseModel {
        RegisterResponseModel {
            object: None,
            captcha_bypass_token: None,
        }
    }
}