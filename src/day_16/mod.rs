use std::convert::TryInto;

const LITERAL_VALUE_ID: u64 = 4;

enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}
use Operation::*;

enum PacketContent {
    Literal {
        value: u64,
    },
    Operator {
        operator: Operation,
        subpackets: Vec<Packet>,
    },
}

struct Packet {
    content: PacketContent,
    version: u8,
}

impl Packet {
    fn version_sum(&self) -> u32 {
        self.version as u32
            + match &self.content {
                PacketContent::Literal { value: _ } => 0,
                PacketContent::Operator {
                    operator: _,
                    subpackets,
                } => subpackets.iter().map(Packet::version_sum).sum::<u32>(),
            }
    }

    fn value(&self) -> u64 {
        match &self.content {
            PacketContent::Literal { value } => *value,
            PacketContent::Operator {
                operator,
                subpackets,
            } => {
                let mut values = subpackets.iter().map(Packet::value);
                match operator {
                    Sum => values.sum(),
                    Product => values.product(),
                    Minimum => values.min().unwrap(),
                    Maximum => values.max().unwrap(),
                    GreaterThan => (values.next() > values.next()) as u64,
                    LessThan => (values.next() < values.next()) as u64,
                    EqualTo => (values.next() == values.next()) as u64,
                }
            }
        }
    }
}

fn hex_to_bin(hex: &str) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn parse_bin(iter: &mut dyn Iterator<Item = char>, n: usize) -> u64 {
    u64::from_str_radix(&iter.take(n).collect::<String>(), 2).unwrap()
}

fn parse_packet(bits: &mut dyn Iterator<Item = char>) -> Packet {
    let version = parse_bin(bits, 3).try_into().unwrap();
    let id = parse_bin(bits, 3);
    let content = if id == LITERAL_VALUE_ID {
        let mut val = String::new();
        loop {
            let next = bits.next();
            val.extend(bits.take(4));
            if Some('0') == next {
                break;
            }
        }
        PacketContent::Literal {
            value: u64::from_str_radix(&val, 2).unwrap(),
        }
    } else {
        let mut subpackets = vec![];
        if Some('0') == bits.next() {
            let length = parse_bin(bits, 15) as usize;
            let mut subpacket_bits = bits.take(length).peekable();
            while matches!(subpacket_bits.peek(), Some(_)) {
                subpackets.push(parse_packet(&mut subpacket_bits));
            }
        } else {
            let num_subpackets = parse_bin(bits, 11);
            for _ in 0..num_subpackets {
                subpackets.push(parse_packet(bits));
            }
        }

        let operator = match id {
            0 => Sum,
            1 => Product,
            2 => Minimum,
            3 => Maximum,
            5 => GreaterThan,
            6 => LessThan,
            7 => EqualTo,
            x => panic!("Unexpected operator type {}", x),
        };

        PacketContent::Operator {
            operator,
            subpackets,
        }
    };

    Packet { content, version }
}
fn parse(input: &str) -> Packet {
    let bin = hex_to_bin(input);
    parse_packet(&mut bin.chars().peekable())
}

#[aoc(day16, part1)]
fn part_1(input: &str) -> u32 {
    parse(input).version_sum()
}

#[aoc(day16, part2)]
fn part_2(input: &str) -> u64 {
    parse(input).value()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test() {
        let input1 = "8A004A801A8002F478";
        assert_eq!(16, part_1(input1));

        let input2 = "620080001611562C8802118E34";
        assert_eq!(12, part_1(input2));

        let input3 = "C0015000016115A2E0802F182340";
        assert_eq!(23, part_1(input3));

        let input4 = "A0016C880162017C3686B18A3D4780";
        assert_eq!(31, part_1(input4));

        let input1 = "C200B40A82";
        assert_eq!(3, part_2(input1));

        let input2 = "04005AC33890";
        assert_eq!(54, part_2(input2));

        let input3 = "880086C3E88112";
        assert_eq!(7, part_2(input3));

        let input4 = "CE00C43D881120";
        assert_eq!(9, part_2(input4));

        let input4 = "D8005AC2A8F0";
        assert_eq!(1, part_2(input4));

        let input4 = "F600BC2D8F";
        assert_eq!(0, part_2(input4));

        let input4 = "9C005AC2F8F0";
        assert_eq!(0, part_2(input4));

        let input4 = "9C0141080250320F1802104A08";
        assert_eq!(1, part_2(input4));
    }
}
