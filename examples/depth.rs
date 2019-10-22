use hts::{AlignmentReader, BamFile, DepthIter};
use std::env::{args, var};
fn main() {
    let argv: Vec<_> = args().take(2).collect();

    if argv.len() != 2 {
        eprintln!("Usage: {} <BAM-FILE>", argv[0]);
        return;
    }

    let bam = BamFile::open(argv[1].as_str()).unwrap();

    if let Ok(data) = var("CRAM_REF") {
        bam.reference_path(data);
    }

    for (tid, pos, dep) in DepthIter::new(&bam) {
        println!("{} {} {}", tid, pos, dep);
    }
}
