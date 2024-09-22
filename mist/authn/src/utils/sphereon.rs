// It's unclear to me why these are needed. The spec itself doesn't mention
// anything about them, so I think they're specific to Sphereon?
// -------------------------------------------------------------------------

use serde::Deserialize;
use ssi::vc::Credential;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SphereonTokenWrapper {
    pub(crate) verifiable_credential: Vec<String>,
}

#[derive(Deserialize)]
pub(crate) struct SphereonCredentialWrapper {
    pub(crate) vc: Credential,
}
