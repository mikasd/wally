use anyhow::Result;
use fltk::{
    app,
    button::Button,
    enums::{Color, Font, FrameType},
    frame::Frame,
    input,
    prelude::{GroupExt, InputExt, WidgetBase, WidgetExt},
    window::Window,
};
use secp256k1::{PublicKey, SecretKey};
use std::str::FromStr;
use tokio;
use web3::types::Address;

mod wallet_lib;
use crate::wallet_lib::{create_keypair, create_txn_object, sign_and_send};

// ropsten ETH for practice
// https://faucet.dimensions.network/ for adding Ropsten-ETH to wallet
const URL: &str = "https://eth-ropsten.alchemyapi.io/v2/KCNPR0famYesyHVy345ynShLVIr9Eaf4";

#[derive(Debug, Clone)]
pub enum WalletMessage {
    NewWallet,
    Send,
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = app::App::default();

    let mut window = Window::default()
        .with_size(500, 800)
        .with_label("basic wallet");
    let mut button_1 = Button::new(195, 450, 120, 45, "Create Wallet");
    let mut button_2 = Button::new(200, 300, 100, 40, "send");
    let mut input_1 = input::Input::new(200, 200, 225, 35, "to: ");
    let mut frame = Frame::default()
        .with_size(600, 100)
        .center_of(&window)
        .with_label("0 wallets");

    frame.set_label_color(Color::White);
    frame.set_label_font(Font::TimesBold);
    frame.set_label_size(24);

    window.set_color(Color::DarkCyan);

    button_1.set_color(Color::White);
    button_1.set_label_color(Color::DarkMagenta);
    button_1.set_label_font(Font::TimesBold);
    button_1.set_frame(FrameType::FlatBox);
    button_1.clear_visible_focus();

    button_2.set_color(Color::White);
    button_2.set_label_color(Color::DarkMagenta);
    button_2.set_label_font(Font::TimesBold);
    button_2.set_frame(FrameType::FlatBox);
    button_2.clear_visible_focus();

    input_1.set_frame(FrameType::FlatBox);

    window.end();
    window.show();

    let (s, r) = app::channel::<WalletMessage>();

    button_1.emit(s.clone(), WalletMessage::NewWallet);
    button_2.emit(s, WalletMessage::Send);

    let web3 = wallet_lib::setup_web3_connection(URL)?;
    let mut keypairs: Vec<(PublicKey, SecretKey)> = Vec::new();

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                WalletMessage::NewWallet => {
                    let (seckey, pubkey) = match create_keypair() {
                        Ok(val) => val,
                        Err(_e) => unimplemented!(),
                    };
                    keypairs.push((pubkey, seckey));
                    frame.set_label(&format!("{} Wallets", keypairs.len()));
                },
                WalletMessage::Send => {
                    let to_address = Address::from_str(&input_1.value().as_str())?;
                    let tx_object = create_txn_object(to_address, 69)?;
                    let result = sign_and_send(web3.clone(), tx_object, keypairs[0].1).await;
                    match result {
                        Ok(val) => frame.set_label(&format!("{}", val)),
                        Err(e) => frame.set_label(&format!("{}", e)),
                    };
                }
            }
        }
    }

    app.run().unwrap();

    Ok(())
}
