use core::fmt;

use super::printer::{print_canonical, print_semantic};
use crate::error::FormatError;
use crate::syntax::Module;

pub const ARTIFACT_FORMAT: &str = "hott-core/0.1";
pub const SEMANTIC_PROJECTION: &str = "hott-semantic/0.1";

/// A SHA-256 digest rendered as the 64 lowercase hexadecimal digits required
/// by the frozen Core v0.1 interchange and foundation-manifest contracts.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Sha256Hex([u8; 64]);

impl Sha256Hex {
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.0).expect("SHA-256 hexadecimal is always ASCII")
    }
}

impl fmt::Display for Sha256Hex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// The two independently versioned identities attached to one decoded module.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModuleHashes {
    artifact_sha256: Sha256Hex,
    semantic_sha256: Sha256Hex,
}

impl ModuleHashes {
    pub const fn artifact_sha256(&self) -> Sha256Hex {
        self.artifact_sha256
    }

    pub const fn semantic_sha256(&self) -> Sha256Hex {
        self.semantic_sha256
    }
}

/// Hash the exact canonical artifact and the name-free semantic projection.
///
/// This is a format identity operation, not logical validation. Any decoded
/// module that can be canonically printed can be hashed; declaration checking
/// remains a separate kernel operation.
pub fn compute_module_hashes(module: &Module) -> Result<ModuleHashes, FormatError> {
    let artifact = print_canonical(module)?;
    let artifact_sha256 = sha256_hex(&artifact);
    drop(artifact);

    let semantic = print_semantic(module)?;
    let semantic_sha256 = sha256_hex(&semantic);

    Ok(ModuleHashes {
        artifact_sha256,
        semantic_sha256,
    })
}

fn sha256_hex(input: &[u8]) -> Sha256Hex {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";

    let digest = sha256(input);
    let mut encoded = [0_u8; 64];
    for (index, byte) in digest.iter().copied().enumerate() {
        encoded[index * 2] = DIGITS[(byte >> 4) as usize];
        encoded[index * 2 + 1] = DIGITS[(byte & 0x0f) as usize];
    }
    Sha256Hex(encoded)
}

fn sha256(input: &[u8]) -> [u8; 32] {
    let mut state = [
        0x6a09e667_u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];

    let (blocks, tail) = input.as_chunks::<64>();
    for block in blocks {
        compress(&mut state, block);
    }

    let mut final_blocks = [0_u8; 128];
    final_blocks[..tail.len()].copy_from_slice(tail);
    final_blocks[tail.len()] = 0x80;

    let final_len = if tail.len() < 56 { 64 } else { 128 };
    let bit_len = (input.len() as u64).wrapping_mul(8);
    final_blocks[final_len - 8..final_len].copy_from_slice(&bit_len.to_be_bytes());

    let (blocks, remainder) = final_blocks[..final_len].as_chunks::<64>();
    debug_assert!(remainder.is_empty());
    for block in blocks {
        compress(&mut state, block);
    }

    let mut digest = [0_u8; 32];
    for (index, word) in state.iter().copied().enumerate() {
        digest[index * 4..index * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    digest
}

fn compress(state: &mut [u32; 8], block: &[u8; 64]) {
    const ROUND: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut schedule = [0_u32; 64];
    for (index, word) in schedule.iter_mut().take(16).enumerate() {
        let offset = index * 4;
        *word = u32::from_be_bytes([
            block[offset],
            block[offset + 1],
            block[offset + 2],
            block[offset + 3],
        ]);
    }
    for index in 16..64 {
        let s0 = schedule[index - 15].rotate_right(7)
            ^ schedule[index - 15].rotate_right(18)
            ^ (schedule[index - 15] >> 3);
        let s1 = schedule[index - 2].rotate_right(17)
            ^ schedule[index - 2].rotate_right(19)
            ^ (schedule[index - 2] >> 10);
        schedule[index] = schedule[index - 16]
            .wrapping_add(s0)
            .wrapping_add(schedule[index - 7])
            .wrapping_add(s1);
    }

    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = *state;

    for index in 0..64 {
        let big1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let choose = (e & f) ^ ((!e) & g);
        let temp1 = h
            .wrapping_add(big1)
            .wrapping_add(choose)
            .wrapping_add(ROUND[index])
            .wrapping_add(schedule[index]);
        let big0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let majority = (a & b) ^ (a & c) ^ (b & c);
        let temp2 = big0.wrapping_add(majority);

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

#[cfg(test)]
mod tests {
    use super::sha256_hex;

    #[test]
    fn matches_standard_sha256_vectors() {
        for (message, expected) in [
            (
                "",
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
            ),
            (
                "abc",
                "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
            ),
            (
                "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq",
                "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1",
            ),
        ] {
            assert_eq!(sha256_hex(message.as_bytes()).as_str(), expected);
        }
    }
}
