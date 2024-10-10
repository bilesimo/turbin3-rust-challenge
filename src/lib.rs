#[cfg(test)] 
mod tests {
  use solana_sdk::signature::{Keypair, Signer};
  use bs58;
  use std::io::{self, BufRead};

  #[test]
  fn keygen() {
    let kp = Keypair::new();
    println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); 
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
  }

  #[test]
  fn base58_to_wallet() {
    println!("Input your private key as base58:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    let wallet = bs58::decode(base58).into_vec().unwrap(); 
    println!("Your wallet file is:");
    println!("{:?}", wallet);
  } 

  #[test]
  fn wallet_to_bas58() {
    println!("Input your private key as byte array:");
    let stdin = io::stdin();
    let wallet = stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').split(',').map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
    let base58 = bs58::encode(wallet).into_string();
    println!("Your private key is:");
    println!("{:?}", base58);
  }
  
  #[test]
  fn airdop() {}

  #[test]
  fn transfer_sol() {}
}