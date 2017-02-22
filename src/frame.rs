use std::fmt;

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
pub enum Codec {
  H264,
  Aac,
  Hevc,
  Pcma,
  Amf
}

pub struct Frame {
  pub dts: i64,
  pub pts: i64,
  pub duration: i32,
  pub flavor: Flavor,
  pub content: Content,
  pub codec: Codec,
  pub body: Vec<u8>
}

impl fmt::Debug for Frame {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Frame {{ {:?} {:?} dts: {}, ctime: {}, body: {} }}", self.content, self.flavor, self.dts, self.pts - self.dts, self.body.len())
  }
}