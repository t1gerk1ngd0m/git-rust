use sha1::{Sha1, Digest};

pub struct Blob {
  pub size: usize,
  pub content: String,
}

impl Blob {
  // 構造体を作成する関数
  // hash値を計算する関数
  // ファイルに書き込むフォーマットにするための関数

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

  pub fn calc_hash(&self) -> Vec<u8> {
      Vec::from(Sha1::digest(&self.as_bytes()).as_slice())
  }

  pub fn as_bytes(&self) -> Vec<u8> {
      let header = format!("blob {}\0", self.size);
      let store = format!("{}{}", header, self.content.to_string());

      Vec::from(store.as_bytes())
  }
}
