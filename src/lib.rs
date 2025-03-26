#![feature(test)]
extern crate test;
use rug::Float;
use rustc_apfloat::{
    ieee::{Double, Half, Quad, Single},
    Round,
};
use softfloat_pure;
use simple_soft_float::{F128, F16, F32, F64};
use softfloat_sys::*;
use std::ops::{Add, Div, Mul};
use test::Bencher;

mod f16 {
    use super::*;

    #[bench]
    fn add_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x1234);
            let b = test::black_box(0x7654);
            let a = F16::from_bits(a);
            let b = F16::from_bits(b);
            let d = a.add(&b, None, None);
            assert_eq!(*d.bits(), 30292);
            d
        });
    }

    #[bench]
    fn add_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x1234);
            let b = test::black_box(0x7654);
            let a = float16_t { v: a };
            let b = float16_t { v: b };
            let d = unsafe { f16_add(a, b) };
            assert_eq!(d.v, 30292);
            d
        });
    }

    #[bench]
    fn add_rug(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x1234);
            let b = test::black_box(0x7654);
            let a = half::f16::from_bits(a);
            let b = half::f16::from_bits(b);
            let a = a.to_f32();
            let b = b.to_f32();
            let a = Float::with_val(11, a);
            let b = Float::with_val(11, b);
            let d = a.add(b);
            let d = half::f16::from_f32(d.to_f32());
            assert_eq!(d.to_bits(), 30292);
            d
        });
    }

    #[bench]
    fn add_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x1234);
            let b = test::black_box(0x7654);
            let a = Half::from_bits(a as u128);
            let b = Half::from_bits(b as u128);
            let d = a.add_r(b, Round::NearestTiesToEven);
            assert_eq!(d.value.to_bits(), 30292);
            d
        });
    }

    #[bench]
    fn mul_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x1234);
            let b = test::black_box(0x7654);
            let a = F16::from_bits(a);
            let b = F16::from_bits(b);
            let d = a.mul(&b, None, None);
            assert_eq!(*d.bits(), 19688);
            d
        });
    }

    #[bench]
    fn mul_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x1234);
            let b = test::black_box(0x7654);
            let a = float16_t { v: a };
            let b = float16_t { v: b };
            let d = unsafe { f16_mul(a, b) };
            assert_eq!(d.v, 19688);
            d
        });
    }

    #[bench]
    fn mul_rug(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x1234);
            let b = test::black_box(0x7654);
            let a = half::f16::from_bits(a);
            let b = half::f16::from_bits(b);
            let a = a.to_f32();
            let b = b.to_f32();
            let a = Float::with_val(11, a);
            let b = Float::with_val(11, b);
            let d = a.mul(b);
            let d = half::f16::from_f32(d.to_f32());
            assert_eq!(d.to_bits(), 19688);
            d
        });
    }

    #[bench]
    fn mul_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x1234);
            let b = test::black_box(0x7654);
            let a = Half::from_bits(a as u128);
            let b = Half::from_bits(b as u128);
            let d = a.mul_r(b, Round::NearestTiesToEven);
            assert_eq!(d.value.to_bits(), 19688);
            d
        });
    }

    #[bench]
    fn div_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x7654);
            let b = test::black_box(0x1234);
            let a = F16::from_bits(a);
            let b = F16::from_bits(b);
            let d = a.div(&b, None, None);
            assert_eq!(*d.bits(), 31744);
            d
        });
    }

    #[bench]
    fn div_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x7654);
            let b = test::black_box(0x1234);
            let a = float16_t { v: a };
            let b = float16_t { v: b };
            let d = unsafe { f16_div(a, b) };
            assert_eq!(d.v, 31744);
            d
        });
    }

    #[bench]
    fn div_rug(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x7654);
            let b = test::black_box(0x1234);
            let a = half::f16::from_bits(a);
            let b = half::f16::from_bits(b);
            let a = a.to_f32();
            let b = b.to_f32();
            let a = Float::with_val(11, a);
            let b = Float::with_val(11, b);
            let d = a.div(b);
            let d = half::f16::from_f32(d.to_f32());
            assert_eq!(d.to_bits(), 31744);
            d
        });
    }

    #[bench]
    fn div_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x7654);
            let b = test::black_box(0x1234);
            let a = Half::from_bits(a as u128);
            let b = Half::from_bits(b as u128);
            let d = a.div_r(b, Round::NearestTiesToEven);
            assert_eq!(d.value.to_bits(), 31744);
            d
        });
    }
}

