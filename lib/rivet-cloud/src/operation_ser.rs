// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_add_namespace_domain(
	input: &crate::input::AddNamespaceDomainInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_add_namespace_domain_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_game(
	input: &crate::input::CreateGameInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_create_game_input(&mut object, input)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_game_build(
	input: &crate::input::CreateGameBuildInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_create_game_build_input(&mut object, input)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_game_cdn_site(
	input: &crate::input::CreateGameCdnSiteInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_create_game_cdn_site_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_game_namespace(
	input: &crate::input::CreateGameNamespaceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_create_game_namespace_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_game_namespace_token_development(
	input: &crate::input::CreateGameNamespaceTokenDevelopmentInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_create_game_namespace_token_development_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_game_version(
	input: &crate::input::CreateGameVersionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_create_game_version_input(&mut object, input)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_export_lobby_logs(
	input: &crate::input::ExportLobbyLogsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_export_lobby_logs_input(&mut object, input)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_export_matchmaker_lobby_history(
	input: &crate::input::ExportMatchmakerLobbyHistoryInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_export_matchmaker_lobby_history_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_game_banner_upload_prepare(
	input: &crate::input::GameBannerUploadPrepareInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_game_banner_upload_prepare_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_game_logo_upload_prepare(
	input: &crate::input::GameLogoUploadPrepareInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_game_logo_upload_prepare_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_group_billing_checkout(
	input: &crate::input::GroupBillingCheckoutInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_group_billing_checkout_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_prepare_custom_avatar_upload(
	input: &crate::input::PrepareCustomAvatarUploadInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_prepare_custom_avatar_upload_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_set_namespace_cdn_auth_type(
	input: &crate::input::SetNamespaceCdnAuthTypeInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_set_namespace_cdn_auth_type_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_toggle_namespace_domain_public_auth(
	input: &crate::input::ToggleNamespaceDomainPublicAuthInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_toggle_namespace_domain_public_auth_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_game_namespace_matchmaker_config(
	input: &crate::input::UpdateGameNamespaceMatchmakerConfigInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_update_game_namespace_matchmaker_config_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_game_namespace_version(
	input: &crate::input::UpdateGameNamespaceVersionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_update_game_namespace_version_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_update_namespace_cdn_auth_user(
	input: &crate::input::UpdateNamespaceCdnAuthUserInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_update_namespace_cdn_auth_user_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_validate_game(
	input: &crate::input::ValidateGameInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_validate_game_input(&mut object, input)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_validate_game_namespace(
	input: &crate::input::ValidateGameNamespaceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_validate_game_namespace_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_validate_game_namespace_matchmaker_config(
	input: &crate::input::ValidateGameNamespaceMatchmakerConfigInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_validate_game_namespace_matchmaker_config_input(&mut object, input)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_validate_game_namespace_token_development(
	input: &crate::input::ValidateGameNamespaceTokenDevelopmentInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_validate_game_namespace_token_development_input(&mut object, input)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_validate_game_version(
	input: &crate::input::ValidateGameVersionInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_validate_game_version_input(
		&mut object,
		input,
	)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_validate_group(
	input: &crate::input::ValidateGroupInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::SerializationError> {
	let mut out = String::new();
	let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
	crate::json_ser::serialize_structure_crate_input_validate_group_input(&mut object, input)?;
	object.finish();
	Ok(aws_smithy_http::body::SdkBody::from(out))
}
