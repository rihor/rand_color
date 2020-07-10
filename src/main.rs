use rand::random;

fn main() {
    let red: u8 = random();
    let green: u8 = random();
    let blue: u8 = random();
    println!("Random color");
    println!("RGB -> rgb({}, {}, {})", red, green, blue);
    
    let red = format!("{:02x}", red);
    let green = format!("{:02x}", green);
    let blue = format!("{:02x}", blue);
    println!("HEX -> #{}{}{}", red, green, blue);

    println!("https://colorpicker.me/#{}{}{}", red, green, blue);
}