mod f32 {
    use super::*;

    #[bench]
    fn add_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667);
            let b = test::black_box(0x76543210);
            let a = F32::from_bits(a);
            let b = F32::from_bits(b);
            let d = a.add(&b, None, None);
            assert_eq!(*d.bits(), 1985229328);
            d
        });
    }

    #[bench]
    fn add_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667);
            let b = test::black_box(0x76543210);
            let a = float32_t { v: a };
            let b = float32_t { v: b };
            let d = unsafe { f32_add(a, b) };
            assert_eq!(d.v, 1985229328);
            d
        });
    }

    #[bench]
    fn add_softfloat_pure(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667);
            let b = test::black_box(0x76543210);
            let a = softfloat_pure::float32_t { v: a };
            let b = softfloat_pure::float32_t { v: b };
            let d = softfloat_pure::softfloat::f32_add(a, b, 0, 0);
            assert_eq!(d.0.v, 1985229328);
            d
        });
    }

    #[bench]
    fn add_rug(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667);
            let b = test::black_box(0x76543210);
            let a = Float::with_val(24, f32::from_bits(a));
            let b = Float::with_val(24, f32::from_bits(b));
            let d = a.add(b);
            assert_eq!(d.to_f32().to_bits(), 1985229328);
            d
        });
    }

    #[bench]
    fn add_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x12345667);
            let b = test::black_box(0x76543210);
            let a = Single::from_bits(a as u128);
            let b = Single::from_bits(b as u128);
            let d = a.add_r(b, Round::NearestTiesToEven);
            assert_eq!(d.value.to_bits(), 1985229328);
            d
        });
    }

    #[bench]
    fn mul_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667);
            let b = test::black_box(0x76543210);
            let a = F32::from_bits(a);
            let b = F32::from_bits(b);
            let d = a.mul(&b, None, None);
            assert_eq!(*d.bits(), 1226144465);
            d
        });
    }

    #[bench]
    fn mul_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667);
            let b = test::black_box(0x76543210);
            let a = float32_t { v: a };
            let b = float32_t { v: b };
            let d = unsafe { f32_mul(a, b) };
            assert_eq!(d.v, 1226144465);
            d
        });
    }

    #[bench]
    fn mul_softfloat_pure(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667);
            let b = test::black_box(0x76543210);
            let a = softfloat_pure::float32_t { v: a };
            let b = softfloat_pure::float32_t { v: b };
            let d = softfloat_pure::softfloat::f32_mul(a, b, 0, 0);
            assert_eq!(d.0.v, 1226144465);
            d
        });
    }

    #[bench]
    fn mul_rug(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667);
            let b = test::black_box(0x76543210);
            let a = Float::with_val(24, f32::from_bits(a));
            let b = Float::with_val(24, f32::from_bits(b));
            let d = a.mul(b);
            assert_eq!(d.to_f32().to_bits(), 1226144465);
            d
        });
    }

    #[bench]
    fn mul_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x12345667);
            let b = test::black_box(0x76543210);
            let a = Single::from_bits(a as u128);
            let b = Single::from_bits(b as u128);
            let d = a.mul_r(b, Round::NearestTiesToEven);
            assert_eq!(d.value.to_bits(), 1226144465);
            d
        });
    }

    #[bench]
    fn div_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x76543210);
            let b = test::black_box(0x12345667);
            let a = F32::from_bits(a);
            let b = F32::from_bits(b);
            let d = a.div(&b, None, None);
            assert_eq!(*d.bits(), 2139095040);
            d
        });
    }

    #[bench]
    fn div_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x76543210);
            let b = test::black_box(0x12345667);
            let a = float32_t { v: a };
            let b = float32_t { v: b };
            let d = unsafe { f32_div(a, b) };
            assert_eq!(d.v, 2139095040);
            d
        });
    }

    #[bench]
    fn div_softfloat_pure(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x76543210);
            let b = test::black_box(0x12345667);
            let a = softfloat_pure::float32_t { v: a };
            let b = softfloat_pure::float32_t { v: b };
            let d = softfloat_pure::softfloat::f32_div(a, b, 0, 0);
            assert_eq!(d.0.v, 2139095040);
            d
        });
    }

    #[bench]
    fn div_rug(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x76543210);
            let b = test::black_box(0x12345667);
            let a = Float::with_val(24, f32::from_bits(a));
            let b = Float::with_val(24, f32::from_bits(b));
            let d = a.div(b);
            assert_eq!(d.to_f32().to_bits(), 2139095040);
            d
        });
    }

    #[bench]
    fn div_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x76543210);
            let b = test::black_box(0x12345667);
            let a = Single::from_bits(a as u128);
            let b = Single::from_bits(b as u128);
            let d = a.div_r(b, Round::NearestTiesToEven);
            assert_eq!(d.value.to_bits(), 2139095040);
            d
        });
    }
}

