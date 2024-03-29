mod custom_algo;

use rs_merkle::algorithms::Sha256;
use rs_merkle::{Hasher, MerkleProof, MerkleTree};
use ethers::utils::keccak256;
use custom_algo::Keccak256Algorithm;


fn main() {

    let data =[
        "0x7B0c2F65A7DC95b11B0b99111192bfddA2F08271",
        "0x655d8A60345188b2C94d543dE4eafF58905A40fD",
        "0x597C9223bc620E1c170055958299cB7769b56eaA",
        "0x184E509eEba9b0dC4985c5eF298649a736c2c615",
        "0x6110b1BBb5Adb7BA07a137a86C02012F9Af88703",
        "0x3Dc4497BF5513AAF7559F553A8df5badb12Bbe89",
        "0x650bD7f008C6C02b4E66f905f11Fd2eFe444c19e",
        "0x4ce063D823DA4ecBb44d8F36Be933842686FAE78",
        "0xfB143327FF810A46320E542b960887c8933A3733",
        "0x78FdeA42009D1AB749494DE3fADAB5A59DdCfB07",
        "0x586EdFf0D8CB087629ab8c30f8346bB4A3e8Bdce",
        "0xB6163f1A9D6dB1e1254E0EC27FBD9C645E9F1F70",
        "0xeEf264cB766C62EA4a748b84Ca8068a7584972C8",
        "0x946278CA30Cc0Dc9cD1C19183CcEd21E1AD71cEF",
        "0xa5bbE2Cd62e275d4Ecaa9a752783823308ACc4d0",
        "0x23C625789c391463997267BDD8b21e5E266014F6",
        "0xb75aB455CFAb355E0EBcAaa1A9981c870ab99044",
        "0x273BD5258935608E999D23208852aeB8be99986d",
        "0xbFb7aA27c6A7977a65b97b679A68cDDF73b89509",
        "0x8cD6127C34B3AC7749430Ae466a0e8f9ebE0d364",
        "0x54a5B10D746d7b22cc457036e0666c22E3A47f6B",
        "0x527a1c6dbb19f745b8f97b87f9ccdc6071ce6dc3",
        "0xa06988C1fccB31fB9431F64e9eaD0E28b2222d95",
        "0x38D2c6C4C9524255dC582bDb1F9992eCd2DCb883",
        "0x040FC936073ff3233DF246b41e82310C97CbA7d4",
        "0x39DBD83bb30a980Fd59c51DCA6abABF4CfC84a84",
        "0x685B63Bd4BCA94B5A1DBcE071911a70f90a738da",
        "0x5c624E77fB9eB29B78F449e5A08c549b9d9520e9",
        "0x0b2Ddc7248a62C54CC3b62A9eb758F3fDC6f89e9",
        "0xB8F3AC467A137c4dA77F7841084825C122993746",
        "0x8e658122aC6c337d26b0d4C6159FD9b2B9804534",
        "0x4cB9362D9d3a2d83FD1838620F60930d9d27C488",
        "0x080461e636084A5aFf38D2f9A2cdCfAF33c797b1",
        "0x647f364cEbC5Ed0932C2C783c7e6d166d3aaC36c",
        "0xD7BC29b7a9072D8a829a3424190f8D1Cc771B7BC",
        "0x509B0E066f3F116ED498fEA800FE5cefc0f100bE",
        "0x53d8A3830cEc79A9cceBB9C0d6B463205F03363d",
        "0xBf06cec47bc981ab9f0522BC053A198acA644ac8",
        "0xc4D226755488A74787FC02CE7c5EB451D21252FD"];
    let leaves: Vec<[u8; 32]> = data
        .iter()
        .map(|x| Sha256::hash(x.as_bytes()))
        .collect();
    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);
    let indices_to_prove = vec![3, 4];
    let leaves_to_prove = leaves.get(3..5).ok_or("can't get leaves to prove").unwrap();
    let merkle_proof = merkle_tree.proof(&indices_to_prove);
    let merkle_root_string = merkle_tree.root_hex().unwrap();
    let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root").unwrap();
    let proof_bytes = merkle_proof.to_bytes();
    let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();
    let verify=  proof.verify(merkle_root, &indices_to_prove, leaves_to_prove, leaves.len());

    //println!("proof {:?}",proof.to_bytes());
   // println!("leaves to prove {:?}",leaves_to_prove);
    println!("root {:?}", merkle_root_string);
    println!("is OK {:?}",verify)

}
