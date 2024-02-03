use serde::Deserialize;
use sea_orm::DatabaseConnection;

#[derive(Deserialize, Debug)]
pub struct TamagotchiId {
    pub fid: i64,
}

#[derive(Deserialize, Debug)]
pub struct CastId {
    fid: i64,
    hash: String,
}

impl CastId {
    // Getter method for `fid`
    pub fn get_fid(&self) -> i64 {
        self.fid
    }

    // Getter method for `hash`
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
}

#[derive(Deserialize, Debug)]
pub struct UntrustedData {
    fid: i64,
    url: String,
    messageHash: String,
    timestamp: u64,
    network: u8,
    buttonIndex: u8,
    castId: CastId,
}

impl UntrustedData {
    // Internal getter method for `fid`
    fn get_fid(&self) -> i64 {
        self.fid
    }

    fn get_button_index(&self) -> u8 {
        self.buttonIndex
    }
}

#[derive(Deserialize, Debug)]
pub struct TrustedData {
    messageBytes: String,
}

impl TrustedData {
    // Internal getter method for `messageBytes`
    fn get_message_bytes(&self) -> String {
        self.messageBytes.clone()
    }
}

#[derive(Deserialize, Debug)]
pub struct FrameData {
    pub untrustedData: UntrustedData,
    pub trustedData: TrustedData,
}

impl FrameData {
    pub fn get_fid(&self) -> i64 {
        // Access `fid` using the `UntrustedData`'s getter method
        self.untrustedData.get_fid()
    }

    pub fn get_button_index(&self) -> u8 {
        // Access `buttonIndex` using the `UntrustedData`'s getter method
        self.untrustedData.get_button_index()
    }

    pub fn get_message_bytes(&self) -> String {
        // Access `messageBytes` using the `TrustedData`'s getter method
        self.trustedData.get_message_bytes()
    }
}
