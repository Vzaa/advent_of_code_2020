use std::collections::HashMap;

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut alg_info_map: HashMap<&str, Vec<&str>> = HashMap::new();

    let mut ings_cnt_map = HashMap::new();

    for line in instr.lines() {
        let mut iter = line.split("(contains");
        let ingredients_str = iter.next().unwrap();
        let allergans_str = iter.next().unwrap().trim_matches(|c| c == ' ' || c == ')');

        let allergans: Vec<_> = allergans_str
            .split_whitespace()
            .map(|a| a.trim_matches(','))
            .collect();
        let ingredients: Vec<_> = ingredients_str.split_whitespace().collect();

        for i in &ingredients {
            let cnt = ings_cnt_map.entry(*i).or_insert(0);
            *cnt += 1;
        }

        for a in &allergans {
            if let Some(ing_list) = alg_info_map.get_mut(*a) {
                ing_list.retain(|i| ingredients.contains(i));
            } else {
                alg_info_map.insert(a, ingredients.clone());
            }
        }
    }

    let mut known = HashMap::new();

    loop {
        let mut cand = alg_info_map.iter().filter(|(_, i)| i.len() == 1);
        if let Some((a, i)) = cand.next() {
            known.insert(i[0], *a);
        } else {
            break;
        }

        for ing_list in alg_info_map.values_mut() {
            ing_list.retain(|i| !known.contains_key(i));
        }
    }

    let safe: Vec<_> = ings_cnt_map
        .keys()
        .filter(|a| !known.contains_key(*a))
        .collect();

    let sum: usize = ings_cnt_map
        .iter()
        .filter(|(k, _)| safe.contains(k))
        .map(|(_, v)| v)
        .sum();

    println!("Part 1: {}", sum);

    let mut dang_list: Vec<_> = known.keys().cloned().collect();
    dang_list.sort_by_key(|f| known[*f]);
    let p2 = dang_list.join(",");
    println!("Part 2: {}", p2);
}
