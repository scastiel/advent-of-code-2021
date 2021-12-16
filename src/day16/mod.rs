mod input;

fn hex_to_bin(hex: &str) -> String {
    hex.chars()
        .map(|c| match c {
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
            c => panic!("Invalid hex char: {}", c),
        })
        .collect::<Vec<_>>()
        .join("")
}

fn bin_to_number(bin: &str) -> u64 {
    u64::from_str_radix(bin, 2).unwrap_or_else(|_| panic!("Failed to convert from bin: {}", bin))
}

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Op {
    pub fn from_type(type_: u64) -> Option<Op> {
        match type_ {
            0 => Some(Op::Sum),
            1 => Some(Op::Product),
            2 => Some(Op::Minimum),
            3 => Some(Op::Maximum),
            5 => Some(Op::GreaterThan),
            6 => Some(Op::LessThan),
            7 => Some(Op::EqualTo),
            _ => None,
        }
    }

    pub fn calc_val(&self, sps: &[Packet]) -> u64 {
        let mut vals = sps.iter().map(|s| s.val());
        match self {
            Op::Sum => vals.sum(),
            Op::Product => vals.product(),
            Op::Minimum => vals.min().unwrap(),
            Op::Maximum => vals.max().unwrap(),
            Op::GreaterThan => to_num(vals.next() > vals.next()),
            Op::LessThan => to_num(vals.next() < vals.next()),
            Op::EqualTo => to_num(vals.next() == vals.next()),
        }
    }
}

#[derive(Debug)]
enum Packet {
    LiteralValue(u64, u64),
    Operator(u64, Op, Vec<Packet>),
}

impl Packet {
    pub fn from_input(input: &'_ str) -> (Packet, &'_ str) {
        let (version_bin, input) = first_chars(input, 3);
        let v = bin_to_number(version_bin);
        let (type_bin, input) = first_chars(input, 3);
        let type_ = bin_to_number(type_bin);
        if type_ == 4 {
            let (value, input) = read_literal_value(input);
            (Packet::LiteralValue(v, value), input)
        } else {
            let (sub_packets, input) = read_length_and_sub_packets(input);
            let op = Op::from_type(type_).unwrap_or_else(|| panic!("Invalid type: {}", type_));
            (Packet::Operator(v, op, sub_packets), input)
        }
    }

    pub fn sum_versions(&self) -> u64 {
        match self {
            Packet::LiteralValue(v, _) => *v,
            Packet::Operator(v, _, sps) => *v + packets_versions_sum(sps),
        }
    }

    pub fn val(&self) -> u64 {
        match self {
            Packet::LiteralValue(_, value) => *value,
            Packet::Operator(_, op, sps) => op.calc_val(sps),
        }
    }
}

fn packets_versions_sum(packets: &[Packet]) -> u64 {
    packets.iter().map(|s| s.sum_versions()).sum::<u64>()
}

fn to_num(bool: bool) -> u64 {
    if bool {
        1
    } else {
        0
    }
}

fn first_chars(input: &'_ str, n: usize) -> (&'_ str, &'_ str) {
    (&input[0..n], &input[n..])
}

fn read_literal_value(input: &'_ str) -> (u64, &'_ str) {
    let mut input = input;
    let mut value_bin = String::new();
    loop {
        let (bin, rest) = first_chars(input, 5);
        input = rest;
        let (prefix, part) = first_chars(bin, 1);
        value_bin.push_str(part);
        if prefix == "0" {
            return (bin_to_number(&value_bin), input);
        }
    }
}

fn read_sub_packets_by_total_length(input: &'_ str, length: u64) -> (Vec<Packet>, &'_ str) {
    let original_length = input.len();
    let mut input = input;
    let mut sub_packets: Vec<Packet> = vec![];
    while input.len() > original_length - length as usize {
        let (packet, rest) = Packet::from_input(input);
        input = rest;
        sub_packets.push(packet);
    }
    (sub_packets, input)
}

fn read_sub_packets_by_num(input: &'_ str, number: u64) -> (Vec<Packet>, &'_ str) {
    let mut input = input;
    let mut sub_packets: Vec<Packet> = vec![];
    for _ in 0..number {
        let (packet, rest) = Packet::from_input(input);
        input = rest;
        sub_packets.push(packet);
    }
    (sub_packets, input)
}

fn read_length_and_sub_packets(input: &'_ str) -> (Vec<Packet>, &'_ str) {
    let (bit, input) = first_chars(input, 1);
    match bit {
        "0" => {
            let (length_bin, input) = first_chars(input, 15);
            read_sub_packets_by_total_length(input, bin_to_number(length_bin))
        }
        "1" => {
            let (length_bin, input) = first_chars(input, 11);
            read_sub_packets_by_num(input, bin_to_number(length_bin))
        }
        _ => panic!("Invalid bit: {}", bit),
    }
}

fn print_result(input: &str) {
    println!("\nInput: {:?}", input);
    let bin = hex_to_bin(input);
    println!("Binary: {:?}", bin);
    let (packet, _) = Packet::from_input(&bin);
    println!("Packet: {:?}", packet);
    println!("Versions sum: {:?}", packet.sum_versions());
    println!("Calculated value: {:?}", packet.val());
}

pub fn main() {
    print_result(input::EXAMPLE_1);
    print_result(input::EXAMPLE_2);
    print_result(input::EXAMPLE_3);
    print_result(input::EXAMPLE_4);
    print_result(input::EXAMPLE_5);
    print_result(input::EXAMPLE_6);
    print_result(input::EXAMPLE_7);
    print_result(input::EXERCISE);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_example1() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_1));
        assert_eq!(packet.sum_versions(), 6);
    }

    #[test]
    fn ex1_example2() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_2));
        assert_eq!(packet.sum_versions(), 9);
    }

    #[test]
    fn ex1_example3() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_3));
        assert_eq!(packet.sum_versions(), 14);
    }

    #[test]
    fn ex1_example4() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_4));
        assert_eq!(packet.sum_versions(), 16);
    }

    #[test]
    fn ex1_example5() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_5));
        assert_eq!(packet.sum_versions(), 12);
    }

    #[test]
    fn ex1_example6() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_6));
        assert_eq!(packet.sum_versions(), 23);
    }

    #[test]
    fn ex1_example7() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_7));
        assert_eq!(packet.sum_versions(), 31);
    }

    #[test]
    fn ex1_exercise() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXERCISE));
        assert_eq!(packet.sum_versions(), 893);
    }

    #[test]
    fn ex2_example1() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_1));
        assert_eq!(packet.val(), 2021);
    }

    #[test]
    fn ex2_example2() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_2));
        assert_eq!(packet.val(), 1);
    }

    #[test]
    fn ex2_example3() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_3));
        assert_eq!(packet.val(), 3);
    }

    #[test]
    fn ex2_example4() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_4));
        assert_eq!(packet.val(), 15);
    }

    #[test]
    fn ex2_example5() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_5));
        assert_eq!(packet.val(), 46);
    }

    #[test]
    fn ex2_example6() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_6));
        assert_eq!(packet.val(), 46);
    }

    #[test]
    fn ex2_example7() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXAMPLE_7));
        assert_eq!(packet.val(), 54);
    }

    #[test]
    fn ex2_exercise() {
        let (packet, _) = Packet::from_input(&hex_to_bin(input::EXERCISE));
        assert_eq!(packet.val(), 4358595186090);
    }
}
