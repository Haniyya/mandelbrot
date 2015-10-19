use num::Complex;

struct Pixel {
    point: Complex,
    iter: i32,
}

fn next_value(previous: f64, point: &Complex) -> f64 {
    let z = previous * previous;
    let result = z + point.abs();
    return result;
}

fn bounded(point: Complex) -> i32 {
    let mut c = 0.0;
    let mut i = 0;
    let point = point;

    while c <= 1000.0 && i <= 20 {
        i += 1;
        c = next_value(c, &point);
    }

    if c > 1000.0 {
        return i;
    } else {
        return -1;
    }
}

#[test]
fn nex_value_of_null() {
    let previous = 0.0;
    let point = Complex::new(0.0,0.0);

    assert_eq!(0.0, next_value(previous,&point));
}

#[test]
fn zero_is_bounded() {
    let point = Complex::new(0.0,0.0);

    assert_eq!(-1,bounded(point));
}

#[test]
fn unbound_is_unbound() {
    let point = Complex::new(-1.0,1.0);

    assert!(bounded(point) != -1);
}
