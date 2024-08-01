use global_error::prelude::*;
use url::Url;

pub async fn get_namespace_url(
	namespace: String,
	ctx: &std::sync::Arc<toolchain_core::ctx::CtxInner>,
) -> GlobalResult<Url> {
	// Build the URL from the game ID and the namespace
	let mut url = unwrap!(Url::parse(&format!(
		"{}/games/{}/namespaces/{}",
		ctx.api_endpoint, ctx.game_id, namespace
	)));

	// Parse the URL and change the subdomain from `api` to `hub`
	let host = url.host_str().unwrap().replace("api", "hub");
	url.set_host(Some(&host)).unwrap();
	Ok(url)
}
