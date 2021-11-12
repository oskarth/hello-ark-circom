use ark_circom::{CircomBuilder, CircomConfig};
use ark_std::rand::thread_rng;
use color_eyre::Result;

use ark_bn254::Bn254;
use ark_groth16::{
    create_random_proof as prove, generate_random_parameters, prepare_verifying_key, verify_proof,
};

// TODO: Run both circom-1 and circom-2 here? with flag
fn groth16_proof() -> Result<()> {
    println!("Load WASM and R1CS for witness and proof generation");

    // Load the WASM and R1CS for witness and proof generation
    // TODO: Not working with the circuit I generated for some reason
    let cfg = CircomConfig::<Bn254>::new(
        "./circuits/circom1_multiplier.wasm",
        "./circuits/circom1_multiplier.r1cs",
        //"./circuits/multiplier2.wasm",
        //"./circuits/multiplier2.r1cs",
        //"./circuits/mycircuit.wasm",
        //"./circuits/mycircuit.r1cs",
    )?;

    println!("Build public input config");

    // Insert our public inputs as key value pairs
    let mut builder = CircomBuilder::new(cfg);
    builder.push_input("a", 3);
    builder.push_input("b", 11);

    println!("Create setup");

    // create an empty instance for setting it up
    let circom = builder.setup();

    println!("Run trusted setup");

    // Run a trusted setup
    let mut rng = thread_rng();
    let params = generate_random_parameters::<Bn254, _, _>(circom, &mut rng)?;

    println!("Get circuit with witness");

    // Get the populated instance of the circuit with the witness
    let circom = builder.build()?;

    let inputs = circom.get_public_inputs().unwrap();

    // TODO Print actual proof
    println!("Generate proof");

    let proof = prove(circom, &params, &mut rng)?;

    let pvk = prepare_verifying_key(&params.vk);

    println!("Validate proof");

    // Check that the proof is valid
    let verified = verify_proof(&pvk, &proof, &inputs)?;

    assert!(verified);

    Ok(())
}

fn main() {
    match groth16_proof() {
        Ok(_val) => println!("All OK"),
        Err(err) =>
            panic!("Error: {:?}", err),
    }
}
