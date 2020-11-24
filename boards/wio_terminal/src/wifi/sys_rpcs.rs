use super::erpc::{codec, id, Err};
use heapless::{consts::U16, String};
use nom::number::streaming;

pub struct GetVersion {}

impl codec::RPC for GetVersion {
    type ReturnValue = String<U16>;
    type Error = ();

    fn header(&self, seq: u32) -> codec::Header {
        codec::Header {
            sequence: seq,
            msg_type: id::MsgType::Invocation,
            service: id::Service::System,
            request: id::SystemRequest::VersionID.into(),
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<String<U16>, Err<()>> {
        let (data, _) = codec::Header::parse::<()>(data)?; // TODO: Check RPC header values

        let (data, length) = streaming::le_u32::<()>(data)?;
        if length > 16 {
            return Err(Err::ResponseOverrun);
        }

        let mut out: Self::ReturnValue = String::new();
        for b in data.iter() {
            out.push(*b as char).ok();
        }
        Ok(out)
    }
}
