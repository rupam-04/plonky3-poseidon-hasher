use p3_field::{ PackedField, PackedValue };
use p3_symmetric::{ CryptographicHasher, PseudoCompressionFunction };

const DIGEST_SIZE: usize = 8;

pub fn calculate_parent_hash<P, PW, H, C>(
    hash: H,
    compresser: C,
    left_child: [P; DIGEST_SIZE],
    right_child: [P; DIGEST_SIZE]
)
    -> [PW; DIGEST_SIZE]
    where
        P: PackedField,
        PW: PackedValue,
        H: CryptographicHasher<P::Scalar, [PW::Value; DIGEST_SIZE]>,
        H: CryptographicHasher<P, [PW; DIGEST_SIZE]>,
        H: Sync,
        C: PseudoCompressionFunction<[PW::Value; DIGEST_SIZE], 2>,
        C: PseudoCompressionFunction<[PW; DIGEST_SIZE], 2>,
        C: Sync
{

    // println!("left_child: {:?}", left_child);
    // println!("right_child: {:?}", right_child);

    let y = left_child.into_iter().chain(right_child.into_iter());
    let root = hash.hash_iter(y);

    let clone_root = root.clone();


    let compresssed_root = compresser.compress([clone_root, clone_root]);

    root
}

fn verify_parent_hash<P, PW, H, C>(
    hash: H,
    compresser: C,
    left_child: [P; DIGEST_SIZE],
    right_child: [P; DIGEST_SIZE],
    parent_hash: [PW; DIGEST_SIZE]
)
    -> bool
    where
        P: PackedField,
        PW: PackedValue,
        H: CryptographicHasher<P::Scalar, [PW::Value; DIGEST_SIZE]>,
        H: CryptographicHasher<P, [PW; DIGEST_SIZE]>,
        H: Sync,
        C: PseudoCompressionFunction<[PW::Value; DIGEST_SIZE], 2>,
        C: PseudoCompressionFunction<[PW; DIGEST_SIZE], 2>,
        C: Sync
{
    let calculated_parent_hash = calculate_parent_hash(hash, compresser, left_child, right_child);

    calculated_parent_hash == parent_hash
}

#[cfg(test)]
mod tests {
    use p3_baby_bear::{ BabyBear, DiffusionMatrixBabyBear };
    use p3_poseidon2::{ Poseidon2, Poseidon2ExternalMatrixGeneral };
    use p3_field::AbstractField;
    use p3_symmetric::{ PaddingFreeSponge, TruncatedPermutation };
    use rand::thread_rng;

    use super::calculate_parent_hash;

    type F = BabyBear;

    type Perm = Poseidon2<F, Poseidon2ExternalMatrixGeneral, DiffusionMatrixBabyBear, 16, 7>;
    type MyHash = PaddingFreeSponge<Perm, 16, 8, 8>;
    type MyCompress = TruncatedPermutation<Perm, 2, 8, 16>;

    #[test]
    fn test_parent_hash() {
        let poseidon2 = Perm::new_from_rng_128(
            Poseidon2ExternalMatrixGeneral,
            DiffusionMatrixBabyBear,
            &mut thread_rng()
        );

        let sponge_hash = MyHash::new(poseidon2.clone());
        let compress = MyCompress::new(poseidon2);
        let sponge_hash_clone = sponge_hash.clone();
        let compress_clone = compress.clone();

        let left_child = [F::from_canonical_u16(20); 8];
        let right_child = [F::from_canonical_u16(30); 8];

        let parent_hash = calculate_parent_hash(sponge_hash, compress, left_child, right_child);
        assert!(super::verify_parent_hash(sponge_hash_clone, compress_clone, left_child, right_child, parent_hash));
    }
}