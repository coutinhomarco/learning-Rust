const STARTING_MISSILEWS: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    let (mut missiles, ready) = (STARTING_MISSILEWS, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles)
}
