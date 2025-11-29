use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

static mut STASH: &i32 = &10;
static mut SOME_OTHER_STASH: &i32 = &21;

fn main() {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );

    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );

    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");

    sort_works(&mut table);
    show(&table);
    assert_eq!(table["Gesualdo"][0], "Tenebrae Responsoria");

    unsafe {
        println!("STASH {}", *STASH);
        f(SOME_OTHER_STASH);
        println!("STASH {}", *STASH);
    }

    let parabola = vec![7, 5, 2, 8, 3, 4];
    let s = smallest(&parabola);
    println!("smallest {}", s);
}

fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn smallest(v: &[i32]) -> &i32 {
    let mut smallest = &v[0];
    for r in &v[1..] {
        if *r < *smallest {
            smallest = r;
        }
    }
    smallest
}
