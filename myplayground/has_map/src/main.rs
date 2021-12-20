use std::collections::HashMap;

fn main() {
    let mut m1 = HashMap::with_capacity(10); // もしくはnew()
    m1.insert("a", 1);
    m1.insert("b", 3);
    println!("m1: {:?}", m1);
    println!("m1.len(): {}", m1.len());
    println!("m1.get(\"b\"): {:?}", m1.get("b"));
    println!("m1.get(\"c\"): {:?}", m1.get("c"));

    // "d"がそんざいするならその値への参照を、存在しないなら"d"に対して0を登録して、その参照を返す
    let d = m1.entry("d").or_insert(0);
    *d += 7;
    println!("m1.get(\"d\"): {:?}", m1.get("d"));

    let m2 = vec![("a", 1), ("b", 3)].into_iter().collect::<HashMap<_, _>>();
    println!("m2: {:?}", m2);
}
