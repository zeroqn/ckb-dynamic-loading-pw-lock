// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_lib_pw_lock::PWLockAcpl;
use ckb_std::debug;
use ckb_std::dynamic_loading::CKBDLContext;

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let mut context = unsafe { CKBDLContext::<[u8; 128 * 1024]>::new() };
    let pw_lock = PWLockAcpl::load(&mut context);
    pw_lock.main().map_err(|err| {
        debug!("pw-lock acpl error: {}", err);
        Error::PWLockAcpl
    })
}
