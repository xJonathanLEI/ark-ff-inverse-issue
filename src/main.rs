use ark_ff::{
    fields::{Fp256, MontBackend, MontConfig},
    BigInteger256, Field,
};

#[derive(MontConfig)]
#[modulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
#[generator = "7"]
pub struct FrConfig;
pub type Fr = Fp256<MontBackend<FrConfig, 4>>;

fn main() {
    let one = Fr::new(BigInteger256::new([1, 0, 0, 0]));
    let num = Fr::new(BigInteger256::new([
        0x84c654a351a7c81b,
        0x15b3b4e99461d70e,
        0xf2b763af35bc5f5d,
        0x405c3191ab3883e,
    ]));
    let inverse = num.inverse().unwrap();

    dbg!(one.0.to_string(), num.0.to_string(), inverse.0.to_string());
    dbg!((num * inverse).0.to_string());

    assert_eq!(num * inverse, one);
}
