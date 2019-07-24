use super::*;
use ink_core::{memory::format, storage};
use ink_model::{state, EnvHandler};
use ink_utils::hash;
use primitives::{
    default::*,
    events::*,
    traits::{Member, SimpleArithmetic},
};

state! {
    /// Each plasma chain MUST have at least one commitment contract.
    /// Commitment contracts hold the block headers for the plasma chain.
    /// Whenever the operator creates a new plasma block, they MUST publish this block to the commitment contract.
    pub struct Commitment {
        /// Block number of the most recently published plasma block.
        /// `uint256 public currentBlock;`
        current_block: storage::Value<BlockNumber>,

        /// Mapping from block number to block header.
        /// `mapping (uint256 => bytes) public blocks;`
        blocks: storage::HashMap<BlockNumber, Hash>,
    }
}

pub fn default_hash(data: &[u8]) -> Hash {
    Hash::decode(&mut &hash::keccak256(data)[..]).expect("keccak256 error")
}

pub fn concat_bytes<T: Encode, U: Encode>(a: &T, b: &U) -> Vec<u8> {
    a.encode()
        .iter()
        .chain(b.encode().iter())
        .map(|x| *x)
        .collect::<Vec<_>>()
}

pub fn concat_hash<T, U, F, H>(a: &T, b: &U, hash: F) -> H
where
    T: Encode,
    U: Encode,
    H: Encode + Eq + Copy,
    F: FnOnce(&[u8]) -> H,
{
    hash(&concat_bytes(a, b)[..])
}

impl traits::Verify for InclusionProof<RangeNumber> {
    /// Verify a state_update. Return `true` if state_update is valid for the value.
    fn verify<T, I>(&self, state_update: &primitives::StateUpdate<T, I>, root: Hash) -> bool
    where
        T: Member + Codec,
        I: Member + SimpleArithmetic + Codec,
    {
        let mut current_node = MerkleIndexTreeInternalNode::<RangeNumber> {
            index: state_update.range.start.clone().as_(),
            hash: default_hash(&state_update.encode()[..]),
        };

        for x in self.proofs.iter() {
            if self.idx < x.index {
                current_node = MerkleIndexTreeInternalNode::<RangeNumber> {
                    index: current_node.index.clone(),
                    hash: concat_hash(&current_node, x, default_hash),
                };
            } else {
                current_node = MerkleIndexTreeInternalNode::<RangeNumber> {
                    index: x.index.clone(),
                    hash: concat_hash(x, &current_node, default_hash),
                };
            }
        }
        default_hash(&current_node.encode()[..]) == root
    }
}

impl traits::Commitment for Commitment {
    /// Initilizes our state to `current_block is 0` upon deploying our smart contract.
    fn deploy(&mut self, env: &mut EnvHandler<ink_core::env::ContractEnv<DefaultSrmlTypes>>) {
        self.current_block.set(0)
    }

    /// Returns the current block number.
    fn current_block(
        &self,
        env: &mut EnvHandler<ink_core::env::ContractEnv<DefaultSrmlTypes>>,
    ) -> BlockNumber {
        let current_block = *self.current_block;
        env.println(&format!("Commitment::current_block = {:?}", current_block));
        current_block
    }

    /// Returns the balance of the given AccountId.
    fn block_hash(
        &self,
        env: &mut EnvHandler<ink_core::env::ContractEnv<DefaultSrmlTypes>>,
        number: BlockNumber,
    ) -> Option<Hash> {
        if let Some(block) = self.blocks.get(&number) {
            env.println(&format!(
                "Commitment::block_hash(number = {:?}) = {:?}",
                number, block
            ));
            return Some(block.clone());
        }
        env.println(&format!(
            "Commitment::block_hash(number = {:?}) = None)",
            number
        ));
        None
    }

