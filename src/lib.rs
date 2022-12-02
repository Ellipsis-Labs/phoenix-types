pub mod dispatch;
pub mod enums;
pub mod events;
pub mod instructions;
pub mod market;
pub mod order_packet;

// You need to import Pubkey prior to using the declare_id macro
use ellipsis_macros::declare_id;
use solana_sdk::pubkey::Pubkey;
declare_id!("phnxNHfGNVjpVVuHkceK3MgwZ1bW25ijfWACKhVFbBH");

/// This is a static PDA with seeds: [b"log"]
/// If the program id changes, this will also need to be updated
pub mod phoenix_log_authority {
    // You need to import Pubkey prior to using the declare_pda macro
    use ellipsis_macros::declare_pda;
    use solana_sdk::pubkey::Pubkey;

    declare_pda!(
        "5v5A5drhYS59hECzjFyGdJFgcwAVjALEPUE1m5ydoLew",
        "phnxNHfGNVjpVVuHkceK3MgwZ1bW25ijfWACKhVFbBH",
        "log"
    );

    pub fn get_log_authority() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[b"log"], &super::id())
    }
}
