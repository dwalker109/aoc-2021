use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> usize {
    let mut bin_remaining = &hex_to_bin(input)[0..];

    let mut packets = Vec::new();

    while !bin_remaining.is_empty() {
        let (p, s) = extract_packet_stream(bin_remaining, false);
        packets.push(p);
        bin_remaining = s;
    }

    fn flatten_packet_versions(packets: Vec<Packet>) -> Vec<u8> {
        let mut flat_versions = Vec::new();

        for p in packets {
            flat_versions.push(p.version);

            if let PacketData::Operator(s) = p.payload {
                let mut sub = flatten_packet_versions(s);
                flat_versions.append(&mut sub);
            }
        }

        flat_versions
    }

    flatten_packet_versions(packets).iter().map(|v| *v as usize).sum()
}

fn hex_to_bin(raw: &str) -> String {
    let bin = raw
        .as_bytes()
        .iter()
        .filter_map(|h| {
            u8::from_str_radix(unsafe { std::str::from_utf8_unchecked(&[*h]) }, 16).ok()
        })
        .map(|b| format!("{:04b}", b))
        .collect::<Vec<_>>()
        .join("");

    bin
}

fn extract_packet_stream(bin: &str, is_sub_packet: bool) -> (Packet, &str) {
    let version = u8::from_str_radix(&bin[0..3], 2).unwrap();
    let type_id = u8::from_str_radix(&bin[3..6], 2).unwrap();

    let (data, size) = match type_id {
        4 => {
            let lit_str = &bin[6..];

            let lit_dec = lit_str
                .as_bytes()
                .chunks(5)
                .fold_while(Vec::new(), |mut acc, cur| {
                    let bits = <[u8; 5]>::try_from(cur).unwrap();

                    let has_more = bits[0];
                    let data = &bits[1..];

                    acc.push(data.to_vec());

                    if has_more == b'1' {
                        Continue(acc)
                    } else {
                        Done(acc)
                    }
                })
                .into_inner();

            let packet_len = (&lit_dec.len() * 5) + 6;

            let lit_bin = unsafe {
                std::str::from_utf8_unchecked(&lit_dec.into_iter().flatten().collect_vec())
                    .to_string()
            };

            (
                PacketData::Literal(usize::from_str_radix(&lit_bin, 2).unwrap()),
                packet_len,
            )
        }
        _ => {
            let length_type_id = u8::from_str_radix(&bin[6..7], 2).unwrap();

            match length_type_id {
                0 => {
                    let mut bit_length_subpackets =
                        usize::from_str_radix(&bin[7..7 + 15], 2).unwrap();
                    let mut bin_remaining = &bin[7 + 15..];
                    let mut subpackets = Vec::new();

                    while bit_length_subpackets > 0 {
                        let (p, s) = extract_packet_stream(bin_remaining, true);
                        bin_remaining = s;
                        bit_length_subpackets -= p.size;
                        subpackets.push(p);
                    }

                    let packet_len = &bin.len() - bin_remaining.len();

                    (PacketData::Operator(subpackets), packet_len)
                }
                1 => {
                    let mut qty_subpackets = usize::from_str_radix(&bin[7..7 + 11], 2).unwrap();
                    let mut bin_remaining = &bin[7 + 11..];
                    let mut subpackets = Vec::new();

                    while qty_subpackets > 0 {
                        let (p, s) = extract_packet_stream(bin_remaining, true);
                        bin_remaining = s;
                        qty_subpackets -= 1;
                        subpackets.push(p);
                    }

                    let packet_len = &bin.len() - bin_remaining.len();

                    (PacketData::Operator(subpackets), packet_len)
                }
                _ => unreachable!(),
            }
        }
    };

    let packet = Packet {
        version,
        type_id,
        payload: data,
        size,
    };

    let next_str_slice = match is_sub_packet {
        false => &bin[calc_raw_packet_len(size)..],
        true => &bin[size..],
    };

    (packet, next_str_slice)
}

/// Ceil the len to a byte boundary (8, 16, 24, 32, 40, 48, 56 etc)
fn calc_raw_packet_len(len: usize) -> usize {
    let mut x = len - 1;
    x = x >> 3;
    x = x + 1;
    x << 3
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    payload: PacketData,
    size: usize,
}

#[derive(Debug)]
enum PacketData {
    Literal(usize),
    Operator(Vec<Packet>),
}

#[cfg(test)]
mod tests {
    use crate::PacketData;

    static INPUT: &str = "../input";

    #[test]
    fn hex_to_bin() {
        let a = super::hex_to_bin("D2FE28");
        assert_eq!(a, "110100101111111000101000");
    }

    #[test]
    fn extract_packet_stream_1() {
        let (p, rem) = super::extract_packet_stream("110100101111111000101000", false);

        assert!(matches!(p.payload, PacketData::Literal(2021)));
        assert_eq!(rem.len(), 0);
    }

    #[test]
    fn extract_packet_stream_2() {
        let (p, rem) = super::extract_packet_stream(
            "00111000000000000110111101000101001010010001001000000000",
            false,
        );

        assert!(matches!(p.payload, PacketData::Operator(_)));
        assert_eq!(rem.len(), 0);
    }

    #[test]
    fn extract_packet_stream_3() {
        let (p, rem) = super::extract_packet_stream(
            "11101110000000001101010000001100100000100011000001100000",
            false,
        );

        assert!(matches!(p.payload, PacketData::Operator(_)));
        assert_eq!(rem.len(), 0);
    }
}
