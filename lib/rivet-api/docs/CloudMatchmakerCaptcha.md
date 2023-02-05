# CloudMatchmakerCaptcha

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**requests_before_reverify** | Option<**f64**> | Denotes how many requests a connection can make before it is required to reverify a captcha. | [optional]
**verification_ttl** | Option<**f64**> | Denotes how long a connection can continue to reconnect without having to reverify a captcha (in milliseconds). | [optional]
**hcaptcha** | Option<[**crate::models::CloudMatchmakerCaptchaHcaptcha**](CloudMatchmakerCaptchaHcaptcha.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


