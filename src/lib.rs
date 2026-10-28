use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UNAgencyState {
    pub agency_id: u32,
    pub agency_name: String,
    pub project_name: String,
    pub portfolio: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultilateralAuditRecord {
    pub audit_id: String,
    pub timestamp: u64,
    pub merkle_root_hash: String,
    pub verified_by_pqc: bool,
}

pub fn get_metadata() -> UNAgencyState {
    UNAgencyState {
        agency_id: 22,
        agency_name: "UN Office on Drugs and Crime (UNODC)".to_string(),
        project_name: "UNODC MaritimeInterdict AI".to_string(),
        portfolio: "Counter-Narcotics, Crime & AML".to_string(),
        strategy: "Tokenless Cryptographic Audit Anchor -- Global Counter-Narcotics and Asset Recovery Evidence".to_string(),
        target_chain: "Internet Computer (ICP) Air-Gapped".to_string(),
        is_active: true,
    }
}
