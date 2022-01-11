// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example ledger --features=ledger-nano --release

use iota_client::{
    signing::{ledger::LedgerNanoSigner, SignerHandle},
    Client, Result,
};

/// In this example we will create addresses with a ledger nano hardware wallet

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .with_node_sync_disabled()
        .finish()
        .await?;

    let ledger_signer = LedgerNanoSigner {
        is_simulator: false,
        ..Default::default()
    };
    let signer = SignerHandle::new(Box::new(ledger_signer));

    // Generate addresses with custom account index and range
    let addresses = iota
        .get_addresses(&signer)
        .with_account_index(0)
        .with_range(0..2)
        .finish()
        .await?;

    println!("List of generated public addresses:\n{:?}\n", addresses);

    Ok(())
}
