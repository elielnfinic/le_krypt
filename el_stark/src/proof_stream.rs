use core::hash;

use serde::{Deserialize, Serialize};
use serde_pickle;
use sha3::{Sha3_256, Digest};

pub struct ProofStream{
    objects : Vec<Vec<u8>>,
    read_index : usize
}

impl ProofStream{
    fn new() -> ProofStream{
        ProofStream{
            objects : Vec::new(),
            read_index : 0
        }
    }

    fn push<T: Serialize>(&mut self, obj : T){
        let serialized_obj = serde_pickle::to_vec(&obj, Default::default()).unwrap();
        self.objects.push(serialized_obj);
    }

    fn pull<T: for<'de> Deserialize<'de>>(&mut self) -> T {
        assert!(self.read_index < self.objects.len(), "ProofStream: cannot pull object; queue empty.");
        let serialized_obj = &self.objects[self.read_index];
        self.read_index += 1;
        serde_pickle::from_slice(serialized_obj, Default::default()).unwrap()
    }

    fn serialize(&self) -> Vec<u8> {
        serde_pickle::to_vec(&self.objects, Default::default()).unwrap()
    }

    fn deserialize(bb: &[u8]) -> Self {
        let objects: Vec<Vec<u8>> = serde_pickle::from_slice(bb, Default::default()).unwrap();
        ProofStream {
            objects,
            read_index: 0,
        }
    }

    fn prover_fiat_shamir(&self, num_bytes: usize) -> Vec<u8> {
        let mut hasher = Sha3_256::default();        
        hasher.update(&self.serialize());
        hasher.finalize()[..num_bytes].to_vec()
    }

    fn verifier_fiat_shamir(&self, num_bytes: usize) -> Vec<u8> {
        let serialized_objects: Vec<Vec<u8>> = self.objects[..self.read_index].to_vec();
        let serialized_data = serde_pickle::to_vec(&serialized_objects, Default::default()).unwrap();
        let mut hasher = Sha3_256::default();
        hasher.update(&serialized_data);
        hasher.finalize()[..num_bytes].to_vec()
    }


}