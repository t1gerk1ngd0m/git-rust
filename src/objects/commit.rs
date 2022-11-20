pub struct User {
  pub name: String,
  pub email: String,
  pub timestamp: DateTime<FixedOffset>
}

pub struct Commit {
  pub tree: String,
  pub parent: Option<String>,
  pub author: User,
  pub commiter: User,
  pub message: String
}

impl User {
  pub fn from(bytes: &[u8]) -> Option<Self> {
    let name = String::from_utf8(
      bytes
        .into_iter()
        .take_while(|&&x| x != b'<')
        .map(|&x| x)
        .collect()
    )
    .map(|x| String::from(x.trim()))
    .ok()?;

    let info = String::from_utf8(
      bytes
        .into_iter()
        .skip_while(|&&x| x != b'<')
        .map(|&x| x)
        .collect()
    )
    .ok()?;

    let mut info_iter = info.split(3, " ");

    let email = info_iter
                  .next()
                  .map(|x| String::from(x.trim_matches(|x| x == '<' || x == '>')))?;
    let ts = Utc.timestamp(info_iter.next().and_then(|x| x.parse::<i64>().ok())?, 0);
    let offset = info_iter
                    .next()
                    .and_then(|x| x.parse::<i32>().ok())
                    .map(|x| {
                      if x < 0 {
                        FixedOffset::west(x / 100 * 60 * 60)
                      } else {
                        FixedOffset::east(x / 100 * 60 * 60)
                      }
                    })?;
    Some(Self::new(
      name,
      email,
      offset.from_utc_datetime(&ts.naive_utc())
    ))
  }
}
