use frame::Frame;
use frame::Content;
use frame::Flavor;
use frame::Codec;
use std;

#[derive(Debug)]
pub enum ReadError {
  Eof,
  TooShortPrefix,
  TooShortTrailer,
  TooShortFrameHeader,
  TooShortFrameBody,
  TooShortFrameTrailer,
  InvalidVideoCodec,
  InvalidAudioCodec,
  Broken,
  InvalidType
}

impl std::convert::From<std::io::Error> for ReadError {
  fn from(_: std::io::Error) -> Self {
    ReadError::Eof
  }
}

pub fn read_frame<T: std::io::Read>(input: &mut T) -> Result<Frame, ReadError> {
  let mut header = [0;11];

  try!(input.read_exact(&mut header[..]));


  if header[0] == 'F' as u8 && header[1] == 'L' as u8 {
    println!("Skip FLV prefix");
    let mut skip = [0; 2];
    match input.read(&mut skip[..]) {
      Ok(2) => {}
      Ok(_) => {return Err(ReadError::TooShortTrailer)}
      Err(_) => { return Err(ReadError::Eof)}
    }

    match input.read(&mut header[..]) {
      Ok(11) => {}
      Ok(_) => {return Err(ReadError::TooShortFrameHeader)}
      Err(_) => { return Err(ReadError::Eof)}
    }
  }


// tag_header(<<Type, Size:24, TimeStamp:24, TimeStampExt, StreamId:24>>) when Type > 0 ->

  let content = match header[0] {
    8 => Content::Audio,
    9 => Content::Video,
    18 => Content::Metadata,
    _ => return Err(ReadError::InvalidType)
  };

  let disk_size = ((header[1] as u32) << 16 | (header[2] as u32) << 8 | (header[3] as u32)) as usize;
  let timestamp = (header[4] as u32) << 16 | (header[5] as u32) << 8 | (header[6] as u32) | (header[7] as u32) << 24;

  // We skip stream id, it is not interesting for us

  // let mut disk_frame = vec![0;disk_size];

  // match input.read(&mut disk_frame[..]) {
  //   Ok(read_bytes) => {
  //     if read_bytes != disk_size {
  //       return Err(ReadError::TooShortFrameBody)
  //     }
  //   }
  //   Err(err) => { return Err(ReadError::Eof)}
  // }

  let frame = match content {
    Content::Video => {
      let mut video_tag = [0;5];
      try!(input.read_exact(&mut video_tag[..]));

      if (video_tag[0] & 15) != 7 { 
        return Err(ReadError::InvalidVideoCodec)
      }
      let mut flavor = Flavor::Frame;
      if video_tag[1] == 0 {
        flavor = Flavor::Config
      } else {
        if (video_tag[0] >> 4) == 1 {
          flavor = Flavor::Keyframe
        }
      }
      let ctime = (video_tag[2] as u32) << 16 | (video_tag[3] as u32) << 8 | (video_tag[4] as u32);

      let mut body = vec![0;disk_size - 5];

      match input.read(&mut body[..]) {
        Ok(read_bytes) => {
          if read_bytes != disk_size-5 {
            return Err(ReadError::TooShortFrameBody)
          }
        }
        Err(_) => { return Err(ReadError::Eof)}
      }
      let codec = Codec::H264;

      Frame{dts: timestamp as i64, pts: (timestamp+ctime) as i64, duration: 0, flavor: flavor, content: content, body: body, codec: codec}
    }
    Content::Audio => {
      let mut audio_tag = [0;2];
      try!(input.read_exact(&mut audio_tag[..]));

      if (audio_tag[0] >> 4) != 10 { 
        return Err(ReadError::InvalidAudioCodec)
      }
      let mut flavor = Flavor::Frame;
      if audio_tag[1] == 0 {
        flavor = Flavor::Config;
      }
      let mut body = vec![0;disk_size - 2];
      try!(input.read_exact(&mut body[..]));
      let codec = Codec::Aac;
      Frame{dts: timestamp as i64, pts: timestamp as i64, duration: 0, flavor: flavor, content: content, body: body, codec: codec}      
    }
    Content::Metadata => {
      let mut body = vec![0;disk_size];
      try!(input.read_exact(&mut body[..]));
      let codec = Codec::Amf;
      Frame{dts: timestamp as i64, pts: timestamp as i64, duration: 0, flavor: Flavor::Frame, content: content, body: body, codec: codec}
    }
  };


  let mut prev_tag_size = [0;4];
  match input.read(&mut prev_tag_size[..]) {
    Ok(4) => {}
    Ok(_) => { return Err(ReadError::TooShortFrameTrailer)}
    Err(_) => { return Err(ReadError::Eof)}
  };


  Ok(frame)
}
