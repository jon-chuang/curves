use ark_ff::{Field, PrimeField};
use ark_serialize::{buffer_bit_byte_size, CanonicalSerialize};
use ark_std::rand::Rng;
use ark_std::test_rng;

use crate::*;

use ark_algebra_test_templates::fields::*;

#[test]
fn test_fr() {
    let mut rng = test_rng();
    let a: Fr = rng.gen();
    let b: Fr = rng.gen();
    field_test(a, b);
    sqrt_field_test(a);
    primefield_test::<Fr>();
}

#[test]
fn test_fq() {
    let mut rng = test_rng();
    let a: Fq = rng.gen();
    let b: Fq = rng.gen();
    field_test(a, b);
    primefield_test::<Fq>();
    sqrt_field_test(a);

    let byte_size = a.serialized_size();
    let (_, buffer_size) = buffer_bit_byte_size(Fq::size_in_bits());
    assert_eq!(byte_size, buffer_size);
    field_serialization_test::<Fq>(byte_size);
}

#[test]
fn test_fq3() {
    let mut rng = test_rng();
    let a: Fq3 = rng.gen();
    let b: Fq3 = rng.gen();
    field_test(a, b);
    sqrt_field_test(a);
    frobenius_test::<Fq3, _>(Fq::characteristic(), 13);
}

#[test]
fn test_fq6() {
    let mut rng = test_rng();
    let a: Fq6 = rng.gen();
    let b: Fq6 = rng.gen();
    field_test(a, b);
    frobenius_test::<Fq6, _>(Fq::characteristic(), 13);
}
