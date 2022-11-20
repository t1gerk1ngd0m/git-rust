// pub enum GitObject {
//     Blob(Blob),
//     Tree(Tree),
//     Commit(Commit)
// }

use std::io::Read;
use std::io;
use libflate::zlib::Decoder;

mod libs;
#[mockall_double::double]
use libs::file_driver::file_driver;

// pub enum GitObject {
//     Blob(Blob),
// }

// impl GitObject {
//     pub fn new(bytes: &[u8]) -> Option<Slef> {

//     }
// }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blob {
    pub size: usize,
    pub content: String
}

impl Blob {
    pub fn new(content: String) -> Self {
        Self {
            size: content.len(),
            content
        }
    }
    pub fn from(bytes: &[u8]) -> Option<Self> {
        let content = String::from_utf8(bytes.to_vec());

        match content {
            Ok(c) => Some(Self {
                size: c.len(),
                content: c
            }),
            _ => None
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let cmd = args.get(1).unwrap().clone();

    match cmd.as_str() {
        "cat_file_p" => {
            println!("{:?}", cat_file_p(args.get(2).unwrap().clone()));
        }
        _ => {}
    }
}

pub fn cat_file_p(hash: String) -> Result<Option<Blob>, io::Error> {
    let (sub_dir, file) = hash.split_at(2);
    let path = format!("{}/.git/objects/{}/{}", env!("CARGO_MANIFEST_DIR"), sub_dir, file);

    let file_content = file_driver::read_file(path).unwrap();

    let mut d = Decoder::new(&file_content[..]).unwrap();
    let mut buf = Vec::new();
    d.read_to_end(&mut buf);

    Ok(Blob::from(&buf[..]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use libflate::zlib::Encoder;

    #[test]
    fn test_cat_file_p() {
        let hash = "a99490da91ada873f415a19ce20ccfa43b2ebf54".to_string();
        let path = format!("{}/.git/objects/a9/9490da91ada873f415a19ce20ccfa43b2ebf54", env!("CARGO_MANIFEST_DIR"));
        let mut encoder = Encoder::new(Vec::new()).unwrap();
        io::copy(&mut &b"Hello world"[..], &mut encoder).unwrap();
        let encoded_file_content = encoder.finish().into_result().unwrap();

        let driver = file_driver::read_file_context();
        driver
            .expect()
            .with(eq(path))
            .times(1)
            .returning(move |_| Ok(encoded_file_content.clone()));

        let actual = Blob::new("Hello world".to_string());
        let expected = cat_file_p(hash).unwrap().unwrap();

        println!("{:?}", expected);

        assert_eq!(actual, expected);
    }
}
