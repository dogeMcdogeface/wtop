use gethostname::gethostname;

fn main() {
    println!("Hello, world!");
    println!("Hostname: {:?}", gethostname());
}
