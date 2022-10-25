mod structs;
use structs::*;

use std::time::Instant;

use serde_json::{Result, Value};

fn main() {
    let contents = std::fs::read_to_string("files/ingredients.json")
        .expect("Should have been able to read the file");

    let p: Value = serde_json::from_str(contents.as_str()).unwrap();

    let mut items: Vec<Item> = vec![];
    match p {
        Value::Object(o) => {
            for (_, value) in o {
                let item: Item = serde_json::from_value(value).unwrap();
                items.push(item);
            }
        }
        _ => {}
    }
    items = items.into_iter().take(22).collect::<Vec<Item>>();
    let lim = items.len();
    let mut best = 0;
    let start_time = Instant::now();
    for a in 0..lim {
        for b in 0..lim {
            for c in 0..lim {
                for d in 0..lim {
                    for e in 0..lim {
                        for f in 0..lim {
                            let (durability, id) = evaluateItem(&[&items[a], &items[b], &items[c], &items[d], &items[e], &items[f]], 
                                &"poison".to_string());
                            if id > best {
                                println!("new best {id}");
                                println!("{a} {b} {c} \n");
                                best = id;
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("best {:?}", best);
    println!("{} s", start_time.elapsed().as_millis() as f32 * 0.001);
}

fn evaluateItem(ingredients: &[&Item; 6], desired_ID: &String) -> (i32, i32) {

    let mut totalMinDurability = 0;

    let mut eff = [
        [100, 100],
        [100, 100],
        [100, 100]
    ];

    let touching_table = [
            [   [[false, true], [true, false], [false, false]], 
                [[true, false], [false, true], [false, false]]], 
            [   [[true, false], [false, true], [true, false]], 
                [[false, true], [true, false], [false, true]]], 
            [   [[false, false], [true, false], [false, true]], 
                [[false, false], [false, true], [true, false]]]
        ];

    for (n, item) in ingredients.iter().enumerate() {
        let (y, x) = (n / 2, n % 2);
        
        totalMinDurability += item.itemIDs.dura;

        // above
        if item.posMods.above != 0 {
            for k in 0..y {
                eff[k][x] += item.posMods.above;
            }
        }
        // under
        if item.posMods.under != 0 {
            for k in (y+1)..3 {
                eff[k][x] += item.posMods.under;
            }
        }
        // left
        if item.posMods.left != 0 {
            for k in 0..x {
                eff[y][k] += item.posMods.left;
            }
        }
        // right
        if item.posMods.left != 0 {
            for k in (x+1)..2 {
                eff[y][k] += item.posMods.right;
            }
        }
        // touching and non touching
        if item.posMods.touching != 0 || item.posMods.touching != 0 {
            let touching = touching_table[y][x];
            for i in 0..2 {
                for j in 0..1 {
                    if touching[i][j] {
                        eff[i][j] += item.posMods.touching;
                    }
                    else {
                        eff[i][j] += item.posMods.notTouching;
                    }
                }
            }
        }
    }

    let mut totalID = 0;
    for (n, item) in ingredients.iter().enumerate() {
        let (y, x) = (n / 2, n % 2);

        totalID += match &item.ids[desired_ID]["maximum"] {
            Value::Number(val) => {
                (val.as_f64().unwrap() * (eff[y][x] as f64) * 0.01) as i32
            },
            _ => {
                // println!("no '{}' for {:?}", desired_ID, item.displayName);
                0
            }
        }
    }

    (totalMinDurability, totalID)
    /* return {
        "minDurability": totalMinDurability,
        "ID": totalID
    }
    */
}