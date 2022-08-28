use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
   v.push(5);
    v.push(6);
    v.push(7);
    println!("{:?}", v);
    let third: &i32 = &v[2];
    println!("{:?}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let v = vec![1,2,3,4,5];
    //let does_not_exist = &v[100];
    for i in &v {
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();
    let s = String::from("");


    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 = {}", s1);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    println!("s1 is {}", s1);




    for c in "Зд".chars() {
        println!("{}", c);
    }

    let hello = "Здравствуйте";
    let answer = &hello[0..4];

    for c in hello.chars() {
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{}", b);
    }


    let mut scoremap = HashMap::new();
    scoremap.insert(String::from("Blue"), 10);
    scoremap.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scoremap.get(&team_name);
    println!("{:?}", score);


    for (key, value) in &scoremap {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{}", field_name);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

