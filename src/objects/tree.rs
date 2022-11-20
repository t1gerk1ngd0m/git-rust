pub struct Tree {
  pub contents: Vec<File>
}

pub struct File {
  pub mode: usize,
  pub name: String,
  pub hash: Vec<u8>
}

impl Tree {
  pub fn from(bytes: &[u8]) -> Option<Self> {
    let contents: Vec<File> = Vec::new();
    let mut iter = bytes.split(|&b| b == b'\0'); // 各エントリーは'\0'で区切られている

    let mut header = iter.next();
    let contents = iter.try_fold(contents, |mut acc, x| {
      let (hash, next_header) = x.split_at(20); // hash値は20bytes
      let file = File::from(header, hash)?;
      acc.push(file);
      header = next_header;
      Some(acc)
    });

    Some(Self { contents })
  }

  pub fn as_bytes(&self) -> Vec<u8> {
    let content: Vec<u8> = self.contents.iter().flat_map(|x| x.encode()).collect();
    let header = format!("tree {}\0", content.len());
    [header.as_bytes(), content.as_slice()].concat()
  }
}

impl File {
  pub fn new(mode: usize, name: String, hash: &[u8]) -> Self {
      Self {
          mode,
          name,
          hash: Vec::from(hash)
      }
  }

  pub fn from(header: &[u8], hash: &[u8]) -> Option<Self> {
      let split_header = String::from_utf8(header.to_vec()).ok()?;
      let mut iter = split_header.split_whitespace();

      // 空白で区切られたiterの最初の要素(mode)を取り出してなんやかんややってる
      let mode = iter.next().and_then(|x| x.parse::<usize>().ok())?;
      let name = iter.next()?;

      Some(Self::new(mode, String::from(name), hash))
  }

  pub fn encode(&self) -> Vec<u8> {
    let header = format!("{} {}\0", self.mode, self.name);
    [header.as_bytes(), &self.hash].concat()
  }
}
