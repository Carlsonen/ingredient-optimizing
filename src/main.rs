mod structs;
use structs::*;

use std::time::Instant;
use serde_json::Value;
use terminal_menu::{menu, label, button, list, scroll, run, mut_menu, numeric, string};

fn main() {
    let all_stats = [
        "hprPct",
        "mr",
        "sdPct",
        "mdPct",
        "ls",
        "ms",
        "xpb",
        "lb",
        "ref",
        "thorns",
        "expd",
        "spd",
        "atkTier",
        "poison",
        "hpBonus",
        "spRegen",
        "eSteal",
        "hprRaw",
        "sdRaw",
        "mdRaw",
        "fDamPct", "wDamPct", "aDamPct", "tDamPct", "eDamPct",
        "fDefPct", "wDefPct", "aDefPct", "tDefPct", "eDefPct",
        "spPct1", "spRaw1",
        "spPct2", "spRaw2",
        "spPct3", "spRaw3",
        "spPct4", "spRaw4",
        "rSdRaw",
        "sprint",
        "sprintReg",
        "jh",
        "lq",
        "gXp",
        "gSpd"
    ];
    
    let menu = menu(vec![
        label("oscar91's crafter"),

        label("- - - - - - - - -"),

        string("Stat", "", false),

        list("Recipe", vec!["Helmet", "Chestplate", "Leggings", "Boots", "Ring", "Bracelet", "Necklace"]),

        scroll("Level range", vec!["3-5", "5-7", "1-3", "10-13", "13-15", "15-17", "17-19", "20-23", "23-25", "7-9", "25-27", "27-29", "30-33", "33-35", "35-37", "37-39", "40-43", "45-47", "47-49", "43-45", "50-53", "53-55", "55-57", "57-59", "60-63", "65-67", "63-65", "67-69", "70-73", "73-75", "77-79", "75-77", "80-83", "83-85", "85-87", "87-89", "93-95", "95-97", "90-93", "97-99", "100-103", "103-105"]),

        numeric("Min durability",
            100.,
            Some(5.),
            Some(0.),
            Some(1000.)
        ),

        button("go!"),

        button("exit")
    ]);
    
    loop {
        run(&menu);
        let mm = mut_menu(&menu);
        if mm.selected_item_name() == "exit" {
            std::process::exit(0x0045);
        }
        let stat = mm.selection_value("Stat"); 
        let recipe = mm.selection_value("Recipe");
        let lvl_range = mm.selection_value("Level range");
        let min_dur = mm.numeric_value("Min durability") as i32;
        if all_stats.contains(&stat) {
            incremental(
                stat, 
                recipe,
                lvl_range,
                min_dur,
                -20000
            );
        }
        else {
            println!("Stat not available");
        }
        use std::io::{stdin, stdout, Write};
        let mut s=String::new();
        print!("Press enter to continue: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
    }
}
fn from_json_file(filepath: &str) -> Result<Value, serde_json::Error> {
    let contents = std::fs::read_to_string(filepath)
        .expect("Should have been able to read the file");
    serde_json::from_str(contents.as_str())
}

fn incremental(desired: &str, recipe_type: &str, recipe_level: &str, min_dur: i32, min_item_dur: i32) {
    let ings_obj: Value = from_json_file("files/ingredients.json").unwrap();
    let reci_obj: Value = from_json_file("files/recipes.json").unwrap();

    let recipe = &reci_obj[format!("{recipe_type}-{recipe_level}")];
    let required_skill = recipe["skill"].as_str().unwrap();
    let mut durability = (recipe["durability"]["minimum"].as_f64().unwrap() * 1.4) as i32;

    let mut all_items: Vec<Item> = vec![];
    match ings_obj {
        Value::Object(o) => {
            for (_, value) in o {
                let item: Item = serde_json::from_value(value).unwrap();
                all_items.push(item);
            }
        }
        _ => {
            println!("wtf duude");
        }
    }
    
    let item_max_lvl: i32 = recipe_level.split("-").collect::<Vec<&str>>()[1].parse().unwrap();

    let items: Vec<Item> = all_items.into_iter()
        .filter(
            |item|
            item.skills.contains(&required_skill.to_string())   // required filters
            && item.lvl <= item_max_lvl                         // ----------------
        )
        .filter(
            |item| 
            match &item.ids[desired]["maximum"] {
                Value::Number(n) => n.as_i64().unwrap() != 0,
                _ => false
            } 
            ||
            item.posMods.has_some()
            ||
            item.itemIDs.dura > 0                               // only useful if positive
        )
        .collect::<Vec<Item>>();
    
    let mut found_one = false;
    let mut best = 0;
    let mut best_ids = [0; 6];
    let start_time = Instant::now();
    let desireds = items.iter().map(|i| get_desired(&i, desired)).collect::<Vec<f32>>();
    let mut eff = [[100; 2]; 3];
    for (n, (a, da)) in items.iter().zip(&desireds).enumerate() {
        let a_eff = get_eff(0, 0, a);
        add_eff(&mut eff, &a_eff);
        durability += a.itemIDs.dura;
        for (b, db) in items.iter().zip(&desireds) {
            let b_eff = get_eff(0, 1, b);
            add_eff(&mut eff, &b_eff);
            durability += b.itemIDs.dura;
            for (c, dc) in items.iter().zip(&desireds) {
                let c_eff = get_eff(1, 0, c);
                add_eff(&mut eff, &c_eff);
                durability += c.itemIDs.dura;
                for (d, dd) in items.iter().zip(&desireds) {
                    let d_eff = get_eff(1, 1, d);
                    durability += d.itemIDs.dura;
                    add_eff(&mut eff, &d_eff);
                    for (e, de) in items.iter().zip(&desireds) {
                        let e_eff = get_eff(2, 0, e);
                        add_eff(&mut eff, &e_eff);
                        durability += e.itemIDs.dura;
                        for (f, df) in items.iter().zip(&desireds) {
                            let f_eff = get_eff(2, 1, f);
                            add_eff(&mut eff, &f_eff);
                            durability += f.itemIDs.dura;

                            let ds = [*da, *db, *dc, *dd, *de, *df];
                            let score = calculate_score(&eff, ds);
                            if score > best && durability > min_dur {
                                best = score;
                                best_ids = [a.id, b.id, c.id, d.id, e.id, f.id];
                                found_one = true;
                            }
                            durability -= f.itemIDs.dura;
                            sub_eff(&mut eff, &f_eff);
                        }
                        durability -= e.itemIDs.dura;
                        sub_eff(&mut eff, &e_eff);
                    }
                    durability -= d.itemIDs.dura;
                    sub_eff(&mut eff, &d_eff);
                }
                durability -= c.itemIDs.dura;
                sub_eff(&mut eff, &c_eff);
            }
            durability -= b.itemIDs.dura;
            sub_eff(&mut eff, &b_eff);
        }
        durability -= a.itemIDs.dura;
        sub_eff(&mut eff, &a_eff);
        println!("{}/{}", n + 1, items.len());
    }
    if found_one {
        let link = get_link(best_ids, recipe["id"].as_i64().unwrap() as i32);
        println!("{}", link);
    }
    else {
        println!("Could not find a valid combination of items");
    }
    println!("{} s", start_time.elapsed().as_millis() as f32 * 0.001);
}

fn get_link(ids: [i32; 6], recipe_id: i32) -> String {
    let link = format!("https://hppeng-wynn.github.io/crafter/#1{}{}{}{}{}{}{}91",
        b64(ids[0]),
        b64(ids[1]),
        b64(ids[2]),
        b64(ids[3]),
        b64(ids[4]),
        b64(ids[5]),
        b64(recipe_id),
    );
    return link;
}

fn b64(i: i32) -> String {
    let i = i as usize;
    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+-".to_string();

    return format!("{}{}", digits.chars().nth((i >> 6) % 64).unwrap(), digits.chars().nth(i % 64).unwrap());
}

fn calculate_score(eff: &[[i32; 2]; 3], ds: [f32; 6]) -> i32 {
    let mut total_id = 0;
    for n in 0..6 {
        let (y, x) = (n / 2, n % 2);
        total_id += (ds[n] * (eff[y][x] as f32) * 0.01) as i32;
    }
    total_id
}

fn get_eff(y: usize, x: usize, item: &Item) -> [[i32; 2]; 3] {
    let mut eff = [[0; 2]; 3];
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
    if item.posMods.right != 0 {
        for k in (x+1)..2 {
            eff[y][k] += item.posMods.right;
        }
    }
    // touching and non touching
    if item.posMods.touching != 0 || item.posMods.touching != 0 {
        let touching = TOUCHING_TABLE[y][x];
        for i in 0..3 {
            for j in 0..2 {
                if touching[i][j] {
                    eff[i][j] += item.posMods.touching;
                }
                else if !(i == y && j == x) {
                    eff[i][j] += item.posMods.notTouching;
                }
            }
        }
    }
    eff
}
fn add_eff(eff: &mut [[i32; 2]; 3], xss: &[[i32; 2]; 3]) {
    for y in 0..3 {
        for x in 0..2 {
            eff[y][x] += xss[y][x];
        }
    }
}
fn sub_eff(eff: &mut [[i32; 2]; 3], xss: &[[i32; 2]; 3]) {
    for y in 0..3 {
        for x in 0..2 {
            eff[y][x] -= xss[y][x];
        }
    }
}


fn get_desired(item: &Item, desired: &str) -> f32 {
    match &item.ids[desired]["maximum"] {
        Value::Number(n) => {
            n.as_f64().unwrap() as f32
        }
        _ => {0.0}
    }
}

const TOUCHING_TABLE: [[[[bool; 2]; 3]; 2]; 3] = [
            [   [[false, true], [true, false], [false, false]], 
                [[true, false], [false, true], [false, false]]], 
            [   [[true, false], [false, true], [true, false]], 
                [[false, true], [true, false], [false, true]]], 
            [   [[false, false], [true, false], [false, true]], 
                [[false, false], [false, true], [true, false]]]
        ];
