use poseidon_rs::Fr;
use poseidon_rs::Poseidon;

pub fn hash(v1: Fr, v2: Fr) -> Fr {
    let mut big_arr: Vec<Fr> = Vec::new();
    big_arr.push(v1);
    big_arr.push(v2);
    let poseidon = Poseidon::new();
    poseidon.hash(big_arr).unwrap()
}

pub fn generate_merkle_root(leaves: Vec<Fr>) -> Fr {
    let mut leaves = leaves.clone();

    for i in 0..(((leaves.len() as f32).log2().round()) as usize) {
        //For max_options
        for j in 0..leaves.len() {
            let step = if i == 0 { 1 } else { 2 << (i - 1) };
            // This is the modulus
            let should_skip = j - (j / 2) * 2;

            if (((j + 1) * step) < leaves.len()) && (should_skip == 0) {
                leaves[j * step] = hash(leaves[j * step], leaves[(j + 1) * step]);
            }
        }
    }
    leaves[0]
}
#[cfg(test)]
mod tests {
    use ff::PrimeField;

    use super::*;
    #[test]
    fn hash_1_2() {
        assert_eq!(
            hash(Fr::from_str("1").unwrap(), Fr::from_str("2").unwrap()).to_string(),
            "Fr(0x9811D68B946C0FC88B0B7FECCC1C35B792A732B3E072CA864DF3AEE94826684)"
        );
    }

    #[test]
    fn merkle_root_test() {
        let one: Fr = Fr::from_str("1").unwrap();
        let two: Fr = Fr::from_str("2").unwrap();
        let three: Fr = Fr::from_str("3").unwrap();

        let votes: [Fr; 32] = [
            one, two, three, two, two, three, one, two, one, two, three, two, two, three, one, two,
            one, two, three, two, two, three, one, two, one, two, three, two, two, three, one, two,
        ];

        assert_eq!(
            generate_merkle_root(votes.to_vec()).to_string(),
            "Fr(0x0d71cbc322578e133085b861a656d34b3abc2cc65ac11d24618aa53d49e5d443)"
        );
    }
}
