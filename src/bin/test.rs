fn c_e_g() {
    let chord = Chord::new("C E G");
    println!("{:?}", chord.pitched_common_name);
}

fn main() {
    c_e_g();
}
