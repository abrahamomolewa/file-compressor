use flate2::write::GzEncoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;


fn main(){
    if args().len() != 3 {
        eprintln!("Usage: `source` `target`");
        return;
    }
    let mut input_file = BufReader::new(File::open(args().nth(1).unwrap()).unwrap());

    let output_file = File::create(args().nth(2).unwrap()).unwrap();

    let mut encode_file = GzEncoder::new(output_file, Compression::default());

    copy(&mut input_file, &mut encode_file).unwrap();

    let output = encode_file.finish().unwrap();
    
    
    println!( "original   file size: {:?} bytes", input_file.get_ref().metadata().unwrap().len());

    println!("Compressed file size: {:?}  bytes", output.metadata().unwrap().len());
       
}


