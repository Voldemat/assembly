mod a64;

fn main() {
    println!(
        "{:?}",
        a64::load_store::STTRB::decode(
            a64::load_store::STTRB {
                source_register: a64::shared::RegisterNumber::try_from(10).unwrap(),
                destination_address_register: a64::shared::RegisterNumber::try_from(31).unwrap(),
                immediate_byte_offset: a64::shared::SignedOffset {
                    sign: true,
                    offset: 0
                },
            }
            .encode()
        ).unwrap()
    );
}
