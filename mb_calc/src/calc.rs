use num::Complex;

fn next_value(previous: f64, point: Complex) -> f64 {
    let z = previous * previous;
    let result = z + point.abs();
    return result;
}

#[test]
fn nex_value_of_null() {
    let previous = 0.0;
    let point = Complex::new(0.0,0.0);

    assert_eq!(0.0, next_value(previous,point));
}