mod f64 {
    use super::*;

    #[bench]
    fn add_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffff);
            let b = test::black_box(0x76543210aaaaaaaa);
            let a = F64::from_bits(a);
            let b = F64::from_bits(b);
            let d = a.add(&b, None, None);
            assert_eq!(*d.bits(), 8526495041683368618u64);
            d
        });
    }

    #[bench]
    fn add_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffff);
            let b = test::black_box(0x76543210aaaaaaaa);
            let a = float64_t { v: a };
            let b = float64_t { v: b };
            let d = unsafe { f64_add(a, b) };
            assert_eq!(d.v, 8526495041683368618u64);
            d
        });
    }

    #[bench]
    fn add_softfloat_pure(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffff);
            let b = test::black_box(0x76543210aaaaaaaa);
            let a = softfloat_pure::float64_t { v: a };
            let b = softfloat_pure::float64_t { v: b };
            let d = softfloat_pure::softfloat::f64_add(a, b, 0, 0);
            assert_eq!(d.0.v, 8526495041683368618u64);
            d
        });
    }

    #[bench]
    fn add_rug(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffff);
            let b = test::black_box(0x76543210aaaaaaaa);
            let a = Float::with_val(53, f64::from_bits(a));
            let b = Float::with_val(53, f64::from_bits(b));
            let d = a.add(b);
            assert_eq!(d.to_f64().to_bits(), 8526495041683368618u64);
            d
        });
    }

    #[bench]
    fn add_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffffu64);
            let b = test::black_box(0x76543210aaaaaaaau64);
            let a = Double::from_bits(a as u128);
            let b = Double::from_bits(b as u128);
            let d = a.add_r(b, Round::NearestTiesToEven);
            assert_eq!(d.value.to_bits(), 8526495041683368618u128);
            d
        });
    }

    #[bench]
    fn mul_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffff);
            let b = test::black_box(0x76543210aaaaaaaa);
            let a = F64::from_bits(a);
            let b = F64::from_bits(b);
            let d = a.mul(&b, None, None);
            assert_eq!(*d.bits(), 5231401168203612158u64);
            d
        });
    }

    #[bench]
    fn mul_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffff);
            let b = test::black_box(0x76543210aaaaaaaa);
            let a = float64_t { v: a };
            let b = float64_t { v: b };
            let d = unsafe { f64_mul(a, b) };
            assert_eq!(d.v, 5231401168203612158u64);
            d
        });
    }

    #[bench]
    fn mul_softfloat_pure(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffff);
            let b = test::black_box(0x76543210aaaaaaaa);
            let a = softfloat_pure::float64_t { v: a };
            let b = softfloat_pure::float64_t { v: b };
            let d = softfloat_pure::softfloat::f64_mul(a, b, 0, 0);
            assert_eq!(d.0.v, 5231401168203612158u64);
            d
        });
    }

    #[bench]
    fn mul_rug(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffff);
            let b = test::black_box(0x76543210aaaaaaaa);
            let a = Float::with_val(53, f64::from_bits(a));
            let b = Float::with_val(53, f64::from_bits(b));
            let d = a.mul(b);
            assert_eq!(d.to_f64().to_bits(), 5231401168203612158u64);
            d
        });
    }

    #[bench]
    fn mul_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffffu64);
            let b = test::black_box(0x76543210aaaaaaaau64);
            let a = Double::from_bits(a as u128);
            let b = Double::from_bits(b as u128);
            let d = a.mul_r(b, Round::NearestTiesToEven);
            assert_eq!(d.value.to_bits(), 5231401168203612158u128);
            d
        });
    }

    #[bench]
    fn div_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x76543210aaaaaaaa);
            let b = test::black_box(0x12345667ffffffff);
            let a = F64::from_bits(a);
            let b = F64::from_bits(b);
            let d = a.div(&b, None, None);
            assert_eq!(*d.bits(), 9218868437227405312u64);
            d
        });
    }

    #[bench]
    fn div_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x76543210aaaaaaaa);
            let b = test::black_box(0x12345667ffffffff);
            let a = float64_t { v: a };
            let b = float64_t { v: b };
            let d = unsafe { f64_div(a, b) };
            assert_eq!(d.v, 9218868437227405312u64);
            d
        });
    }

    #[bench]
    fn div_softfloat_pure(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x76543210aaaaaaaa);
            let b = test::black_box(0x12345667ffffffff);
            let a = softfloat_pure::float64_t { v: a };
            let b = softfloat_pure::float64_t { v: b };
            let d = softfloat_pure::softfloat::f64_div(a, b, 0, 0);
            assert_eq!(d.0.v, 9218868437227405312u64);
            d
        });
    }

    #[bench]
    fn div_rug(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x76543210aaaaaaaa);
            let b = test::black_box(0x12345667ffffffff);
            let a = Float::with_val(53, f64::from_bits(a));
            let b = Float::with_val(53, f64::from_bits(b));
            let d = a.div(b);
            assert_eq!(d.to_f64().to_bits(), 9218868437227405312u64);
            d
        });
    }

    #[bench]
    fn div_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x76543210aaaaaaaau64);
            let b = test::black_box(0x12345667ffffffffu64);
            let a = Double::from_bits(a as u128);
            let b = Double::from_bits(b as u128);
            let d = a.div_r(b, Round::NearestTiesToEven);
            assert_eq!(d.value.to_bits(), 9218868437227405312u128);
            d
        });
    }
}

