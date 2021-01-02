use super::*;
use crate::errors::*;
use crate::message::packer::*;

// An OPTResource is an OPT pseudo Resource record.
//
// The pseudo resource record is part of the extension mechanisms for DNS
// as defined in RFC 6891.
#[derive(Default)]
pub struct OPTResource {
    options: Vec<DNSOption>,
}

// An Option represents a DNS message option within OPTResource.
//
// The message option is part of the extension mechanisms for DNS as
// defined in RFC 6891.
#[derive(Default)]
pub struct DNSOption {
    code: u16, // option code
    data: Vec<u8>,
}

impl fmt::Display for DNSOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "dnsmessage.Option{{Code: {}, Data: {:?}}}",
            self.code, self.data
        )
    }
}

impl fmt::Display for OPTResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: Vec<String> = self.options.iter().map(|o| o.to_string()).collect();
        write!(f, "dnsmessage.OPTResource{{options: {}}}", s.join(","))
    }
}

impl ResourceBody for OPTResource {
    fn real_type(&self) -> DNSType {
        DNSType::OPT
    }

    fn pack(
        &self,
        mut msg: Vec<u8>,
        _compression: &mut Option<HashMap<String, usize>>,
        _compression_off: usize,
    ) -> Result<Vec<u8>, Error> {
        for opt in &self.options {
            msg = pack_uint16(msg, opt.code);
            msg = pack_uint16(msg, opt.data.len() as u16);
            msg = pack_bytes(msg, &opt.data);
        }
        Ok(msg)
    }

    fn unpack(&mut self, msg: &[u8], mut off: usize, length: usize) -> Result<usize, Error> {
        let mut opts = vec![];
        let old_off = off;
        while off < old_off + length {
            let (code, new_off) = unpack_uint16(msg, off)?;
            off = new_off;

            let (l, new_off) = unpack_uint16(msg, off)?;
            off = new_off;

            let mut opt = DNSOption {
                code,
                data: vec![0; l as usize],
            };
            if off + l as usize > msg.len() {
                return Err(ERR_CALC_LEN.to_owned());
            }
            opt.data.copy_from_slice(&msg[off..off + l as usize]);
            off += l as usize;
            opts.push(opt);
        }
        self.options = opts;
        Ok(off)
    }
}
