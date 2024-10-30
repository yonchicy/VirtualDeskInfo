use winvd::{get_desktop_count, listen_desktop_events, switch_desktop, DesktopEvent};

fn get_current_desktop_id()->Option<u32>{
    get_desktop_id(winvd::get_current_desktop().unwrap())
}
fn get_desktop_id(desktop: winvd::Desktop)->Option<u32>{
    for (idx, d) in winvd::get_desktops().unwrap().iter().enumerate(){
        if *d==desktop{
            return Some(idx as u32);
        }
    }
    None
}
fn main() {
    // Desktop count
    println!("Desktops: {:?}", get_desktop_count().unwrap());
    // get current desktop
    println!("Current Desktop: {:?}", winvd::get_current_desktop().unwrap());
    // get current desktop index

    // Go to second desktop, index = 1
    switch_desktop(1).unwrap();

    // To listen for changes, use crossbeam, mpsc or winit proxy as a sender
    let (tx, rx) = std::sync::mpsc::channel::<DesktopEvent>();
    let _notifications_thread = listen_desktop_events(tx);

    // Keep the _notifications_thread alive for as long as you wish to listen changes
    std::thread::spawn(|| {
        for item in rx {
            println!("get info");
            println!("{:?}", item);
            // println current desktop id
            println!("Current Desktop: {:?}", get_current_desktop_id().unwrap());
            match item {
                DesktopEvent::DesktopChanged { new, old } =>{
                    println!("Desktop changed from {:?} to {:?}", old, new);
                    // print new desktop name
                    println!("Desktop changed from {:?} to {:?}", new.get_name().unwrap(), old.get_name().unwrap());
                    // println old desktop id to new desktop id
                    println!("Desktop changed from {:?} to {:?}", get_desktop_id(old).unwrap(), get_desktop_id(new).unwrap());
                }
                _=>{}
            }
        }
    });

    // Wait for keypress
    
    switch_desktop(0).unwrap();
    println!("⛔ Press enter to stop");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
