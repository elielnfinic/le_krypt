use blake2::{Blake2b, Digest};

pub struct Merkle;

impl Merkle {
    pub fn commit(leafs: &[Vec<u8>]) -> Vec<u8> {
        assert!(leafs.len() & (leafs.len() - 1) == 0, "length must be power of two");

        if leafs.len() == 1 {
            return leafs[0].clone();
        } else {
            let mid = leafs.len() / 2;
            let left_commit = Merkle::commit(&leafs[..mid]);
            let right_commit = Merkle::commit(&leafs[mid..]);
            return Blake2b::digest(&[left_commit, right_commit].concat()).to_vec();
        }
    }

    pub fn open(index: usize, leafs: &[Vec<u8>]) -> Vec<Vec<u8>> {
        assert!(leafs.len() & (leafs.len() - 1) == 0, "length must be power of two");
        assert!(index < leafs.len(), "cannot open invalid index");

        if leafs.len() == 2 {
            return vec![leafs[1 - index].clone()];
        } else {
            let mid = leafs.len() / 2;
            if index < mid {
                let mut path = Merkle::open(index, &leafs[..mid]);
                path.push(Merkle::commit(&leafs[mid..]));
                return path;
            } else {
                let mut path = Merkle::open(index - mid, &leafs[mid..]);
                path.push(Merkle::commit(&leafs[..mid]));
                return path;
            }
        }
    }

    pub fn verify(root: &[u8], index: usize, path: &[Vec<u8>], leaf: &[u8]) -> bool {
        assert!(index < (1 << path.len()), "cannot verify invalid index");

        if path.len() == 1 {
            if index == 0 {
                return root == &*Blake2b::digest(&[leaf, &path[0]].concat()).to_vec();
            } else {
                return root == &*Blake2b::digest(&[&path[0], leaf].concat()).to_vec();
            }
        } else {
            let new_leaf = if index % 2 == 0 {
                Blake2b::digest(&[leaf, &path[0]].concat()).to_vec()
            } else {
                Blake2b::digest(&[&path[0], leaf].concat()).to_vec()
            };
            return Merkle::verify(root, index >> 1, &path[1..], &new_leaf);
        }
    }
}