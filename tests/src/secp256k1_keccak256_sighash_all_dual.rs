use super::common::{
    eth160, sign_tx_keccak256, MAX_CYCLES, SECP256K1_DATA, SECP256K1_KECCAK256_SIGHASH_DUAL,
};
use super::*;
use ckb_testtool::context::Context;
use ckb_tool::ckb_crypto::secp::Generator;
use ckb_tool::ckb_types::core::{Capacity, TransactionBuilder, TransactionView};
use ckb_tool::ckb_types::packed::{CellDep, CellInput, CellOutput, WitnessArgsBuilder};
use ckb_tool::ckb_types::{bytes::Bytes, prelude::*};
use rand::{thread_rng, Rng};

fn gen_tx(lock_args: Bytes) -> (Context, TransactionView) {
    build_test_context(vec![(lock_args, 1)])
}

fn build_test_context(grouped_args: Vec<(Bytes, usize)>) -> (Context, TransactionView) {
    let mut context = Context::default();
    let mut rng = thread_rng();

    // Deploy contract lockscript
    let pw_lock_bin = Loader::default().load_binary("pw-lock-dynamic-verify");
    let pw_lock_out_point = context.deploy_cell(pw_lock_bin);
    let pw_lock_dep = CellDep::new_builder()
        .out_point(pw_lock_out_point.clone())
        .build();

    // Deploy secp256k1_keccak256_lock_lib
    let secp256k1_keccak256_lib_out_point =
        context.deploy_cell(Bytes::from(SECP256K1_KECCAK256_SIGHASH_DUAL.as_slice()));
    let secp256k1_keccak256_dep = CellDep::new_builder()
        .out_point(secp256k1_keccak256_lib_out_point)
        .build();

    // Deploy secp256k1_data
    let secp256k1_data_out_point = context.deploy_cell(Bytes::from(SECP256K1_DATA.as_slice()));
    let secp256k1_data_dep = CellDep::new_builder()
        .out_point(secp256k1_data_out_point)
        .build();

    let deps = vec![pw_lock_dep, secp256k1_data_dep, secp256k1_keccak256_dep];

    let output_lock_args = grouped_args[0].0.clone();

    let mut inputs = vec![];
    let mut witnesses = vec![];
    for (args, inputs_size) in grouped_args {
        for _ in 0..inputs_size {
            let lock_script = context
                .build_script(&pw_lock_out_point, args.clone())
                .expect("build lock script");

            let input_out_point = context.create_cell(
                CellOutput::new_builder()
                    .capacity(Capacity::shannons(100).pack())
                    .lock(lock_script)
                    .build(),
                Bytes::new(),
            );

            let input = CellInput::new_builder()
                .previous_output(input_out_point)
                .build();

            let mut random_extra_witness = [0u8; 32];
            rng.fill(&mut random_extra_witness);
            let witness_args = WitnessArgsBuilder::default()
                .output_type(Some(Bytes::from(random_extra_witness.to_vec())).pack())
                .build();

            inputs.push(input);
            witnesses.push(witness_args.as_bytes());
        }
    }

    let output_lock_script = context
        .build_script(&pw_lock_out_point, output_lock_args)
        .expect("build output lock script");

    let output = CellOutput::new_builder()
        .capacity(Capacity::shannons(100).pack())
        .lock(output_lock_script)
        .build();

    let tx = TransactionBuilder::default()
        .inputs(inputs)
        .outputs(vec![output])
        .outputs_data(vec![Bytes::new()].pack())
        .cell_deps(deps)
        .witnesses(witnesses.pack())
        .build();

    (context, tx)
}

#[test]
fn test_keccak_all_unlock() {
    let privkey = Generator::random_privkey();
    let pubkey = privkey.pubkey().expect("pubkey");
    let pubkey_hash = eth160(pubkey);

    let (mut context, tx) = gen_tx(pubkey_hash);
    let tx = context.complete_tx(tx);
    let tx = sign_tx_keccak256(&mut context, tx, &privkey);

    context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
}
