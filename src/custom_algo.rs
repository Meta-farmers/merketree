use rs_merkle::Hasher;
use ethers::utils::keccak256;
//use sha3::Keccak256;
#[derive(Clone)]
pub struct Keccak256Algorithm {}

impl Hasher for Keccak256Algorithm {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> [u8; 32] {
        keccak256(data)
    }

    fn concat_and_hash(_left: &Self::Hash, _right: Option<&Self::Hash>) -> Self::Hash {
        todo!()
    }

    fn hash_size() -> usize {
        todo!()
    }
}
