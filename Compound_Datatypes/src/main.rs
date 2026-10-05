fn main() {
    //number array
    let number: [i32; 2] = [0,1];
    println!("Array {:?}", number);

    //String array
    let strings: [&str; 3] = ["Muhammad", "Taha", "Rajput"];
    println!("Strings {:?}",strings);

    //tuple
    let tuple: (&str, i32, bool) = ("Taha",20, true);
    println!("Tupple: {:?}", tuple);

    let mix = ("Taha", 20 , true, [45,22,45,11,45]);
    println!("MIX {:?}", mix);

    //slice
    let slice1: &[i32] = &[1,5,4,7,96,1,2,3];
    println!("Slice1: {:?}", slice1);
    let slice2: &[i32] = &slice1[0..5];
    println!("Slice2 {:?}", slice2);

    //string slice
    let mut mike: String = String::from("Hello");
    println!{"Mike: {:?}", mike};
    mike.push_str("! Idoits");
    println!("Mike Update: {:?}",mike);

    //string slice
    let mike_slice: &str = &mike[0..6];
    println!("Slice2 {:?}", mike_slice);
}
