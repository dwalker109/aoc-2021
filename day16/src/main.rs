use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};

static INPUT: &str = include_str!("../input");

fn main() {
    env_logger::init();

    println!("Part 1: {}", part_1(INPUT));
    println!("Part 2: {}", part_2(INPUT));
}

#[logging_timer::time]
fn part_1(input: &str) -> usize {
    let packet = parse_input(input);

    packet.version_sum()
}

#[logging_timer::time]
fn part_2(input: &str) -> usize {
    let packet = parse_input(input);

    packet.value()
}

fn parse_input(input: &str) -> Packet {
    let (packet, _) = extract_packet_stream(&hex_to_bin(input)[0..]);

    packet
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

fn extract_packet_stream(bin: &str) -> (Packet, &str) {
    let version = u8::from_str_radix(&bin[0..3], 2).unwrap();
    let packet_type = PacketType::from(u8::from_str_radix(&bin[3..6], 2).unwrap());

    let (payload, size) = match packet_type {
        PacketType::Literal => extract_literal(bin),
        _ => extract_operator(bin),
    };

    let packet = Packet {
        version,
        packet_type,
        payload,
        size,
    };

    (packet, &bin[size..])
}

/// VVVTTTAAAAABBBBBCCCCC
fn extract_literal(raw: &str) -> (PacketData, usize) {
    let literal = &raw[6..];
    let literal = literal
        .as_bytes()
        .chunks(5)
        .fold_while(Vec::new(), |mut acc, cur| {
            let bits = <[u8; 5]>::try_from(cur).unwrap();

            let has_more = bits[0] == b'1';
            let data = &bits[1..];

            acc.append(&mut data.to_vec());

            if has_more {
                Continue(acc)
            } else {
                Done(acc)
            }
        })
        .into_inner();

    let literal = unsafe { std::str::from_utf8_unchecked(&literal).to_string() };
    let packet_len = (literal.len() + (literal.len() / 4)) + 6;

    (
        PacketData::Literal(usize::from_str_radix(&literal, 2).unwrap()),
        packet_len,
    )
}

/// VVVTTTILLLLLLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBBBBBB
fn extract_operator(raw: &str) -> (PacketData, usize) {
    let length_type_id = u8::from_str_radix(&raw[6..7], 2).unwrap();

    match length_type_id {
        0 => {
            let mut len_subpackets = usize::from_str_radix(&raw[7..7 + 15], 2).unwrap();
            let mut bin_remaining = &raw[7 + 15..];
            let mut subpackets = Vec::new();

            while len_subpackets > 0 {
                let (p, s) = extract_packet_stream(bin_remaining);
                bin_remaining = s;
                len_subpackets -= p.size;
                subpackets.push(p);
            }

            let packet_len = 7 + 15 + subpackets.iter().map(|p| p.size).sum::<usize>();

            (PacketData::Operator(subpackets), packet_len)
        }
        1 => {
            let mut qty_subpackets = usize::from_str_radix(&raw[7..7 + 11], 2).unwrap();
            let mut bin_remaining = &raw[7 + 11..];
            let mut subpackets = Vec::with_capacity(qty_subpackets);

            while qty_subpackets > 0 {
                let (p, s) = extract_packet_stream(bin_remaining);
                bin_remaining = s;
                qty_subpackets -= 1;
                subpackets.push(p);
            }

            let packet_len = 7 + 11 + subpackets.iter().map(|p| p.size).sum::<usize>();

            (PacketData::Operator(subpackets), packet_len)
        }
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    payload: PacketData,
    size: usize,
}

impl Packet {
    fn version_sum(&self) -> usize {
        fn flatten_packet_versions(packets: Vec<&Packet>) -> Vec<u8> {
            packets.iter().fold(Vec::new(), |mut acc, &cur| {
                acc.push(cur.version);

                if let PacketData::Operator(sp) = &cur.payload {
                    let mut sub = flatten_packet_versions(sp.iter().collect_vec());
                    acc.append(&mut sub);
                }

                acc
            })
        }

        flatten_packet_versions(vec![self])
            .into_iter()
            .map(|v| v as usize)
            .sum()
    }

    fn value(&self) -> usize {
        match &self.payload {
            PacketData::Literal(val) => *val,
            PacketData::Operator(packets) => match self.packet_type {
                PacketType::Sum => packets.iter().map(|p| p.value()).sum(),
                PacketType::Product => packets.iter().map(|p| p.value()).product(),
                PacketType::Minimum => packets.iter().map(|p| p.value()).min().unwrap(),
                PacketType::Maximum => packets.iter().map(|p| p.value()).max().unwrap(),
                PacketType::Literal => unreachable!(),
                PacketType::GreaterThan => {
                    let [first, second] =
                        <[usize; 2]>::try_from(packets.iter().map(|p| p.value()).collect_vec())
                            .unwrap();

                    (first > second) as usize
                }
                PacketType::LessThan => {
                    let [first, second] =
                        <[usize; 2]>::try_from(packets.iter().map(|p| p.value()).collect_vec())
                            .unwrap();

                    (second > first) as usize
                }
                PacketType::EqualTo => {
                    let [first, second] =
                        <[usize; 2]>::try_from(packets.iter().map(|p| p.value()).collect_vec())
                            .unwrap();

                    (first == second) as usize
                }
            },
        }
    }
}

#[derive(Debug)]
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl From<u8> for PacketType {
    fn from(n: u8) -> Self {
        match n {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            4 => Self::Literal,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::EqualTo,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum PacketData {
    Literal(usize),
    Operator(Vec<Packet>),
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        assert_eq!(super::part_1("8A004A801A8002F478"), 16);
        assert_eq!(super::part_1("620080001611562C8802118E34"), 12);
        assert_eq!(super::part_1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(super::part_1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::part_2("C200B40A82"), 3);
        assert_eq!(super::part_2("04005AC33890"), 54);
        assert_eq!(super::part_2("880086C3E88112"), 7);
        assert_eq!(super::part_2("CE00C43D881120"), 9);
        assert_eq!(super::part_2("D8005AC2A8F0"), 1);
        assert_eq!(super::part_2("F600BC2D8F"), 0);
        assert_eq!(super::part_2("9C005AC2F8F0"), 0);
        assert_eq!(super::part_2("9C0141080250320F1802104A08"), 1);
    }
}
