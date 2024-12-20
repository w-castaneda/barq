// barq-plugin/src/methods/offer.rs

/// Represents a Bolt12 Offer.
pub struct Offer {
    pub amount: u64,                  // Amount the offer is for.
    pub description: String,          // Description of the offer.
    pub expiration: Option<u64>,      // Optional expiration time for the offer.
    pub min_cltv_expiry: u32,         // Minimum CLTV expiry.
    pub max_accepted_htlc_count: u32, // Maximum number of HTLCs.
}
