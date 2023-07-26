//! The `Keccak` hash functions.

use super::{bits_to_rate, keccakf::KeccakF, Hasher, KeccakState};

/// The `Keccak` hash functions defined in [`Keccak SHA3 submission`].
///
/// # Usage
///
/// ```toml
/// [dependencies]
/// fast-keccak = { version = "0.1.0", features = ["keccak"] }
/// ```
///
/// [`Keccak SHA3 submission`]: https://keccak.team/files/Keccak-submission-3.pdf
#[derive(Clone)]
pub struct Keccak {
    state: KeccakState<KeccakF>,
}

impl Keccak {
    const DELIM: u8 = 0x01;

    /// Creates  new [`Keccak`] hasher with a security level of 224 bits.
    ///
    /// [`Keccak`]: struct.Keccak.html
    #[inline(always)]
    pub fn v224() -> Keccak {
        Keccak::new(224)
    }

    /// Creates  new [`Keccak`] hasher with a security level of 256 bits.
    ///
    /// [`Keccak`]: struct.Keccak.html
    #[inline(always)]
    pub fn v256() -> Keccak {
        Keccak::new(256)
    }

    /// Creates  new [`Keccak`] hasher with a security level of 384 bits.
    ///
    /// [`Keccak`]: struct.Keccak.html
    #[inline(always)]
    pub fn v384() -> Keccak {
        Keccak::new(384)
    }

    /// Creates  new [`Keccak`] hasher with a security level of 512 bits.
    ///
    /// [`Keccak`]: struct.Keccak.html
    #[inline(always)]
    pub fn v512() -> Keccak {
        Keccak::new(512)
    }

    #[inline(always)]
    fn new(bits: usize) -> Keccak {
        Keccak {
            state: KeccakState::new(bits_to_rate(bits), Self::DELIM),
        }
    }
}

impl Hasher for Keccak {
    /// Absorb additional input. Can be called multiple times.
    ///
    /// # Example
    ///
    /// ```
    /// # use fast_keccak::{Hasher, Keccak};
    /// #
    /// # fn main() {
    /// # let mut keccak = Keccak::v256();
    /// keccak.update(b"hello");
    /// keccak.update(b" world");
    /// # }
    /// ```
    #[inline]
    fn update(&mut self, input: &[u8]) {
        self.state.update(input);
    }

    /// Pad and squeeze the state to the output.
    ///
    /// # Example
    ///
    /// ```
    /// # use fast_keccak::{Hasher, Keccak};
    /// #
    /// # fn main() {
    /// # let keccak = Keccak::v256();
    /// # let mut output = [0u8; 32];
    /// keccak.finalize(&mut output);
    /// # }
    /// #
    /// ```
    #[inline]
    fn finalize(self, output: &mut [u8]) {
        self.state.finalize(output);
    }
}
