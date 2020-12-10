use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use blake2b_rs::Blake2bBuilder;

const BUF_SIZE: usize = 8 * 1024;
const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";

const PW_LOCK_CELL_PATH: &str = "../pw-lock/specs/cells";
const PW_LOCK_SECP256K1_KECCAK256: &str = "secp256k1_keccak256_sighash_all";
const PW_LOCK_SECP256K1_KECCAK256_ACPL: &str = "secp256k1_keccak256_sighash_all_acpl";

fn hash_cell_binary(path: &str) -> [u8; 32] {
    let cell_path = {
        let mut p = PathBuf::from(PW_LOCK_CELL_PATH);
        p.push(path);
        p
    };

    if !cell_path.exists() {
        panic!("{}/{} not found", PW_LOCK_CELL_PATH, path);
    }

    let mut buf = [0u8; BUF_SIZE];
    let mut blake2b = Blake2bBuilder::new(32)
        .personal(CKB_HASH_PERSONALIZATION)
        .build();

    let mut cell = File::open(cell_path).expect("open cell file");
    loop {
        let read_bytes = cell.read(&mut buf).expect("read cell");
        if read_bytes > 0 {
            blake2b.update(&buf[..read_bytes]);
        } else {
            break;
        }
    }

    let mut hash = [0u8; 32];
    blake2b.finalize(&mut hash);
    hash
}

fn main() {
    let out_path = Path::new("src").join("code_hashes.rs");
    let mut out_file = BufWriter::new(File::create(&out_path).expect("create code_hashes.rs"));

    let cells = vec![
        ("SECP256K1_KECCAK256", PW_LOCK_SECP256K1_KECCAK256),
        ("SECP256K1_KECCAK256_ACPL", PW_LOCK_SECP256K1_KECCAK256_ACPL),
    ];

    for (name, path) in cells {
        write!(
            &mut out_file,
            "pub const {}: [u8; 32] = {:?};\n",
            name,
            hash_cell_binary(path)
        )
        .expect("write to code_hashes.rs");
    }
}
