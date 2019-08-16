#[cfg(test)]
use crate::vec3::Vec3;

#[test]
fn test_add_vector3() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    let v3 = Vec3(5.0, 7.0, 9.0);
    assert!(v1 + v2 == v3);
}

#[test]
fn test_add_assign_vector3() {
    let mut v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    v1 += v2;
    let v3 = Vec3(5.0, 7.0, 9.0);
    assert!(v1 == v3);
}

#[test]
fn test_sub_vector3() {
    let v1 = Vec3(5.0, 7.0, 9.0);
    let v2 = Vec3(4.0, 5.0, 6.0);
    let v3 = Vec3(1.0, 2.0, 3.0);

    assert!(v1 - v2 == v3);
}

#[test]
fn test_sub_assign_vector3() {
    let mut v1 = Vec3(5.0, 7.0, 9.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    v1 -= v2;
    let v3 = Vec3(1.0, 2.0, 3.0);
    assert!(v1 == v3);
}

#[test]
fn test_mul_vector3() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    let v3 = Vec3(4.0, 10.0, 18.0);
    assert!(v1 * v2 == v3);
}

#[test]
fn test_mul_assign_vector3() {
    let mut v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    v1 *= v2;
    let v3 = Vec3(4.0, 10.0, 18.0);
    assert!(v1 == v3);
}

#[test]
fn test_scalar_mul_vector3() {
    let v1 = Vec3(1.0, 2.0, 3.0);
    let scalar: f32 = 3.0;

    let v2 = Vec3(3.0, 6.0, 9.0);
    assert!(v1 * scalar == v2);
}

#[test]
fn test_scalar_mul_assign_vector3() {
    let mut v1 = Vec3(1.0, 2.0, 3.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    v1 *= v2;
    let v3 = Vec3(4.0, 10.0, 18.0);
    assert!(v1 == v3);
}

#[test]
fn test_div_vector3() {
    let v1 = Vec3(4.0, 10.0, 18.0);
    let v2 = Vec3(4.0, 5.0, 6.0);
    let v3 = Vec3(1.0, 2.0, 3.0);

    assert!(v1 / v2 == v3);
}

#[test]
fn test_div_assign_vector3() {
    let mut v1 = Vec3(4.0, 10.0, 18.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    v1 /= v2;
    let v3 = Vec3(1.0, 2.0, 3.0);
    assert!(v1 == v3);
}

#[test]
fn test_scalar_div_vector3() {
    let v1 = Vec3(3.0, 6.0, 9.0);
    let scalar: f32 = 3.0;

    let v2 = Vec3(1.0, 2.0, 3.0);
    assert!(v1 / scalar == v2);
}

#[test]
fn test_scalar_div_assign_vector3() {
    let mut v1 = Vec3(4.0, 10.0, 18.0);
    let v2 = Vec3(4.0, 5.0, 6.0);

    v1 /= v2;
    let v3 = Vec3(1.0, 2.0, 3.0);
    assert!(v1 == v3);
}
