use anyhow::Result;

mod wallet_lib;

const URL: &str = "a";
fn main() -> Result<()> {
    let keypair = wallet_lib::create_keypair();
    
    println!("{:?}", keypair);

    let web3 = wallet_lib::setup_web3_connection(url)?;
    
    Ok(())
}
