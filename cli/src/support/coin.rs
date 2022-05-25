use std::str::FromStr;

use anyhow::Context;
use cosmrs::Coin;
use regex::Regex;

#[derive(Debug)]
pub struct CoinFromStr {
    inner: Coin,
}

impl CoinFromStr {
    pub fn inner(&self) -> &Coin {
        &self.inner
    }
}

impl FromStr for CoinFromStr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\d+)(.+)$").unwrap();
        let caps = re
            .captures(s)
            .with_context(|| format!("Unable to parse `{s}` as Coin."))?;

        let c = Coin {
            amount: caps
                .get(1)
                .with_context(|| format!("`{s}` does not contain valid amount"))?
                .as_str()
                .parse()
                .unwrap(),
            denom: caps
                .get(2)
                .with_context(|| format!("`{s}` does not contain valid denom"))?
                .as_str()
                .parse()
                .unwrap(),
        };

        Ok(CoinFromStr { inner: c })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    #[test]
    fn test_coin_from_str_with_correct_denom() {
        let c: CoinFromStr = "1000uosmo".parse().unwrap();
        assert_eq!(
            c.inner,
            Coin {
                amount: 1000u64.into(),
                denom: "uosmo".parse().unwrap()
            }
        )
    }

    #[test]
    fn test_coin_from_str_with_incorrect_denom() {
        let e = "uosmo1000".parse::<CoinFromStr>().unwrap_err();

        assert_eq!(
            e.to_string(),
            anyhow!("Unable to parse `uosmo1000` as Coin.").to_string()
        );
    }
}
