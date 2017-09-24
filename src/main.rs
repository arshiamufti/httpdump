extern crate pcap;
use pcap::Capture;
use std::path::Path;
extern crate peel_ip;
use peel_ip::prelude::*;

fn main() {

    let mut peel = PeelIp::default();
    peel.set_log_level(LogLevel::Trace);
    let p = Path::new("httpbin_pcap.pcapng");
    let mut cap = Capture::from_file(&p).unwrap();
    // let limit = 5;
    let mut count = 0;
    println!("beginning pcap file parse");
    while let Ok(packet) = cap.next() {
        println!("traversing a packet....");
        println!("{:?}", packet.data);
        peel.traverse(&packet.data, vec![]).result;
        // count = count + 1;
        // if count > limit {
        //     break;
        // }
    }
    println!("end of pcap file parse");

}
