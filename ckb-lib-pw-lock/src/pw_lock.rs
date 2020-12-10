use ckb_std::dynamic_loading::{CKBDLContext, Symbol};

use crate::code_hashes::SECP256K1_KECCAK256_ACPL;

type Main = unsafe extern "C" fn() -> i32;

const MAIN: &[u8; 4] = b"main";

pub struct PWLockAcpl {
    main: Symbol<Main>,
}

impl PWLockAcpl {
    pub fn load<T>(context: &mut CKBDLContext<T>) -> Self {
        let bin = context
            .load(&SECP256K1_KECCAK256_ACPL)
            .expect("load pw-lock secp256k1_keccak256_acpl");

        let main: Symbol<Main> = unsafe { bin.get(MAIN).expect("load main function") };

        PWLockAcpl { main }
    }

    pub fn verify(&self) -> Result<(), i32> {
        let main = &self.main;
        let error_code = unsafe { main() };
        if error_code != 0 {
            return Err(error_code);
        }
        Ok(())
    }
}
