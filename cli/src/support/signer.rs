use anyhow::bail;
use clap::Parser;
use cosmrs::{bip32, crypto::secp256k1::SigningKey};

use crate::framework::config::Account;

#[derive(Debug, Parser, Clone)]
pub struct SignerArgs {
    /// Specifies predefined account as a tx signer
    #[clap(long, conflicts_with_all=&["signer-mnemonic", "signer-private-key"])]
    pub signer_account: Option<String>,

    /// Specifies mnemonic as a tx signer
    #[clap(long, conflicts_with_all=&["signer-account", "signer-private-key"])]
    pub signer_mnemonic: Option<String>,

    /// Specifies private_key as a tx signer (base64 encoded string)
    #[clap(long, conflicts_with_all=&["signer-account", "signer-mnemonic"])]
    pub signer_private_key: Option<String>,
}

impl SignerArgs {
    pub fn private_key(
        &self,
        global_config: &crate::framework::config::GlobalConfig,
    ) -> Result<SigningKey, anyhow::Error> {
        let Self {
            signer_account,
            signer_mnemonic,
            signer_private_key,
        } = self;
        let derivation_path = global_config.derivation_path();
        let signer_priv = if let Some(signer_account) = signer_account {
            match global_config.accounts().get(signer_account) {
                None => bail!("signer account: `{signer_account}` is not defined"),
                Some(Account::FromMnemonic { mnemonic }) => {
                    SigningKey::from_mnemonic(mnemonic.as_str(), derivation_path)
                }
                Some(Account::FromPrivateKey { private_key }) => {
                    Ok(SigningKey::from_bytes(&base64::decode(private_key)?).unwrap())
                }
            }
        } else if let Some(signer_mnemonic) = signer_mnemonic {
            SigningKey::from_mnemonic(signer_mnemonic, derivation_path)
        } else if let Some(signer_private_key) = signer_private_key {
            Ok(SigningKey::from_bytes(&base64::decode(signer_private_key)?).unwrap())
        } else {
            bail!("Unable to retrive signer private key")
        }?;
        Ok(signer_priv)
    }
}

pub trait SigningKeyExt {
    fn from_mnemonic(phrase: &str, derivation_path: &str) -> Result<SigningKey, anyhow::Error> {
        let seed = bip32::Mnemonic::new(phrase, bip32::Language::English)?.to_seed("");
        let xprv = bip32::XPrv::derive_from_path(seed, &derivation_path.parse()?)?;
        let signer_priv: SigningKey = xprv.into();
        Ok(signer_priv)
    }
}

impl SigningKeyExt for SigningKey {}
