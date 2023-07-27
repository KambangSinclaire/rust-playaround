use ethers::types::Address;
use std::str::FromStr;

trait EtherAddress {
    fn convert_add(&self) -> Result<Address, &'static str>;
}

impl EtherAddress for &str {
    fn convert_add(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Etherium address string"),
        }
    }
}

impl EtherAddress for Address {
    fn convert_add(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}
fn get_ether_data<T: EtherAddress>(address: T) -> Address {
    let converted_address: Address = address.convert_add().unwrap();
    converted_address
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_poly() {
        let address: Address = Address::from_str("testing_hello").unwrap();
        let new_address: Address = get_ether_data(address);
        assert_eq!(new_address, Address::from_str("testing_hello").unwrap());
    }
}
