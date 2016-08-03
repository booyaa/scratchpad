use std::collections::HashMap;

fn main() {
    // let mut foo: HashMap<u32, Vec<String>> = HashMap::new();
    //
    // foo.insert(1,
    //            vec!["one".to_string(), "two".to_string(), "three".to_string()]);
    // foo.insert(2, vec!["a".to_string()]);
    //
    //
    // println!("{:?}", foo);
    //
    // foo.entry(2).or_insert(Vec::new()).push("b".to_string());
    //
    // println!("foo: {:?}", foo);
    //
    // println!("foo.get(1) {:?}", foo.get(&1_u32));
    //
    // println!("grades(2) {:?}", grades(2));

    // println!("grade_keys {:?}", grade_keys());

    // let mut unsorted = vec![3, 1, 2];
    // println!("unsorted: {:?}", unsorted);
    // unsorted.sort();
    // println!("sorted: {:?}", unsorted);

    let mut map = HashMap::new();
    map.insert(1, "foo");
    map.insert(2, "bar");

    for key in map.keys() {
        // in a loop, the iterator will handle option type automatically.
        println!("{}", key);
    }

    let mut iter = map.keys();
    println!("{:?}", iter.next()); // Some(value)

    let vec = map.keys().cloned().into_iter().collect::<Vec<u32>>();
    println!("{:?}", vec);
}

#[allow(dead_code)]
fn grades(grade: u32) -> Vec<String> {
    let mut bar: HashMap<u32, Vec<String>> = HashMap::new();
    bar.insert(1,
               vec!["one".to_string(), "two".to_string(), "three".to_string()]);
    bar.insert(2, vec!["a".to_string()]);

    // assert_eq!(m.iter().collect::<Vec<(&String, &u32)>>(), vec![]);
    // .collect::<Vec<_>>();
    bar.get(&grade).unwrap().to_vec()
}

#[allow(dead_code)]
fn grade_keys() -> Vec<u32> {
    let mut bar: HashMap<u32, Vec<&str>> = HashMap::new();
    bar.insert(3, vec!["b"]);
    bar.insert(1, vec!["a", "c"]);
    bar.insert(2, vec!["d"]);
    // bar.insert(1,
    //            vec!["one".to_string(), "two".to_string(), "three".to_string()]);
    // bar.insert(2, vec!["a".to_string()]);

    // let mut buzz = bar.keys().cloned().collect::<Vec<u32>>();//.sort_by(|a, b| b.cmp(a));
    let mut buzz = bar.keys().cloned().collect::<Vec<u32>>();
    buzz.iter().cloned().collect::<Vec<u32>>();
    buzz.sort_by(|a, b| b.cmp(a));


    buzz
}
