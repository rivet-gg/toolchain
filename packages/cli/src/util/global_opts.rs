use clap::Parser;
use serde::Serialize;

#[derive(Parser, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalOpts {}
