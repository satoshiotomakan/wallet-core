// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use crate::Error;
use starknet_ff::FieldElement;
use std::ops::Range;
use tw_hash::H256;
use tw_utils::traits::ToBytesVec;

/// cbindgen:ignore
const R_RANGE: Range<usize> = 0..32;
/// cbindgen:ignore
const S_RANGE: Range<usize> = 32..64;

pub struct Signature {
    pub(crate) signature: starknet_crypto::Signature,
}

impl Signature {
    pub const fn len() -> usize {
        64
    }

    pub(crate) fn new(signature: starknet_crypto::Signature) -> Signature {
        Signature { signature }
    }

    pub(crate) fn inner(&self) -> &starknet_crypto::Signature {
        &self.signature
    }

    pub fn r(&self) -> H256 {
        H256::from(self.signature.r.to_bytes_be())
    }

    pub fn s(&self) -> H256 {
        H256::from(self.signature.s.to_bytes_be())
    }
}

impl ToBytesVec for Signature {
    fn to_vec(&self) -> Vec<u8> {
        let mut to_return = Vec::with_capacity(Signature::len());
        to_return.extend_from_slice(self.r().as_slice());
        to_return.extend_from_slice(self.s().as_slice());
        to_return
    }
}

impl<'a> TryFrom<&'a [u8]> for Signature {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        if bytes.len() != Signature::len() {
            return Err(Error::InvalidSignature);
        }

        let r_bytes = H256::try_from(&bytes[R_RANGE]).expect("Expected a valid r range");
        let s_bytes = H256::try_from(&bytes[S_RANGE]).expect("Expected a valid s range");

        let r =
            FieldElement::from_bytes_be(&r_bytes.take()).map_err(|_| Error::InvalidSignature)?;
        let s =
            FieldElement::from_bytes_be(&s_bytes.take()).map_err(|_| Error::InvalidSignature)?;

        Ok(Signature {
            signature: starknet_crypto::Signature { r, s },
        })
    }
}