    /// Allows a user to submit a block with the given header.
    /// `function submitBlock(bytes _header) public`
    fn submit_block(
        &mut self,
        env: &mut EnvHandler<ink_core::env::ContractEnv<DefaultSrmlTypes>>,
        header: Hash,
    ) -> primitives::Result<BlockSubmitted> {
        *self.current_block += 1;
        self.blocks.insert(*self.current_block, header.clone());
        Ok(BlockSubmitted {
            number: *self.current_block,
            header: header,
        })
    }

    /// Inclusion Proof.
    /// This function verifies state_update in PlasmaChain with inclusion_proof.
    fn verify_state_update_inclusion<T, P, I>(
        &self,
        env: &mut EnvHandler<ink_core::env::ContractEnv<DefaultSrmlTypes>>,
        state_update: &primitives::StateUpdate<T, I>, // leaf
        inclusion_proof: &P,                          // inclusion_proof
    ) -> bool
    where
        T: Member + Codec,
        P: Member + traits::Verify + Codec,
        I: Member + SimpleArithmetic + Codec,
    {
        if let Some(root_hash) = self.blocks.get(&state_update.plasma_block_number) {
            return inclusion_proof.verify(state_update, *root_hash);
        }
        false
    }

    /// Inclusion Proof upper layer.
    /// verifyAssetStateRootInclusion
    fn verify_asset_state_root_inclusion<T, P, I>(
        &self,
        env: &mut EnvHandler<ink_core::env::ContractEnv<DefaultSrmlTypes>>,
        asset_state: &primitives::StateUpdate<T, I>,
        inclusion_proof: &P,
    ) -> bool
    where
        T: Member + Codec,
        P: Member + traits::Verify + Codec,
        I: Member + SimpleArithmetic + Codec,
    {
        if let Some(root_hash) = self.blocks.get(&asset_state.plasma_block_number) {
            return inclusion_proof.verify(asset_state, *root_hash);
        }
        false
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;
    use crate::traits::Commitment as _;
    use ink_core::storage::{
        alloc::{AllocateUsing, BumpAlloc, Initialize as _},
        Key,
    };
    use ink_model::EnvHandler;
    impl Commitment {
        /// Deploys the testable contract by initializing it with the given values.
        pub fn deploy_mock() -> (
            Self,
            EnvHandler<ink_core::env::ContractEnv<DefaultSrmlTypes>>,
        ) {
            let (mut commitment, mut env) = unsafe {
                let mut alloc = BumpAlloc::from_raw_parts(Key([0x0; 32]));
                (
                    Self::allocate_using(&mut alloc),
                    AllocateUsing::allocate_using(&mut alloc),
                )
            };
            commitment.initialize(());
            commitment.deploy(&mut env);
            (commitment, env)
        }
    }

    #[test]
    fn it_works() {
        let (mut contract, mut env) = Commitment::deploy_mock();
        assert_eq!(contract.current_block(&mut env), 0);
        assert_eq!(contract.block_hash(&mut env, 0), None);

        let header_1: Hash =
            Hash::decode(&mut &[1u8; 32].to_vec()[..]).expect("hash decoded error.");
        assert_eq!(
            Ok(BlockSubmitted {
                number: 1,
                header: header_1.clone(),
            }),
            contract.submit_block(&mut env, header_1.clone())
        );
        assert_eq!(contract.current_block(&mut env), 1);
        assert_eq!(contract.block_hash(&mut env, 0), None);
        assert_eq!(contract.block_hash(&mut env, 1), Some(header_1));

        let header_2: Hash =
            Hash::decode(&mut &[2u8; 32].to_vec()[..]).expect("hash decoded error.");
        assert_eq!(
            Ok(BlockSubmitted {
                number: 2,
                header: header_2.clone(),
            }),
            contract.submit_block(&mut env, header_2.clone())
        );
        assert_eq!(contract.current_block(&mut env), 2);
        assert_eq!(contract.block_hash(&mut env, 0), None);
        assert_eq!(contract.block_hash(&mut env, 1), Some(header_1));
        assert_eq!(contract.block_hash(&mut env, 2), Some(header_2));
    }

    #[test]
    fn verify_inclusion_proof() {
        // TODO
    }
}
