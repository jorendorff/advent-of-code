use aoc_runner_derive::*;

#[aoc_generator(day16, part1, jorendorff)]
#[aoc_generator(day16, part2, jorendorff)]
fn parse_input(text: &str) -> Vec<bool> {
    text.trim()
        .chars()
        .flat_map(|c| {
            let digit = c.to_digit(16).unwrap();
            (0..4).rev().map(move |bit| (1 << bit) & digit != 0)
        })
        .collect()
}

#[derive(PartialEq, Debug)]
struct Packet {
    version: u32,
    type_id: u32,
    payload: Payload,
}

#[derive(PartialEq, Debug)]
enum Payload {
    Literal(u64),
    SubPackets(Vec<Packet>),
}

struct Parser {
    bits: Vec<bool>,
    point: usize,
}

impl Parser {
    fn parse_packet(bits: &[bool]) -> anyhow::Result<Packet> {
        let mut parser = Parser {
            bits: bits.to_vec(),
            point: 0,
        };
        let packet = parser.read_packet();
        assert!(parser.bits[parser.point..].iter().all(|&bit| !bit));
        Ok(packet)
    }

    fn read(&mut self, nbits: usize) -> u32 {
        let mut total = 0;
        for _i in 0..nbits {
            total <<= 1;
            total |= self.bits[self.point] as u32;
            self.point += 1;
        }
        total
    }

    fn read_packet(&mut self) -> Packet {
        let version = self.read(3);
        let type_id = self.read(3);
        let payload = match type_id {
            4 => {
                let mut n = 0u64;
                loop {
                    let more = self.bits[self.point];
                    self.point += 1;
                    n <<= 4;
                    n |= self.read(4) as u64;
                    if !more {
                        break;
                    }
                }
                Payload::Literal(n)
            }
            _ => match self.read(1) {
                0 => {
                    let subpacket_len_bits = self.read(15) as usize;
                    let end = self.point + subpacket_len_bits;
                    let mut subpackets = vec![];
                    while self.point < end {
                        subpackets.push(self.read_packet());
                    }
                    Payload::SubPackets(subpackets)
                }
                1 => {
                    let subpacket_count = self.read(11);
                    Payload::SubPackets((0..subpacket_count).map(|_| self.read_packet()).collect())
                }
                _ => unreachable!(),
            },
        };
        Packet {
            version,
            type_id,
            payload,
        }
    }
}

impl Packet {
    fn version_sum(&self) -> u32 {
        self.version
            + match &self.payload {
                Payload::SubPackets(kids) => kids.iter().map(Packet::version_sum).sum::<u32>(),
                _ => 0,
            }
    }

    fn args(&self) -> impl Iterator<Item = u64> + '_ {
        match &self.payload {
            Payload::SubPackets(v) => v.iter().map(Self::eval),
            _ => unreachable!(),
        }
    }

    fn eval(&self) -> u64 {
        match self.type_id {
            0 => self.args().sum(),
            1 => self.args().product(),
            2 => self.args().min().unwrap(),
            3 => self.args().max().unwrap(),
            4 => match &self.payload {
                Payload::Literal(x) => *x,
                _ => unreachable!(),
            },
            5 => {
                let mut args = self.args();
                let a = args.next().unwrap();
                let b = args.next().unwrap();
                if a > b {
                    1
                } else {
                    0
                }
            }
            6 => {
                let mut args = self.args();
                let a = args.next().unwrap();
                let b = args.next().unwrap();
                if a < b {
                    1
                } else {
                    0
                }
            }
            7 => {
                let mut args = self.args();
                let a = args.next().unwrap();
                let b = args.next().unwrap();
                if a == b {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!("bad opcode"),
        }
    }
}

#[aoc(day16, part1, jorendorff)]
fn part_1(bits: &[bool]) -> u32 {
    Parser::parse_packet(bits).unwrap().version_sum()
}

#[aoc(day16, part2, jorendorff)]
fn part_2(bits: &[bool]) -> u64 {
    Parser::parse_packet(bits).unwrap().eval()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            Parser::parse_packet(&parse_input("D2FE28")).unwrap(),
            Packet {
                version: 6,
                type_id: 4,
                payload: Payload::Literal(2021)
            }
        );
        assert_eq!(
            Parser::parse_packet(&parse_input("EE00D40C823060")).unwrap(),
            Packet {
                version: 7,
                type_id: 3,
                payload: Payload::SubPackets(vec![
                    Packet {
                        version: 2,
                        type_id: 4,
                        payload: Payload::Literal(1)
                    },
                    Packet {
                        version: 4,
                        type_id: 4,
                        payload: Payload::Literal(2)
                    },
                    Packet {
                        version: 1,
                        type_id: 4,
                        payload: Payload::Literal(3)
                    },
                ])
            }
        );
        assert_eq!(part_1(&parse_input("8A004A801A8002F478")), 16);
        assert_eq!(part_1(&parse_input("620080001611562C8802118E34")), 12);
        assert_eq!(part_1(&parse_input("C0015000016115A2E0802F182340")), 23);
        assert_eq!(part_1(&parse_input("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse_input("C200B40A82")), 3);
        assert_eq!(part_2(&parse_input("04005AC33890")), 54);
        assert_eq!(part_2(&parse_input("880086C3E88112")), 7);
        assert_eq!(part_2(&parse_input("CE00C43D881120")), 9);
        assert_eq!(part_2(&parse_input("D8005AC2A8F0")), 1);
        assert_eq!(part_2(&parse_input("F600BC2D8F")), 0);
        assert_eq!(part_2(&parse_input("9C005AC2F8F0")), 0);
        assert_eq!(part_2(&parse_input("9C0141080250320F1802104A08")), 1);
    }
}
