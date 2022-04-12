use crate::nal::Nal;
use crate::Context;

use crate::annexb::AnnexBReader;
use crate::push::{NalInterest};
use crate::nal::sps::{SeqParameterSet};
use crate::nal::{RefNal, UnitType};

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_pts() {
        pub fn parse_avc_sps(data: &[u8]) -> Option<SeqParameterSet> {
            let mut ctx = Context::default();
            let sps_handler = |nal: RefNal<'_>| {
                if let Ok(nal_hdr) = nal.header() {
                    match nal_hdr.nal_unit_type() {
                        UnitType::SeqParameterSet if nal.is_complete() => {
                            let sps =
                                SeqParameterSet::from_bits(nal.rbsp_bits()).unwrap();
                            ctx.put_seq_param_set(sps);
                        }
                        _ => {
                            println!("Ignore nal {:?}", nal_hdr)
                        }
                    }
                }
                NalInterest::Buffer
            };

            let mut reader = AnnexBReader::accumulate(sps_handler);
            reader.push(data);

            let sps = ctx.sps().next();
            sps.cloned()
        }

        let bytes =
            hex::decode("00000001e0000080c00a37030b0e6317030b005300000001091000000001674d0020e980a00b74a404180500000303e8000186a08da1425b800000000168ee06e2000000000106042fb500314741393403ccfffa0000fa0000fa0000fa0000fa0000fa0000fa0000fa0000fa0000fa0000fa0000fa0000ff8000000165b80000d778001c50a313b2d505b04bb7add6f55764d32c939a455e9ba75d744818cf6760749f31070f1245c8f0e16ba283717159").unwrap();

        let sps = parse_avc_sps(&bytes);
        assert!(sps.is_some());
        let sps = sps.unwrap();
        println!("SPS {:?}", sps);
    }
}