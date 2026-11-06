use std::env;

use orchard::keys::{FullViewingKey, SpendingKey};

/// The wallet's viewing key for a seed, exactly as the node derives it.
fn fvk_bytes_from_seed(seed: [u8; 32]) -> Option<[u8; 96]> {
    let sk = Option::<SpendingKey>::from(SpendingKey::from_bytes(seed))?;
    Some(FullViewingKey::from(&sk).to_bytes())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seed_hex = env::args().nth(1).ok_or("usage: derive_merchant <32-byte-seed-hex>")?;
    let seed: [u8; 32] = hex::decode(seed_hex)?.try_into().map_err(|_| "seed must be exactly 32 bytes")?;
    let fvk = fvk_bytes_from_seed(seed).ok_or("seed does not derive a valid Orchard full viewing key")?;
    println!("{}", hex::encode(fvk));
    Ok(())
}
