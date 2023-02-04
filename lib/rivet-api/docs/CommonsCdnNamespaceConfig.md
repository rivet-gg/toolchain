# CommonsCdnNamespaceConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**enable_domain_public_auth** | Option<**bool**> | Whether or not to allow users to connect to the given namespace via domain name. | [optional]
**domains** | [**Vec<crate::models::CommonsCdnNamespaceDomain>**](CommonsCdnNamespaceDomain.md) | A list of CDN domains for a given namespace. | 
**auth_type** | [**crate::models::CommonsCdnAuthType**](CommonsCdnAuthType.md) |  | 
**auth_user_list** | [**Vec<crate::models::CommonsCdnNamespaceAuthUser>**](CommonsCdnNamespaceAuthUser.md) | A list of CDN authenticated users for a given namespace. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


