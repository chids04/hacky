use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

use hacky::Assembler;

use clap::Parser;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None, override_usage = "hacky <SRC> --out <OUT>")]
struct Args {
    /// path to the .asm file
    #[arg(required = true, index = 1)]
    src: PathBuf,

    /// path to place assembled .hack file
    #[arg(short, long)]
    out: Option<PathBuf>,
}


fn main() {
    let args = Args::parse();

    println!("src - {:?} out - {:?}", args.src, args.out);
    
    if !args.src.is_file() {
        println!(".asm file missing");
        return;
    }

    if args.src.extension() != Some(std::ffi::OsStr::new("asm")) {
        println!(".asm file required for input");
        return;
    }

    let out = match args.out {
        Some(p) => {
            if p.extension() != Some(std::ffi::OsStr::new("hack")) {
                println!(".hack file required for output");
                return;
            }
            else {
                p
            }
        }
        None => {
            let mut base = std::env::current_dir().unwrap();
            base.push(&args.src.file_name().unwrap());
            base
        },
    };

    let file = fs::File::open(args.src).expect("failed to open asm file");

    let reader = BufReader::new(file);

    let mut assembler = Assembler::new(reader).unwrap();

    match assembler.assemble(out) {
        Ok(()) => {}
        Err(e) => println!("{e:?}"),
    }
}
