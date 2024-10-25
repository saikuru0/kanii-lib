#![allow(dead_code)]
mod packets;

#[cfg(test)]
mod tests {
    use std::{fs::File, io::{self, BufRead}, path::Path};

    use packets::{server::ServerPacket, types::Sockchatable, Packet};

    use super::*;

    fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut lines = Vec::new();

        for line in reader.lines() {
            lines.push(line?);
        }

        Ok(lines)
    }

    #[test]
    fn it_works() {
        assert!(true)
    }

    #[test]
    fn server_tripple_conversion() {
        let lines = read_lines("server_packets.txt").expect("Could not read from file");

        for (i, line) in lines.iter().enumerate() {
            // println!("packeting line {}: {}", i, line);
            let packet = Packet::Server(line.parse::<ServerPacket>().unwrap());
            let converted = packet.to_sockstr();
            // println!("converted line {}: {}", i, converted);
            // println!("reconverting from {:#?}", packet);
            let reconverted = Packet::Server(converted.parse::<ServerPacket>().unwrap()).to_sockstr();
            println!("original {}: {}\nconverted: {}\nreconverted: {}\n", i, line, converted, reconverted);
            assert_eq!(converted, reconverted, "tripple conversion mismatch");
        }
    }
}
