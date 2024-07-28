mod gber;

fn main() {
    let r = gber::GBERepresentation::new(u32::MAX, 2);
    dbg!(r.to_decimal(), r.calculate_components());
}
