use std::array::IntoIter;

fn hex_to_bits(hex: u8) -> Option<IntoIter<bool, 4>> {
    let val = (hex as char).to_digit(16)?;

    Some(
        [
            val & (1 << 3) != 0,
            val & (1 << 2) != 0,
            val & (1 << 1) != 0,
            val & (1 << 0) != 0,
        ]
        .into_iter(),
    )
}

#[derive(Debug)]
struct Packet {
    version: u8,
    id: u8,
    data: PacketData,
}

#[derive(Debug)]
enum PacketData {
    Val { val: u64 },
    Operator { subpackets: Vec<Packet> },
}

fn get_n(bits: &mut impl Iterator<Item = bool>, n: usize) -> Option<u64> {
    let mut result = 0;

    for _ in 0..n {
        result = (result << 1) | bits.next()? as u64;
    }

    Some(result)
}

fn parse_packet<It: Iterator<Item = bool>>(mut bits: &mut It) -> Option<(Packet, u64)> {
    let version = get_n(&mut bits, 3)? as u8;
    let id = get_n(&mut bits, 3)? as u8;
    let mut parsed_bytes = 6;

    let data = if id == 4 {
        let mut val = 0;
        while let Some(true) = bits.next() {
            val = (val << 4) | get_n(&mut bits, 4)?;
            parsed_bytes += 5;
        }
        val = (val << 4) | get_n(&mut bits, 4)?;
        parsed_bytes += 5;

        PacketData::Val { val }
    } else {
        let len_type = bits.next()?;
        parsed_bytes += 1;
        let mut subpackets = Vec::new();

        if len_type {
            parsed_bytes += 11;
            let mut expected_packets = get_n(&mut bits, 11)?;
            while expected_packets > 0 {
                expected_packets -= 1;
                let (subpacket, sub_parsed_bytes) = parse_packet(bits)?;
                subpackets.push(subpacket);

                parsed_bytes += sub_parsed_bytes;
            }
        } else {
            parsed_bytes += 15;
            let goal_bytes = get_n(&mut bits, 15)? + parsed_bytes;

            while goal_bytes > parsed_bytes {
                let (subpacket, sub_parsed_bytes) = parse_packet(bits)?;
                subpackets.push(subpacket);

                parsed_bytes += sub_parsed_bytes;
            }
        }

        PacketData::Operator { subpackets }
    };

    Some((Packet { version, id, data }, parsed_bytes))
}

fn part1(packet: &Packet) -> usize {
    packet.version as usize
        + match &packet.data {
            PacketData::Val { val: _ } => 0,
            PacketData::Operator { subpackets } => subpackets.iter().map(part1).sum(),
        }
}

fn part2(packet: &Packet) -> u64 {
    match &packet.data {
        PacketData::Val { val } => *val,
        PacketData::Operator { subpackets } => match packet.id {
            0 => subpackets.iter().map(part2).sum(),
            1 => subpackets.iter().map(part2).product(),
            2 => subpackets.iter().map(part2).min().unwrap(),
            3 => subpackets.iter().map(part2).max().unwrap(),
            5 => (part2(&subpackets[0]) > part2(&subpackets[1])) as u64,
            6 => (part2(&subpackets[0]) < part2(&subpackets[1])) as u64,
            7 => (part2(&subpackets[0]) == part2(&subpackets[1])) as u64,
            _ => panic!(),
        },
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut bits = content
        .bytes()
        .flat_map(|hex| hex_to_bits(hex).expect("failed to parse packet"));

    let (packet, _) = parse_packet(&mut bits).expect("failed to parse packet");

    println!("part1: {}", part1(&packet));
    println!("part2: {}", part2(&packet));
}
