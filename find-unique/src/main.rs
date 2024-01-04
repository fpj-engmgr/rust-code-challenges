fn unique(a: Vec<i32>) -> Vec<i32> {
    
    let mut out_vec = vec![];

//    println!("Entering unique with vector {:?}", a);
    if a.len() == 0 {
        return out_vec;
    }

'outer:    for i in 0..a.len() {
        if out_vec.len() == 0 {
            out_vec.push(a[i]);
//            println!(" first element to out_vec {:?} for a:{} ", out_vec, i);
        } else {
               for j in 0..out_vec.len() {
//                println!("Loop counters i: {}, j: {}",i, j);
//                println!("Values a[i]: {}, out_vec[j]: {}, length of out_vec: {}",a[i], out_vec[j], out_vec.len());
                if a[i] != out_vec[j] {
                    if j == (out_vec.len() - 1) {
                        out_vec.push(a[i]);
                        continue;
                    }

                } else {
//                    println!("We have a[i] ({}) already", a[i]);
                    continue 'outer;
                }
            }
        }
    };
    return out_vec;
}

// advanced 1: use generic types
// fn unique(a: Vec<T>) -> Vec<T> {
//     todo!();
// }

// advanced 2: keep items in order
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

// advanced 3: use iterators
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

fn main() {
    let input = vec![2, 1, 1];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}


#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 5, 2];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}


#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let expected_output = vec![1, 5, 2];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x,y| x.partial_cmp(y).unwrap());
    let expected_output = vec![1, 2, 5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}
