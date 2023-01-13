fn main() {
    // an array of random float numbers between 1000 and 10000
    let numbers = [10., 12., 23., 23., 16., 23., 21., 16.];

    fn standard_deviation(numbers: Vec<f64>) -> Result<f64, std::io::Error> {
        // sqrt( (sum of (x - mean)^2) / (n - 1) )
        let n = numbers.len();
        // mean of numbers
        let mean = numbers.iter().sum::<f64>() as f64 / n as f64;

        let sum_of_squares: f64 = numbers.iter().map(|x| (x - mean) * (x - mean)).sum();
        let variance = sum_of_squares / (n - 1) as f64;
        let standard_deviation = variance.sqrt();
        Ok(standard_deviation)
    }

    // standard deviation of numbers array
    let standard_deviation = standard_deviation(numbers.to_vec()).unwrap();
    println!("Standard deviation of numbers: {}", standard_deviation);
}
