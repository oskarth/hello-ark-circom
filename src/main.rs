use ark_circom::{CircomBuilder, CircomConfig};
use ark_std::rand::thread_rng;
use color_eyre::Result;

use ark_bn254::Bn254;
use ark_groth16::{
    create_random_proof as prove, generate_random_parameters, prepare_verifying_key, verify_proof,
};

// Include ark circom for hello world
fn main() {
    println!("Hello, world!");

    // TODO Add these
    // Load the WASM and R1CS for witness and proof generation
    let cfg = CircomConfig::<Bn254>::new(
        "./mycircuit.wasm",
        "./mycircuit.r1cs",
    )?;

    // Insert our public inputs as key value pairs
    let mut builder = CircomBuilder::new(cfg);
    builder.push_input("a", 3);
    builder.push_input("b", 11);

    // Create an empty instance for setting it up
    let circom = builder.setup();

    // Run a trusted setup
    let mut rng = thread_rng();
    let params = generate_random_parameters::<Bn254, _, _>(circom, &mut rng)?;

    // Get the populated instance of the circuit with the witness
    let circom = builder.build()?;

    let inputs = circom.get_public_inputs().unwrap();

    // Generate the proof
    let proof = prove(circom, &params, &mut rng)?;

    // Check that the proof is valid
    let pvk = prepare_verifying_key(&params.vk);
    let verified = verify_proof(&pvk, &proof, &inputs)?;
    assert!(verified);

}
