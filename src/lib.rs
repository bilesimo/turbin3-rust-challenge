#[cfg(test)] 
mod tests {
  use solana_sdk::{signature::{Keypair, Signer, read_keypair_file}, transaction::Transaction, message::Message};
  use bs58;
  use std::{io::{self, BufRead}, thread::sleep, time::Duration};
  use solana_client::rpc_client::RpcClient;
  use solana_program::system_instruction::transfer;

  const RPC_URL: &str = "https://api.devnet.solana.com";

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
  fn public_key() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    println!("Your public key is: {}", keypair.pubkey().to_string());
  }
  
  #[test]
  fn airdop() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let client = RpcClient::new(RPC_URL);

    let mut retries = 5; // Number of retry attempts
    let mut delay = 2; // Initial delay in seconds

    loop {
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(s) => {
                println!("Success! Check out your TX here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    s.to_string()
                );
                break;
            }
            Err(e) => {
                if retries > 0 {
                    println!("Airdrop failed due to rate limit. Retrying in {} seconds...", delay);
                    sleep(Duration::from_secs(delay));
                    delay *= 2; // Exponential backoff
                    retries -= 1;
                } else {
                    println!("Failed after multiple retries. Error: {}", e.to_string());
                    break;
                }
            }
        };
    }
  }

  #[test]
  fn balance() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let client = RpcClient::new(RPC_URL);
    let balance = client.get_balance(&keypair.pubkey()).unwrap();
    println!("Your balance is: {} SOL", balance as f64 / 1_000_000_000.0);
  }

  #[test]
  fn transfer_sol() {
    let from_keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find dev-wallet file");
    let to_keypair = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find Turbin3-wallet file");
    let to_pubkey = &to_keypair.pubkey();
    
    let rpc_client = RpcClient::new(RPC_URL);
    let recent_blockhash = rpc_client .get_latest_blockhash().expect("Failed to get recent blockhash");

    let balance = rpc_client.get_balance(&from_keypair.pubkey()).expect("Failed to get balance");

    let message = Message::new_with_blockhash(
      &[transfer(
        &from_keypair.pubkey(), 
        &to_pubkey, 
        balance
      )],  
      Some(&from_keypair.pubkey()), 
      &recent_blockhash
    );

    let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee calculator");

    let transaction = Transaction::new_signed_with_payer( 
      &[transfer(
        &from_keypair.pubkey(), 
        &to_pubkey, 
        balance - fee
      )], 
      Some(&from_keypair.pubkey()), 
      &vec![&from_keypair], 
      recent_blockhash
    );

    match rpc_client.send_and_confirm_transaction(&transaction) {
      Ok(signature) => {
          println!(
              "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
              signature
          );
      }
      Err(e) => {
          println!("Transaction failed: {}", e);
      }
  }

  }

}