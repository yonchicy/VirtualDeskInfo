
use winvd::{get_desktop_count};
fn main(){
    println!("Number of desktops: {:?}", get_desktop_count().unwrap());
}