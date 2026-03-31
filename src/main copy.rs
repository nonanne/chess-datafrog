// src/main.rs

use datafrog::Iteration;

fn main() {
    let mut iteration = Iteration::new();

    // ========= 事実 (Facts) =========
    let parents = vec![
        ("Abe", "Homer"),
        ("Mona", "Homer"),
        ("Homer", "Bart"),
        ("Marge", "Bart"),
        ("Homer", "Lisa"),
        ("Marge", "Lisa"),
    ];

    let parent_var = iteration.variable::<(&'static str, &'static str)>("parent");
    parent_var.insert(parents.into());

    // ========= ルール (Rule) =========
    let parent_keyed_by_child = iteration.variable::<(&'static str, &'static str)>("parent_keyed_by_child");
    parent_keyed_by_child.from_map(&parent_var, |&(a, b)| (b, a));

    let grandparent_var = iteration.variable::<(&'static str, &'static str)>("grandparent");

    grandparent_var.from_join(&parent_keyed_by_child, &parent_var, |&_b, &a, &c| {
        // ★★★ ここが修正点です ★★★
        // Some() を外して、タプル(a, c)を直接返します。
        (a, c)
    });

    // ========= 結果 (Result) =========
    let result = grandparent_var.complete();

    println!("Found grandparents:");
    for (grandparent, grandchild) in result.elements {
        println!("  - {} is a grandparent of {}", grandparent, grandchild);
    }
}