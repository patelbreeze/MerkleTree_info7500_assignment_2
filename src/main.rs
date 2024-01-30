// main.rs

mod util;
mod prover;
mod verifier;

// main.rs

use prover::gen_merkle_proof;
use util::{Hash32Bytes, MerkleProof, hash_leaf, hash_internal, encode_hash, write_merkle_proof};

fn main() {
    // Generate example leaves

    let leaves: Vec<String> = vec![
        "data item 0".to_string(),
        "data item 1".to_string(),
        "data item 2".to_string(),
        "data item 3".to_string(),
    ];

    // Specify the leaf position for which you want to generate a proof
    let leaf_position = 1;

    // Generate Merkle proof for the specified leaf position
    let proof_hashes = gen_merkle_proof(leaves.clone(), leaf_position);

    // Print the generated proof hashes
    println!("Merkle Proof Hashes:");
    for (i, hash) in proof_hashes.iter().enumerate() {
        println!("Level {}: {}", i, encode_hash(*hash));
    }

    // Create a MerkleProof struct
    let proof = MerkleProof {
        leaf_position,
        leaf_value: leaves[leaf_position].clone(),
        proof_hash_values_base64: proof_hashes.iter().map(|hash| encode_hash(*hash)).collect(),
        proof_hash_values: None,
    };

    // Write the Merkle proof to a file
    write_merkle_proof(&proof, "merkle_proof.yaml");

    // Additional code to demonstrate verification (using verifier.rs) can be added here.
}
