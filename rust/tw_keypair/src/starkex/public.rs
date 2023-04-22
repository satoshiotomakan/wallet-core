// Copyright © 2017-2023 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

use crate::starkex::field_element_from_bytes_be;
use crate::starkex::signature::Signature;
use crate::traits::VerifyingKeyTrait;
use crate::Error;
use starknet_ff::FieldElement;
use starknet_signers::VerifyingKey;
use tw_encoding::hex;
use tw_hash::H256;
use tw_utils::traits::ToBytesVec;
use tw_utils::try_or_false;

pub struct PublicKey {
    public: VerifyingKey,
}

impl PublicKey {
    pub(crate) fn from_scalar(public: FieldElement) -> PublicKey {
        PublicKey {
            public: VerifyingKey::from_scalar(public),
        }
    }
}

impl VerifyingKeyTrait for PublicKey {
    type SigningHash = Vec<u8>;
    type VerifySignature = Signature;

    fn verify(&self, signature: Self::VerifySignature, hash: Self::SigningHash) -> bool {
        let hash = try_or_false!(field_element_from_bytes_be(&hash));
        self.public
            .verify(&hash, signature.inner())
            .unwrap_or_default()
    }
}

impl<'a> TryFrom<&'a [u8]> for PublicKey {
    type Error = Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let bytes = H256::try_from(bytes).map_err(|_| Error::InvalidPublicKey)?;
        let public_scalar =
            FieldElement::from_bytes_be(&bytes.take()).map_err(|_| Error::InvalidPublicKey)?;
        Ok(PublicKey::from_scalar(public_scalar))
    }
}

impl From<&'static str> for PublicKey {
    fn from(hex: &'static str) -> Self {
        let bytes = hex::decode(hex).expect("Expected a valid Public Key hex");
        PublicKey::try_from(bytes.as_slice()).expect("Expected a valid Public Key")
    }
}

impl ToBytesVec for PublicKey {
    fn to_vec(&self) -> Vec<u8> {
        self.public.scalar().to_bytes_be().to_vec()
    }
}
