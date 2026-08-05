/// Controls the status of the replacement email address or phone number. Defaults to verified. Set to reserved
/// to create it reserved (unverified but usable for sign-in and locked) instead of verified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum IdentificationStatus {
	#[serde(rename = "verified")]
	Verified,
	#[serde(rename = "reserved")]
	Reserved,
}
