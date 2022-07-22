fn main() {
    let my_numbers = vec![2.0, 4.0, 5.0, 6.0];
    println!("Median is {:?}", median(my_numbers));
    let my_numbers = vec![2.0, 4.0, 6.0];
    println!("Median is {:?}", median(my_numbers));
}

fn median(mut a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None
    }

    a.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let n_elements = a.len();
    let middle = n_elements / 2;

    let median = match n_elements % 2 {
        0 => a[middle] + a [middle - 1] / 2.0,
        _ => a[middle],
    };

    Some(median)
}