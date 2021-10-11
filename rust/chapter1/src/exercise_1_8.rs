use std::collections::HashSet;

pub fn zero_rows_and_columns_with_zero(array: &mut Vec<Vec<i32>>) {
    let mut zero_rows = HashSet::new();
    let mut zero_columns = HashSet::new();

    for i in 0..array.len() {
        for j in 0..array[i].len() {
            if array[i][j] == 0 {
                zero_rows.insert(i);
                zero_columns.insert(j);
            }
        }
    }

    for row in zero_rows {
        for j in 0..array[row].len() {
            array[row][j] = 0;
        }
    }

    for column in zero_columns {
        for i in 0..array.len() {
            array[i][column] = 0;
        }
    }
}

#[test]
fn zero_rows_and_columns_with_zero_with1234_5607_8912_returns_1204_0000_8902() {
    let mut array = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 0, 7],
        vec![8, 9, 1, 2],
    ];

    zero_rows_and_columns_with_zero(&mut array);

    let expect =vec![
        vec![1, 2, 0, 4],
        vec![0, 0, 0, 0],
        vec![8, 9, 0, 2],
    ];

    assert!(expect[0].iter().eq(&array[0]));
    assert!(expect[1].iter().eq(&array[1]));
    assert!(expect[2].iter().eq(&array[2]));
}