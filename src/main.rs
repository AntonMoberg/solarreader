#![no_std]
#![no_main]
use panic_halt as _;
use rp_pico::entry;

#[entry]
fn main() -> ! {
    let doc = EpubDoc::new("./epubs/wallace-sandi-the-kingmaker.epub");

    if doc.is_ok() {
        let doc = doc.unwrap();
        let meta = &doc.metadata;

        for (key, value) in meta {
            println!("{:?}: {:?}", key, value);
        }
    }
}
