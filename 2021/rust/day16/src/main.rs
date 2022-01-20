use utils::stdin_to_str_vec;
/*
- Input is hex, goes to binary.
- Each transmission contains a single packet, with more packets inside of it. The end might be extra zero padded.
- Packet structure:
    - First three bits = version
    - Next three bits = type ID

- types:
    - 4: Literal value
        - encode a single binary number
        - padded with zeros until len is multiple of four bits
    - Not 4: Operator
        - Length ID Bit:
            - 0 -> next 15 bits: total length in bits of subpackets in packet
            - 1 -> next 11 bits: number of sub packets in this packet


*/

struct Packet {
    version: usize,
    type_id: usize,
    value: usize,
    sub_packets: Option<Vec<Packet>>,
}

fn bin_to_usize(s: String) -> usize {
    usize::from_str_radix(s.as_str(), 2).unwrap()
}

impl Packet {
    fn parse_header(v: &Vec<char>, pntr: &mut usize) -> (usize, usize) {
        // ew
        // very python, figure out how to do this in rust
        let verison = bin_to_usize(v[*pntr..*pntr + 3].into_iter().collect::<String>());
        let type_id = bin_to_usize(v[*pntr + 3..*pntr + 6].into_iter().collect::<String>());
        *pntr += 6;
        (verison, type_id)
    }

    fn parse_body(type_id: usize, v: &Vec<char>, pntr: &mut usize) -> (usize, Option<Vec<Packet>>) {
        match type_id {
            4 => Packet::parse_literal(v, pntr),
            _ => Packet::parse_operator(type_id, v, pntr),
        }
    }

    fn parse_literal(v: &Vec<char>, pntr: &mut usize) -> (usize, Option<Vec<Packet>>) {
        let mut s = String::new();
        loop {
            let indicator = v[*pntr];
            *pntr += 1;
            for _ in 0..4 {
                s.push(v[*pntr]);
                *pntr += 1;
            }
            if indicator == '0' {
                break;
            }
        }
        (bin_to_usize(s), None)
    }

    fn parse_operator(
        type_id: usize,
        v: &Vec<char>,
        pntr: &mut usize,
    ) -> (usize, Option<Vec<Packet>>) {
        let length_type_id = v[*pntr];
        *pntr += 1;
        let sub_packets = match length_type_id {
            '0' => Packet::parse_length_type_id_0(v, pntr),
            '1' => Packet::parse_length_type_id_1(v, pntr),
            _ => panic!("nope"),
        };
        let value: usize = match type_id {
            0 => sub_packets.iter().map(|p| p.value).sum::<usize>(),
            1 => sub_packets.iter().map(|p| p.value).product::<usize>(),
            2 => sub_packets.iter().map(|p| p.value).min().unwrap(),
            3 => sub_packets.iter().map(|p| p.value).max().unwrap(),
            5 => {
                if sub_packets[0].value > sub_packets[1].value {
                    1 as usize
                } else {
                    0 as usize
                }
            }
            6 => {
                if sub_packets[0].value < sub_packets[1].value {
                    1
                } else {
                    0
                }
            }
            7 => {
                if sub_packets[0].value == sub_packets[1].value {
                    1
                } else {
                    0
                }
            }
            _ => panic!("nada"),
        };
        (value, Some(sub_packets))
    }

    fn parse_length_type_id_0(v: &Vec<char>, pntr: &mut usize) -> Vec<Packet> {
        let n_bits_to_parse = bin_to_usize(v[*pntr..*pntr + 15].into_iter().collect::<String>());
        *pntr += 15;
        let mut sub_packets = Vec::new();
        let parse_to = *pntr + n_bits_to_parse;
        while *pntr < parse_to {
            sub_packets.push(Packet::from_bits(v, pntr));
        }

        sub_packets
    }

    fn parse_length_type_id_1(v: &Vec<char>, pntr: &mut usize) -> Vec<Packet> {
        let n_packets_to_parse = usize::from_str_radix(
            v[*pntr..*pntr + 11]
                .into_iter()
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap();

        *pntr += 11;

        let mut sub_packets = Vec::with_capacity(n_packets_to_parse);

        for _ in 0..n_packets_to_parse {
            sub_packets.push(Packet::from_bits(v, pntr));
        }
        sub_packets
    }

    pub fn from_bits(v: &Vec<char>, pntr: &mut usize) -> Packet {
        let (version, type_id) = Packet::parse_header(v, pntr);
        let (value, sub_packets) = Packet::parse_body(type_id, v, pntr);
        Packet {
            version,
            type_id,
            value,
            sub_packets,
        }
    }
}

fn decode(code: &String) -> Vec<char> {
    code.chars()
        .map(|x| match x {
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
            _ => panic!("This is not right."),
        })
        .flat_map(|x| x.chars())
        .collect()
}

fn get_version_number(p: &Packet) -> usize {
    p.version
        + match &p.sub_packets {
            Some(x) => x.iter().map(get_version_number).sum(),
            None => 0,
        }
}

fn part_1(code: &String) -> usize {
    let bits = decode(code);
    let mut pntr: usize = 0;

    let packet = Packet::from_bits(&bits, &mut pntr);

    get_version_number(&packet)
}

fn part_2(code: &String) -> usize {
    let bits = decode(code);
    let mut pntr: usize = 0;

    let packet = Packet::from_bits(&bits, &mut pntr);

    packet.value
}

fn main() {
    let raw_input = &stdin_to_str_vec()[0];
    println!("{:?}", part_1(raw_input));
    println!("{:?}", part_2(raw_input));
}

#[test]
fn examples() {
    let mut raw_input = vec![
        ("8A004A801A8002F478", 16),
        ("620080001611562C8802118E34", 12),
        ("C0015000016115A2E0802F182340", 23),
        ("A0016C880162017C3686B18A3D4780", 31),
    ];
    for (code, answer) in raw_input {
        print!("{:?}", code);
        assert_eq!(part_1(&code.to_string()), answer);
    }
    raw_input = vec![
        ("C200B40A82", 3),
        ("04005AC33890", 54),
        ("880086C3E88112", 7),
        ("CE00C43D881120", 9),
        ("D8005AC2A8F0", 1),
        ("F600BC2D8F", 0),
        ("9C005AC2F8F0", 0),
        ("9C0141080250320F1802104A08", 1),
    ];
    for (code, answer) in raw_input {
        print!("{:?}", code);
        assert_eq!(part_2(&code.to_string()), answer);
    }
}
