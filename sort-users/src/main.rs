use std::cmp;

fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    let num_users = usernames.len() - 1;

    for i in 0..num_users {
        let name1 = usernames[i];
        let name2 = usernames[i + 1];

        // step through the uppercase characters in each name until
        // there is a mismatch
        let loop_count = cmp::min(name1.len(), name2.len()) - 1;

        for j in 0..loop_count {
            let char1 = name1.chars().nth(j).unwrap().to_uppercase();
            let char2 = name2.chars().nth(j).unwrap().to_uppercase();

            if char1 == char2 {
                continue;
            } else if char1 > char2 {
                usernames[i]     = name2;
                usernames[i + 1] = name1;
            }
        }


    };
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}