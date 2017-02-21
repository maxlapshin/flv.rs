
#[derive(Debug)]
pub enum Flavor {
  Config,
  Keyframe,
  Frame
}

#[derive(Debug)]
pub enum Content {
  Metadata,
  Video,
  Audio
}

#[derive(Debug)]
pub struct Frame {
  pub dts: i64, // in 90 Khz baze
  pub pts: i64,
  pub duration: i32,
  pub flavor: Flavor,
  pub content: Content,
  pub body: Vec<u8>
}

