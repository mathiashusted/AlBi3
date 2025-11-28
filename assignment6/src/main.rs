/*
Mathias Husted, Markus Apel


Aufgabe 1: Alignmentstatistik Praxis

In folgender Aufgabe sollen Sie eine Alignment-Statistik für simulierte Sequenzen
aufstellen, graphisch darstellen und mit einer Normalverteilung vergleichen.
a) Erzeugen Sie 20.000 zufällige DNA-Sequenzen der Länge 2000bp. Nehmen Sie an,
dass die Nukleotide unabhängig voneinander mit den folgenden Wahrscheinlichkeiten auftreten: P(A) = P(T) = 0.3 und P(C) = P(G) = 0.2. Implementieren
Sie dazu eine Funktion, die einen zufälligen Wert zwischen 0 und 1 als Input bekommt, und dann ein Nukleotid gemäß der obigen Verteilung ausgibt. Schreiben
Sie die simulierten Sequenzen in eine fasta-Datei mit Namen library.fasta.


*/
use weighted_rand::builder::*;
use std::fs::File;
use std::io::prelude::*;

const ALPHABET: [char; 4] = ['A', 'T', 'C', 'G'];
const ALPH_WEIGHTS: [f32; 4] = [0.3, 0.3, 0.2, 0.2];

fn gen_seq(sample_size: i32, seq_length: i32) -> Vec<String> {
    let builder = WalkerTableBuilder::new(&ALPH_WEIGHTS);
    let wa_table = builder.build();

    let mut sample = Vec::new();

    for _ in 0..sample_size {
        let mut sequence = String::new();
        for _ in 0..seq_length {
            let next_char: usize = wa_table.next();
            sequence.push(ALPHABET[next_char]);
        }
        sample.push(sequence);
    }
    
    sample
}

// This function is kind of useless now though 
fn get_nucleotide(input: f64) -> char {
    if input < 0.3 {
        ALPHABET[0]
    }
    else if input < 0.6 {
        ALPHABET[1]
    }
    else if input < 0.8 {
        ALPHABET[2]
    }
    else {
        ALPHABET[3]
    }
}

fn write_to_fasta(sample: Vec<String>, file_name: &str) -> std::io::Result<()>{
    let mut file = File::create(file_name)?;

    for (i, sequence) in sample.iter().enumerate() {
        writeln!(file, ">seq_{}", i)?;
        writeln!(file, "{}", sequence)?;
    }
    Ok(())
}

/*
b) Erzeugen Sie außerdem eine zweite Fasta-Datei mit Namen query.fasta, die eine
Seiquenz der Länge 200bp mit derselben Nukleotidverteilung enthält. Wie oft
kommt diese Sequenz in der Library vor?
*/

fn read_fasta(file_name: &str) -> std::io::Result<Vec<String>> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let mut sequences = Vec::new();
    let mut current_sequence = String::new();
    
    for line in contents.lines() {
        if line.starts_with('>') {
            if !current_sequence.is_empty() {
                sequences.push(current_sequence.clone());
                current_sequence.clear();
            }
        } else {
            current_sequence.push_str(line.trim());
        }
    }
    
    if !current_sequence.is_empty() {
        sequences.push(current_sequence);
    }
    
    Ok(sequences)
}

fn second_fasta() {
    let second_sample = gen_seq(1, 200);
    let query_sequence = second_sample[0].clone();
    
    write_to_fasta(second_sample, "query.fasta").expect("Failed to write query.fasta");

    let first_library = read_fasta("library.fasta").expect("Failed to read library.fasta");

    let mut matches: i32 = 0;

    for line in first_library.iter() {
        if line.contains(&query_sequence) {
            matches += 1;
        }
    }
    
    println!("Query sequence found {} times in library", matches);
}

// OUTPUT: Query sequence found 0 times in library

/*
c) Berechnen Sie die optimalen paarweisen Alignments (Query-Sequenz vs. jede
der Library-Sequenzen) mit Hilfe des Smith-Waterman-Algorithmus (search36)
aus dem FASTA-Paket1 und einer sehr hohen Gap-Penalty (z.B. -g -10) und
lassen Sie sich die maximalen Scores pro Library-Sequenz mit Hilfe der Option
-R scores.txt ausgeben.


OUTPUT:

# ./ssearch36 query.fasta library.fasta -g 10 -R scores.txt
>>>0 200	seq_0 - 200 nt
seq_300        2000 0 -1.00000 -1.00000   74    0    0  1  0    0    0    0  1  0   300   602890
seq_300        2000 1 -1.00000 -1.00000   63    0    0  0  0    0    0    0  0  0   300   602890
seq_301        2000 0 -1.00000 -1.00000   54    0    0  0  0    0    0    0  0  0   301   604900
seq_301        2000 1 -1.00000 -1.00000   60    0    0  1  0    0    0    0  1  0   301   604900
...
seq_14398      2000 0 -1.00000 -1.00000   53    0    0  0  0    0    0    0  0  0 14398 28957666
seq_14398      2000 1 -1.00000 -1.00000   69    0    0  1  0    0    0    0  1  0 14398 28957666
seq_14399      2000 0 -1.00000 -1.00000   63    0    0  1  0    0    0    0  1  0 14399 28959678
seq_14399      2000 1 -1.00000 -1.00000   50    0    0  0  0    0    0    0  0  0 14399 28959678
#Algorithm : Smith-Waterman (SSE2, Michael Farrar 2006) (7.2 Nov 2010/SIMDe Nov 2020)
#Parameters : +5/-4 matrix (5:-4), open/ext: -12/-10
#Query: 0>>> q_len: 200; seq_0 - 200 nt                                    
#Library: n_seq: 20000; library.fasta                                     
#Stat: zsflag: 2
#Stat: ngLambda: 0.191528; ngK: 0.172925; ngH: 0.356724
#Stat: ave_n1: 2000.0; sample_fract: 1; zs_off: 0
#Stat: ag_stat_str: {
#Stat: K: 0.648323; Lambda: 0.196894; a_n0f: 0; a_n0: 200 }
*/


fn main() {
    let sample = gen_seq(20000, 2000);

    write_to_fasta(sample, "library.fasta").expect("Failed to write FASTA file");

    second_fasta();
}