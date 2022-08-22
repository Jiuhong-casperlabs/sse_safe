#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use casper_types::{
        bytesrepr::{Bytes, FromBytes},
        AccessRights, CLValue, URef, URefAddr,
    };

    #[test]
    fn deserialize_data() {
        let bytes_original = "4e00000001000000030000000a000000746f6b656e5f6e616d65050000006170706c650c000000746f6b656e5f737570706c790200000031300c000000746f6b656e5f73796d626f6c050000004150504c450e110a0a200000002403d95c263d092c1b4af04568aba9a1a25fe846987f13e4aff7339d4ff658cc0100000031";

        let bytes = base16::decode(bytes_original).unwrap();

        let (cl_value, bytes): (CLValue, _) = FromBytes::from_bytes(&bytes).unwrap();
        println!("cl_value is {:?}", cl_value);
        let (uref_addr, bytes): (Bytes, _) = FromBytes::from_bytes(bytes).unwrap();

        let uref_addr: URefAddr = uref_addr.inner_bytes().clone().try_into().unwrap();
        let uref = URef::new(uref_addr, AccessRights::READ_ADD_WRITE);
        println!("Events dictionary seed: {:?}", uref.to_formatted_string());
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
