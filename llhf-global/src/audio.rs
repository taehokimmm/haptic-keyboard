use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use lazy_static::lazy_static;
use rodio::source::Buffered;
use rodio::{Decoder, Source};

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
        ('C', String::from("K")),
        ('D', String::from("D")),
        ('E', String::from("EH")),
        ('F', String::from("F")),
        ('G', String::from("G")),
        ('H', String::from("HH")),
        ('I', String::from("IY")),
        ('J', String::from("G")),
        ('K', String::from("K")),
        ('L', String::from("L")),
        ('M', String::from("M")),
        ('N', String::from("N")),
        ('O', String::from("OW")),
        ('P', String::from("P")),
        ('Q', String::from("K")),
        ('R', String::from("R")),
        ('S', String::from("S")),
        ('T', String::from("T")),
        ('U', String::from("UW")),
        ('V', String::from("V")),
        ('W', String::from("UW")),
        ('X', String::from("Z")),
        ('Y', String::from("EY")),
        ('Z', String::from("Z")),
    ]);
    kvp
}

pub fn load_sources() -> HashMap<char, Buffered<Decoder<BufReader<File>>>> {
    let mut sources = HashMap::new();
    for (key, value) in KEY_VALUE.iter() {
        let mut path = PathBuf::from("phonemes-whc_modified");
        path.push(format!("{}.wav", value));
        let file = File::open(&path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        let source = source.buffered();
        sources.insert(*key, source);
    }
    sources
}
