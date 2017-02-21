
enum Flavor {
  Config,
  Keyframe,
  Frame
}

enum Content {
  Metadata,
  Video,
  Audio
}

pub struct Frame {
  dts: i64, // in 90 Khz baze
  pts: i64,
  duration: i32,
  flavor: Flavor,
  content: Content,
  body: Vec<u8>
}

