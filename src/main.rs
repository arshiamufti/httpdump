extern crate pcap;
use pcap::Capture;
use std::path::Path;

fn main() {
    let p = Path::new("test_pcap.pcapng");
    let mut cap = Capture::from_file(&p).unwrap();

    println!("beginning pcap file parse");

        while let Ok(packet) = cap.next() {
            println!("received packet! {:?}", packet);
        }
    println!("end of pcap file parse");

}
