use ckb_std::dynamic_loading::{CKBDLContext, Symbol};

use crate::code_hashes::SECP256K1_KECCAK256;

type VerifySecp256k1KeccakSighashAll = unsafe extern "C" fn(eth_address: *const u8) -> i32;

const VERIFY_SECP256K1_KECCAK_SIGHASH_ALL: &[u8; 35] = b"verify_secp256k1_keccak_sighash_all";

pub struct PWLockAcpl {
    verify_secp256k1_keccak_sighash_all: Symbol<VerifySecp256k1KeccakSighashAll>,
}

impl PWLockAcpl {
    pub fn load<T>(context: &mut CKBDLContext<T>) -> Self {
        let bin = context
            .load(&SECP256K1_KECCAK256)
            .expect("load pw-lock secp256k1_keccak256_acpl");

        let verify_secp256k1_keccak_sighash_all: Symbol<VerifySecp256k1KeccakSighashAll> = unsafe {
            bin.get(VERIFY_SECP256K1_KECCAK_SIGHASH_ALL)
                .expect("load verify function")
        };

        PWLockAcpl {
            verify_secp256k1_keccak_sighash_all,
        }
    }

    pub fn verify(&self, eth_address: &mut [u8; 20]) -> Result<(), i32> {
        let f = &self.verify_secp256k1_keccak_sighash_all;
        let error_code = unsafe { f(eth_address.as_mut_ptr()) };
        if error_code != 0 {
            return Err(error_code);
        }
        Ok(())
    }
}
