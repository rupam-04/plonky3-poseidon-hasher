use p3_poseidon2::{Poseidon2, Poseidon2ExternalMatrixGeneral};
use p3_baby_bear::{BabyBear, DiffusionMatrixBabyBear};
use p3_symmetric::PaddingFreeSponge;
use rand::thread_rng;

type Val = BabyBear;
type Perm = Poseidon2<Val, Poseidon2ExternalMatrixGeneral, DiffusionMatrixBabyBear, 16, 7>;
type MyHash = PaddingFreeSponge<Perm, 16, 8, 8>;

fn poseidon_hash(left_node: &Val, right_node: &Val) {
    let perm = Perm::new_from_rng_128(
        Poseidon2ExternalMatrixGeneral,
        DiffusionMatrixBabyBear,
        &mut thread_rng(),
    );
    type MyHash = PaddingFreeSponge<Perm, 16, 8, 8>;
    let hash = MyHash::new(perm.clone());
}