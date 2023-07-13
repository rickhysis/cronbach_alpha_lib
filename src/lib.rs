fn mean(data: &[i32]) -> Option<f32> {
    let sum = data.iter().sum::<i32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &[i32]) -> Option<f32> {
    match (mean(data), data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>() / ((count as f32) - 1.0);

            Some(variance)
        },
        _ => None
    }
}

fn transpose_data(arr: &[Vec<i32>]) -> f32 {
    let m = arr.len();
    let n = arr[0].len();
    
    let mut transposed: Vec<Vec<i32>> = vec![vec![0; m]; n];
    let mut sum_all_tranposed: Vec<i32> = vec![];

    for i in 0..m {
        for j in 0..n {
            transposed[j][i] = arr[i][j];
        }
    }

    for t in transposed.iter(){
        sum_all_tranposed.push(t.iter().sum::<i32>())
    }

    let std_dev = std_deviation(&sum_all_tranposed);
    if std_dev.is_some() {
        std_dev.unwrap()
    }else{
        0.0
    }

}

pub fn reliable(items:  &Vec<Vec<i32>>) -> f32 {
    let mut items_variance: Vec<f32> = vec![];
    let transposed = transpose_data(&items); // transposed items 
    let k: f32 = items.len() as f32 / (items.len() as f32 - 1.0); // has many variant
    let mut result: f32;

    for i in items.iter() { // 👍 questionare.iter() turn the array into a simple iterator
        let data_std_deviation = std_deviation(&i);
        items_variance.push(data_std_deviation.unwrap());
    }

    result = items_variance.iter().sum::<f32>(); // sum all varians

    result = 1.0 - (&result / transposed); //  1-(∑ σ_b^2)/(σ_t^2 )
    
    println!("k: {}", items.len());
    println!("k/k-1: {}", k);
    println!("1-(∑ σ_b^2)/(σ_t^2 ): {}", result);

    result = format!("{:.3}", k * &result).parse().unwrap();
    println!("cronbach alpha value: {}", &result);

    return result; // cronbach alpha value [ k/(k-1) * 1-(∑ σ_b^2)/(σ_t^2 )]
}

pub fn is_reliable(reliable_value:  f32) -> bool {
    if reliable_value > 0.6 {
        println!("is reliable");
    }else{
        println!("is not reliable");
    }

    return reliable_value > 0.6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reliable() {
        // likert scale 10 items 15 * 10
        let data = vec![
            vec![1, 2, 1, 2, 3, 2, 1, 2, 3, 2], // Q1 = question 1
            vec![3, 2, 3, 1, 3, 3, 1, 3, 3, 2], // Q2 = question 2
            vec![3, 2, 3, 2, 3, 3, 1, 2, 3, 2], // Q3 = question 3
            vec![3, 2, 3, 2, 4, 3, 1, 2, 4, 3], // Q4 = question 4
            vec![2, 3, 3, 3, 4, 3, 1, 2, 2, 1], // Q5 = question 5
            vec![2, 3, 3, 3, 4, 4, 1, 2, 2, 1], // Q6 = question 6
            vec![2, 3, 3, 3, 4, 4, 1, 2, 2, 2], // Q7 = question 7
            vec![2, 3, 3, 4, 3, 4, 1, 2, 2, 2], // Q8 = question 8
            vec![1, 4, 3, 4, 3, 1, 1, 2, 2, 4], // Q9 = question 9
            vec![1, 4, 4, 4, 3, 2, 1, 2, 2, 4], // Q10 = question 10
            vec![1, 4, 4, 4, 3, 2, 1, 2, 2, 3], // Q11 = question 11
            vec![3, 4, 3, 1, 3, 2, 1, 2, 3, 2], // Q12 = question 12
            vec![4, 3, 4, 1, 3, 2, 1, 1, 3, 2], // Q13 = question 13
            vec![3, 3, 2, 1, 3, 3, 1, 1, 3, 2], // Q14 = question 14
            vec![4, 3, 2, 1, 3, 3, 1, 1, 3, 4]  // Q15 = question 15
        ];
        let result: f32 = reliable(&data);
        assert_eq!(result, 0.893);
        assert_eq!(is_reliable(result), true);
    }
}
