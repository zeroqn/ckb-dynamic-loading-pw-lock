// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_lib_pw_lock::PWLockAcpl;
use ckb_std::debug;
use ckb_std::dynamic_loading::CKBDLContext;
use ckb_std::{
    ckb_types::{bytes::Bytes, prelude::*},
    high_level::load_script,
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    // return an error if args is invalid
    if args.is_empty() {
        return Err(Error::NoLockArgs);
    }
    if args.len() != 20 {
        return Err(Error::WrongArgsLength);
    }

    let mut context = unsafe { CKBDLContext::<[u8; 255 * 1024]>::new() };
    let pw_lock = PWLockAcpl::load(&mut context);

    let mut buf = [0u8; 20];
    buf.copy_from_slice(&args[0..20]);

    pw_lock.verify(&mut buf).map_err(|err| {
        debug!("pw-lock acpl error: {}", err);
        Error::PWLockAcpl
    })
}