mod f128 {
    use super::*;

    #[bench]
    fn add_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffffccccccccccccccccu128);
            let b = test::black_box(0x76543210aaaaaaaaccccccccccccccccu128);
            let a = F128::from_bits(a);
            let b = F128::from_bits(b);
            let d = a.add(&b, None, None);
            assert_eq!(*d.bits(), 157286071879686556347165517936193227980u128);
            d
        });
    }

    #[bench]
    fn add_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffffccccccccccccccccu128);
            let b = test::black_box(0x76543210aaaaaaaaccccccccccccccccu128);
            let a = float128_t {
                v: [a as u64, (a >> 64) as u64],
            };
            let b = float128_t {
                v: [b as u64, (b >> 64) as u64],
            };
            let d = unsafe { f128_add(a, b) };
            let mut x = 0u128;
            x |= d.v[0] as u128;
            x |= (d.v[1] as u128) << 64;
            assert_eq!(x, 157286071879686556347165517936193227980u128);
            x
        });
    }

    #[bench]
    fn add_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffffccccccccccccccccu128);
            let b = test::black_box(0x76543210aaaaaaaaccccccccccccccccu128);
            let a = Quad::from_bits(a);
            let b = Quad::from_bits(b);
            let d = a.add_r(b, Round::NearestTiesToEven);
            assert_eq!(
                d.value.to_bits(),
                157286071879686556347165517936193227980u128
            );
            d
        });
    }

    #[bench]
    fn mul_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffffccccccccccccccccu128);
            let b = test::black_box(0x76543210aaaaaaaaccccccccccccccccu128);
            let a = F128::from_bits(a);
            let b = F128::from_bits(b);
            let d = a.mul(&b, None, None);
            assert_eq!(*d.bits(), 96418871070149102153708677870054030703u128);
            d
        });
    }

    #[bench]
    fn mul_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffffccccccccccccccccu128);
            let b = test::black_box(0x76543210aaaaaaaaccccccccccccccccu128);
            let a = float128_t {
                v: [a as u64, (a >> 64) as u64],
            };
            let b = float128_t {
                v: [b as u64, (b >> 64) as u64],
            };
            let d = unsafe { f128_mul(a, b) };
            let mut x = 0u128;
            x |= d.v[0] as u128;
            x |= (d.v[1] as u128) << 64;
            assert_eq!(x, 96418871070149102153708677870054030703u128);
            x
        });
    }

    #[bench]
    fn mul_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x12345667ffffffffccccccccccccccccu128);
            let b = test::black_box(0x76543210aaaaaaaaccccccccccccccccu128);
            let a = Quad::from_bits(a);
            let b = Quad::from_bits(b);
            let d = a.mul_r(b, Round::NearestTiesToEven);
            assert_eq!(
                d.value.to_bits(),
                96418871070149102153708677870054030703u128
            );
            d
        });
    }

    #[bench]
    fn div_simple_soft_float(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x76543210aaaaaaaaccccccccccccccccu128);
            let b = test::black_box(0x12345667ffffffffccccccccccccccccu128);
            let a = F128::from_bits(a);
            let b = F128::from_bits(b);
            let d = a.div(&b, None, None);
            assert_eq!(*d.bits(), 170135991163610696904058773219554885632u128);
            d
        });
    }

    #[bench]
    fn div_softfloat_sys(b: &mut Bencher) {
        b.iter(|| {
            let a = test::black_box(0x76543210aaaaaaaaccccccccccccccccu128);
            let b = test::black_box(0x12345667ffffffffccccccccccccccccu128);
            let a = float128_t {
                v: [a as u64, (a >> 64) as u64],
            };
            let b = float128_t {
                v: [b as u64, (b >> 64) as u64],
            };
            let d = unsafe { f128_div(a, b) };
            let mut x = 0u128;
            x |= d.v[0] as u128;
            x |= (d.v[1] as u128) << 64;
            assert_eq!(x, 170135991163610696904058773219554885632u128);
            x
        });
    }

    #[bench]
    fn div_rustc_apfloat(b: &mut Bencher) {
        use rustc_apfloat::Float;
        b.iter(|| {
            let a = test::black_box(0x76543210aaaaaaaaccccccccccccccccu128);
            let b = test::black_box(0x12345667ffffffffccccccccccccccccu128);
            let a = Quad::from_bits(a as u128);
            let b = Quad::from_bits(b as u128);
            let d = a.div_r(b, Round::NearestTiesToEven);
            assert_eq!(
                d.value.to_bits(),
                170135991163610696904058773219554885632u128
            );
            d
        });
    }
}
