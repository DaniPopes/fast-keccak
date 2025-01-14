//! The `cSHAKE` extendable-output functions defined in [`SP800-185`].
//!
//! [`SP800-185`]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf

use crate::{bits_to_rate, keccakf::KeccakF, left_encode, Hasher, KeccakState, Xof};

/// The `cSHAKE` extendable-output functions defined in [`SP800-185`].
///
/// # Usage
///
/// ```toml
/// [dependencies]
/// fast-keccak = { version = "0.1.0", features = ["cshake"] }
/// ```
///
/// [`SP800-185`]: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-185.pdf
#[derive(Clone)]
pub struct CShake {
    state: KeccakState<KeccakF>,
}

impl CShake {
    const DELIM: u8 = 0x04;

    /// Creates  new [`CShake`] hasher with a security level of 128 bits.
    ///
    /// [`CShake`]: struct.CShake.html
    #[inline]
    pub fn v128(name: &[u8], custom_string: &[u8]) -> CShake {
        CShake::new(name, custom_string, 128)
    }

    /// Creates  new [`CShake`] hasher with a security level of 256 bits.
    ///
    /// [`CShake`]: struct.CShake.html
    #[inline]
    pub fn v256(name: &[u8], custom_string: &[u8]) -> CShake {
        CShake::new(name, custom_string, 256)
    }

    #[inline]
    pub(crate) fn new(name: &[u8], custom_string: &[u8], bits: usize) -> CShake {
        let rate = bits_to_rate(bits);
        // if there is no name and no customization string
        // cSHAKE is SHAKE
        if name.is_empty() && custom_string.is_empty() {
            let state = KeccakState::new(rate, 0x1f);
            return CShake { state };
        }

        let mut state = KeccakState::new(rate, Self::DELIM);
        state.update(left_encode(rate).value());
        state.update(left_encode(name.len() * 8).value());
        state.update(name);
        state.update(left_encode(custom_string.len() * 8).value());
        state.update(custom_string);
        state.fill_block();
        CShake { state }
    }

    #[inline]
    pub(crate) fn fill_block(&mut self) {
        self.state.fill_block();
    }
}

impl Hasher for CShake {
    #[inline]
    fn update(&mut self, input: &[u8]) {
        self.state.update(input);
    }

    #[inline]
    fn finalize(self, output: &mut [u8]) {
        self.state.finalize(output);
    }
}

impl Xof for CShake {
    #[inline]
    fn squeeze(&mut self, output: &mut [u8]) {
        self.state.squeeze(output);
    }
}
