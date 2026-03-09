use warrior_util::utils::base64;

fn main() {
    println!("test {} ", base64::encode(b"Man"));
    println!("test {}", base64::encode(b"Ma"));
    println!("test {}", base64::encode(b"M"));
    println!("test {}", base64::encode(b"Hello World"));
    println!("test {}", base64::encode(b""));
}
