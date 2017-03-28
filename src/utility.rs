pub fn clamp(val: f64, min: f64, max:f64) -> f64{
    let mut resultValue = val;
    if val < min {
        resultValue = min;
    } else if max < val {
        resultValue = max
    }

    resultValue
}