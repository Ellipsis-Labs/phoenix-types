pub mod dispatch;
pub mod enums;
pub mod events;
pub mod instructions;
pub mod market;
pub mod order_packet;

solana_sdk::declare_id!("phnxNHfGNVjpVVuHkceK3MgwZ1bW25ijfWACKhVFbBH");

/// This is a static PDA with seeds: [b"log"]
/// If the program id changes, this will also need to be updated
pub mod phoenix_log_authority {
    use solana_sdk::pubkey::Pubkey;

    solana_sdk::declare_id!("5v5A5drhYS59hECzjFyGdJFgcwAVjALEPUE1m5ydoLew");
    pub const BUMP: u8 = 254;

    pub fn get_log_authority() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"log"], &super::id())
    }
}
