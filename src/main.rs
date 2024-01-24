use epub::doc::EpubDoc;

fn main() {
    let doc = EpubDoc::new("./epubs/wallace-sandi-the-kingmaker.epub");
    assert!(doc.is_ok());

    let doc = doc.unwrap();
    let meta = &doc.metadata;

    for (key, value) in meta {
        println!("{:?}: \"{:?}\"", key, value);
    }
}
