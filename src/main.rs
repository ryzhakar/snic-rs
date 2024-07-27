mod gber;

fn main() {
    let r = gber::GBERepresentation::new(u64::MAX, 2);
    dbg!(r, r.to_decimal(), r.calculate_components());
}
