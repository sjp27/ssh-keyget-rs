// Copyright 2025 sjp27 <https://github.com/sjp27>. All rights reserved.
// Use of this source code is governed by the MIT license that can be
// found in the LICENSE file.

// Utility to get the SSH public key from a server.

use makiko::mac;
use makiko::cipher;
use makiko::kex;
use makiko::pubkey;
use base64::prelude::*;
use clap::Parser;
#[derive(Parser)]
/// Utility to get the SSH public key from a server v1.0.0
struct Cli {
    /// The host and port to use e.g. 192.168.1.1:22
    host_port: String,
    /// The key type to get ed25519, rsa_sha2, ecdsa, rsa
    key_type: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    // Connect to the SSH server.
    let socket = tokio::net::TcpStream::connect(args.host_port).await
        .expect("Could not open a TCP socket");

    // Configure the SSH client.
    let config = get_client_config(&*args.key_type);

    // Create the SSH client.
    let (client, mut client_rx, client_fut) = makiko::Client::open(socket, config)
        .expect("Could not open client");

    // Spawn a Tokio task that polls the client.
    tokio::task::spawn(async move {
        client_fut.await.expect("Error in client future");
    });

    loop {
        // Wait for the next event.
        let event = client_rx.recv().await
            .expect("Error while receiving client event");

        // Exit the loop when the client has closed.
        let Some(event) = event else {
            break
        };

        match event {
            // Handle the server public key
            makiko::ClientEvent::ServerPubkey(pubkey, accept) => {
                print!("{}", BASE64_STANDARD.encode(pubkey.encode()));
                println!(" {} {}", pubkey.type_str(), pubkey.fingerprint());
                accept.accept();
            },

            // All other events can be ignored
            _ => {},
        }
        break
    }

    // We aren't going to use the client
    let _ = client;
}
pub fn get_client_config(key_type: &str) -> makiko::ClientConfig {
    makiko::ClientConfig::default().with(|c| {
        c.kex_algos.extend_from_slice(&[
            &kex::DIFFIE_HELLMAN_GROUP14_SHA256,
            &kex::DIFFIE_HELLMAN_GROUP16_SHA512,
            &kex::DIFFIE_HELLMAN_GROUP18_SHA512,
            &kex::DIFFIE_HELLMAN_GROUP14_SHA1,
        ]);

        if key_type == "ed25519" {
            c.server_pubkey_algos.splice(
                0..,
                [ &pubkey::SSH_ED25519 ]
            );
        }
        else if key_type == "rsa_sha2" {
            c.server_pubkey_algos.splice(
                0..,
                [ &pubkey::RSA_SHA2_256,  &pubkey::RSA_SHA2_512 ]
            );
        }
        else if key_type == "ecdsa" {
            c.server_pubkey_algos.splice(
                0..,
                [ &pubkey::ECDSA_SHA2_NISTP256,  &pubkey::ECDSA_SHA2_NISTP384 ]
            );
        }
        else if key_type == "rsa" {
            c.server_pubkey_algos.splice(
                0..,
                [ &pubkey::SSH_RSA_SHA1 ]
            );
        }
        else {
            c.server_pubkey_algos.clear();
        }

        c.cipher_algos.extend_from_slice(&[
            &cipher::AES128_CBC, &cipher::AES192_CBC, &cipher::AES256_CBC,
        ]);
        c.mac_algos.extend_from_slice(&[
            &mac::HMAC_SHA1_ETM, &mac::HMAC_SHA1,
        ]);
    })
}
