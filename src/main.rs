use std::io;
use std::fs;
use std::collections::HashMap;
use substring::Substring;
use regex::Regex;

enum Mode {
    Express,
    Transcribe,
    Translate,
    Exit,
}

fn main() {
    println!("KegelC: ArtX Interactive Mode - The Central Dogma of Biology\n");
    
    loop {
        println!("Avaliable Modes:");
        println!("0] Gene Expression (DNA   -RNA-> Amino)");
        println!("1] Transcription   (DNA   -----> RNA)");
        println!("2] Translation     (RNA   -----> Amino)");
        println!("*] Exit");
        
        println!("Select Mode:");
        let mut mode = String::new();
        io::stdin()
            .read_line(&mut mode)
            .expect("Failed to Read Desired Mode");

        let mode: u32 = match mode.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mode = match mode {
            0 => Mode::Express,
            1 => Mode::Transcribe,
            2 => Mode::Translate,
            _ => Mode::Exit,
        };

        //Enter Each Mode
        match mode {
            Mode::Express    => express_gene(),
            Mode::Transcribe => transcribe_mode(),
            Mode::Translate  => translate_mode(),
            Mode::Exit       => break,
        };
    }
    println!("Exiting Program - Goodbye!");
}

fn express_gene() -> String {
    println!("Welcome to Expression Mode!");
    let rna = transcribe_mode();
    rna

}

fn transcribe_dna(sense: &String, antisense: &mut String, rna: &mut String) {
    rna.push_str(&sense.replace('T', "U"));
    
    for base in sense.chars() {
        antisense.push(match base {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
             _  => 'X',
        });
    }
}

fn transcribe_mode() -> String {
    println!("Welcome to Transcription Mode!");
    println!("Enter a DNA Sequence (Sense Strand):");
    
    let mut sense = String::new();
    io::stdin()
        .read_line(&mut sense)
        .expect("Failed to Read DNA Sequence");
    
    let sense = sense.trim().to_uppercase();
    
    let mut antisense = String::new();
    let mut rna = String::new();
    let mut amino = String::new();

    transcribe_dna(&sense, &mut antisense, &mut rna);

    println!("Antisense Strand: {antisense}");
    println!("RNA Strand      : {rna}");

    translate_rna(&mut rna, &mut amino);

    println!("Polypeptide Chain: {amino}");
    amino 
}

fn translate_rna(rna: &mut String, amino: &mut String) {
    let mut table = HashMap::<String, String>::new();
    load_amino_file("amino_table.txt",&mut table);

    let start_codon = rna.find("AUG").unwrap();
    let translatable_rna = rna.split_off(start_codon);

    let codon_re = Regex::new("(...)").unwrap();
    let codons = codon_re.captures_iter(&translatable_rna);

    let mut stop_codon = String::new();
    stop_codon.push_str("Stop");

    for codon in codons {
        let acid = table.get(&codon[0]).unwrap_or(&stop_codon);
        if acid.eq(&stop_codon) {
            amino.push_str(&stop_codon);
            break;
        }
        amino.push_str(acid);
        amino.push_str(", ");
    }
}

fn translate_mode() -> String {
    println!("Welcome to Translation Mode!");
    println!("Amino Acid Table Loaded!");

    println!("Input an RNA Sequence:");
    let mut rna = String::new();
    io::stdin()
        .read_line(&mut rna)
        .expect("Failed to Read RNA Sequence");
    let mut rna = rna.trim().to_uppercase();
    
    let mut amino = String::new();
    translate_rna(&mut rna, &mut amino);
    
    println!("Polypeptide Chain: {amino}");

    amino
}


//Helper Functions
fn load_amino_file(filename: &str, table: &mut HashMap::<String, String>) {
    let contents = fs::read_to_string(filename)
        .expect("Error Reading File");
    let contents = contents.trim();

    let split = contents.split('\n');

    for translation in split {
        let triplet = translation.substring(0,3).to_uppercase();
        let amino = translation.substring(4, translation.len());
        table.insert(String::from(triplet), String::from(amino));
    }
}
