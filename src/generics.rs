use codec::{Decode, Encode};
use elgamal_trex::RawPublicKey;
use sp_core::U256;

/// Minimum difficulty.
pub const MIN_DIFFICULTY: u128 = 40;
/// Maximum difficulty.
pub const MAX_DIFFICULTY: u128 = 224;

/// type alias for solutions of pollard rho method.
pub type Solutions<I> = (Solution<I>, Solution<I>);

/// Solution within pollard rho method.
#[derive(Debug, Clone, Eq, PartialEq, Encode, Decode)]
pub struct Solution<I> {
    pub a: I,
    pub b: I,
    pub n: I,
}

/// The raw form of integer as seeds to derive a chain of public keys.
pub type RawKeySeeds = [RawKeySeedsData; (MAX_DIFFICULTY - MIN_DIFFICULTY) as usize];

#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug, Copy)]
pub enum RawKeySeedsData {
    U128(u128),
    U256(U256),
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, Debug)]
pub struct Seal {
    /// Mining difficulty of current block sealed by this seal.
    pub difficulty: u128,
    /// The public key being mined in current block.
    pub pubkey: RawPublicKey,
    /// A list of seeds to derive the public keys for next blocks.
    pub seeds: RawKeySeeds,
    /// A pair of solution for current time-lock puzzle found by mining current block.
    pub solutions: Solutions<U256>,
    /// A nonce value to seal and verify current mining works.
    pub nonce: U256,
}