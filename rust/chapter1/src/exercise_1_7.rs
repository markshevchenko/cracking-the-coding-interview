pub fn rotate90(array: &mut Vec<Vec<u32>>) {
    let len = array.len();
    if len == 0 || len == 1 {
        return;
    }

    for i in 0..len/2 {
        for j in 0..(len + 1)/2 {
            swap4(array, i, j);
        }
    }
}

#[test]
fn rotate90_with1234_returns3142() {
    let mut image = vec![
        vec![1u32, 2u32],
        vec![3u32, 4u32],
    ];

    rotate90(&mut image);

    assert_eq!(3, image[0][0]);
    assert_eq!(1, image[0][1]);

    assert_eq!(4, image[1][0]);
    assert_eq!(2, image[1][1]);
}

#[test]
fn rotate90_with123456789_returns741852963() {
    let mut image = vec![
        vec![1u32, 2u32, 3u32],
        vec![4u32, 5u32, 6u32],
        vec![7u32, 8u32, 9u32],
    ];

    rotate90(&mut image);

    assert_eq!(7, image[0][0]);
    assert_eq!(4, image[0][1]);
    assert_eq!(1, image[0][2]);

    assert_eq!(8, image[1][0]);
    assert_eq!(5, image[1][1]);
    assert_eq!(2, image[1][2]);

    assert_eq!(9, image[2][0]);
    assert_eq!(6, image[2][1]);
    assert_eq!(3, image[2][2]);
}

fn swap4(array: &mut Vec<Vec<u32>>, i: usize, j: usize) {
    let len = array.len();
    let t = array[i][j];
    array[i][j] = array[len - 1 - j][i];
    array[len - 1 - j][i] = array[len - 1 - i][len - 1 - j];
    array[len - 1 - i][len - 1 - j] = array[j][len - 1 - i];
    array[j][len - 1 - i] = t;
}

#[test]
fn swap4_with_1x3xxx7x9_i0_j0_returns_7x1xxx9x3() {
    let x = 1000;

    let mut image = vec![
        vec![1u32, x, 3u32],
        vec![x, x, x],
        vec![7u32, x, 9u32],
    ];

    swap4(&mut image, 0, 0);

    assert_eq!(7, image[0][0]);
    assert_eq!(1, image[0][2]);

    assert_eq!(9, image[2][0]);
    assert_eq!(3, image[2][2]);
}