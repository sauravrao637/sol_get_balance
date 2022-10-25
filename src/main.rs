use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

fn get_balance(url: &str, public_key: &Pubkey) -> u64{
    let rpc_client = RpcClient::new(url);
    rpc_client.get_account(public_key).unwrap().lamports
}
fn main() {
    let url = "https://api.devnet.solana.com";
    let pubkey_str = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";
    let public_key = Pubkey::from_str(pubkey_str).unwrap();
    let balance = get_balance(url, &public_key);
    println!("{pubkey_str} has {balance} lamports");
}
