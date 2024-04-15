//fn search(
//    word: &str,
//    words_mat: &[Vec<Option<String>>],
//) -> Vec<Vec<Option<String>>> {
//    words_mat
//        .iter()
//        .filter(|words|{
//            words
//                .iter()
//                .flatten()
//                .filter(|cmp|cmp==&word)
//                .next()
//                .is_some()
//        })
//        .cloned()
//        .collect()
//}

// search = xword
// table = yword

use infag::*;
fn main() {
    let table = fake_table();

    let x = table.compare("Vec<T> -> Option<T>", 2);
    println!("{x:?}");
}

fn fake_table() -> Table<String> {
    let src_max = std::env::var("LEVSMAX")
        .map(|a| a.parse().ok())
        .ok()
        .flatten()
        .unwrap_or(30);
    Table::<String>::new(
        src_max,
        2,
        vec![
            vec!["search", "Vec<T>,T -> Option<T>"],
            vec!["first", "Vec<T> -> Option<T>"],
            vec!["take", "Option<T> -> Option<T>"],
            vec!["ok_or", "Option<T>, E -> Eesult<T, E>"],
        ]
        .into_iter()
        .map(|fnc| fnc.into_iter().map(String::from).collect())
        .collect(),
    )
    .expect("manual dev table")
}
