use qos_nsm::{
	types::{NsmRequest, NsmResponse},
	NsmProvider,
};

use crate::protocol::{ProtocolError, ProtocolState, QosHash};

pub(in crate::protocol) fn live_attestation_doc(
	state: &mut ProtocolState,
) -> Result<NsmResponse, ProtocolError> {
	let ephemeral_public_key =
		state.handles.get_ephemeral_key()?.public_key().to_bytes();
	let manifest_hash =
		state.handles.get_manifest_envelope()?.manifest.qos_hash().to_vec();

	Ok(get_post_boot_attestation_doc(
		&*state.attestor,
		ephemeral_public_key,
		manifest_hash,
	))
}

pub(super) fn get_post_boot_attestation_doc(
	attestor: &dyn NsmProvider,
	ephemeral_public_key: Vec<u8>,
	manifest_hash: Vec<u8>,
) -> NsmResponse {
	let request = NsmRequest::Attestation {
		user_data: Some(manifest_hash),
		nonce: None,
		public_key: Some(ephemeral_public_key),
	};

	attestor.nsm_process_request(request)
}
