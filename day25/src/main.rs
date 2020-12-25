fn find_loop(tgt: i64) -> usize {
    let mut v = 1;
    let mut l = 0;

    while v != tgt {
        v = transform(v, 7);
        l += 1;
    }

    l
}

fn transform(v: i64, sub_no: i64) -> i64 {
    (v * sub_no) % 20201227
}

fn main() {
    let instr = std::fs::read_to_string("input").unwrap();
    let mut iter = instr.lines().map(|l| l.parse().unwrap());

    let p_key_1: i64 = iter.next().unwrap();
    let p_key_2: i64 = iter.next().unwrap();

    let l1 = find_loop(p_key_1);
    let l2 = find_loop(p_key_2);

    let enc_key_a = (0..l2).fold(1, |acc, _| transform(acc, p_key_1));
    let enc_key_b = (0..l1).fold(1, |acc, _| transform(acc, p_key_2));

    assert_eq!(enc_key_a, enc_key_b);

    println!("Part 1: {}", enc_key_a);
}
