use std::str::FromStr;

use apx_core::caip2::MoneroNetwork;
use monero::{
    network::Network,
    util::{
        address::{
            Error as AddressError,
            PaymentId,
        },
        key::{Error as KeyError},
    },
};

pub use monero::util::{
    address::{Address, AddressType},
    key::PrivateKey,
};

pub const LOCK_DURATION: u64 = 10; // blocks
pub const BLOCK_TIME: u16 = 120;

pub fn address_network(address: Address) -> MoneroNetwork {
    match address.network {
        Network::Mainnet => MoneroNetwork::Mainnet,
        Network::Stagenet => MoneroNetwork::Stagenet,
        Network::Testnet => MoneroNetwork::Testnet,
    }
}

pub fn parse_monero_address(address: &str) -> Result<Address, AddressError> {
    Address::from_str(address)
}

pub fn parse_monero_view_key(view_key: &str) -> Result<PrivateKey, KeyError> {
    PrivateKey::from_str(view_key)
}

pub fn create_integrated_address(
    primary_address: Address,
) -> Address {
    let payment_id = PaymentId::random();
    Address::integrated(
        primary_address.network,
        primary_address.public_spend,
        primary_address.public_view,
        payment_id,
    )
}

pub fn get_payment_id(address: Address) -> Option<PaymentId> {
    if let AddressType::Integrated(payment_id) = address.addr_type {
        Some(payment_id)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monero_address() {
        let address_str = "888tNkZrPN6JsEgekjMnABU4TBzc2Dt29EPAvkRxbANsAnjyPbb3iQ1YBRk1UXcdRsiKc9dhwMVgN5S9cQUiyoogDavup3H";
        let address = parse_monero_address(address_str).unwrap();
        assert_eq!(address.to_string(), address_str);
        assert_eq!(address.network, Network::Mainnet);
        assert_eq!(address.addr_type, AddressType::SubAddress);
    }

    #[test]
    fn test_parse_monero_view_key() {
        let key_str = "77916d0cd56ed1920aef6ca56d8a41bac915b68e4c46a589e0956e27a7b77404";
        let key = parse_monero_view_key(key_str).unwrap();
        assert_eq!(key.to_string(), key_str);
    }
}
