use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;


use lazy_static::lazy_static;
use rodio::{Decoder, Source};
use rodio::source::Buffered;

lazy_static! {
    static ref KEY_VALUE: HashMap<char, String> = create_kvp();
}

// AA → A
// AH → Y
// AW → W
// AY → J
// B → B
// C → C
// D → D
// DH → X
// EH → H
// EY → E
// F → F
// G → G
// IY → I
// K → K
// L → L
// M → M
// N → N
// OW → O
// P → P
// R → R
// S → S
// T → T
// UW → U, Q
// V → V
// Z → Z

fn create_kvp() -> HashMap<char, String> {
    let kvp: HashMap<char, String> = HashMap::from([
        ('A', String::from("AA")),
        ('B', String::from("B")),
        ('C', String::from("calibration")),
        ('D', String::from("D")),
        ('E', String::from("EH")),
        ('F', String::from("F")),
        ('G', String::from("G")),
        ('H', String::from("AH")),
        ('I', String::from("IY")),
        ('J', String::from("AY")),
        ('K', String::from("K")),
        ('L', String::from("L")),
        ('M', String::from("M")),
        ('N', String::from("N")),
        ('O', String::from("OW")),
        ('P', String::from("P")),
        ('Q', String::from("calibration2")),
        ('R', String::from("R")),
        ('S', String::from("S")),
        ('T', String::from("T")),
        ('U', String::from("UW")),
        ('V', String::from("V")),
        ('W', String::from("AW")),
        ('X', String::from("DH")),
        ('Y', String::from("IY")),
        ('Z', String::from("Z")),
    ]);
    kvp
}

pub fn load_sources() -> HashMap<char, Buffered<Decoder<BufReader<File>>>> {
    let mut sources = HashMap::new();
    for (key, value) in KEY_VALUE.iter() {
        let path = format!("phonemes-toh/{}.wav", value);
        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        let source = source.buffered();
        sources.insert(*key, source);
    }
    sources
}