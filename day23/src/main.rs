#[derive(Debug)]
struct Node {
    dat: i64,
    next: usize,
    prev: usize,
}

fn nth_fwd(buf: &[Node], id: usize, n: usize) -> usize {
    let mut cur = id;
    for _ in 0..n {
        cur = buf[cur].next;
    }
    cur
}

#[allow(dead_code)]
fn nth_back(buf: &[Node], id: usize, n: usize) -> usize {
    let mut cur = id;
    for _ in 0..n {
        cur = buf[cur].prev;
    }
    cur
}

fn insert_after(buf: &mut Vec<Node>, empty_slots: &mut Vec<usize>, id: usize, v: i64) -> usize {
    let old_next = buf[id].next;

    let new_id = if let Some(f) = empty_slots.pop() {
        buf[f] = Node {
            dat: v,
            next: old_next,
            prev: id,
        };
        f
    } else {
        buf.push(Node {
            dat: v,
            next: old_next,
            prev: id,
        });
        buf.len() - 1
    };

    buf[id].next = new_id;
    buf[old_next].prev = new_id;

    new_id
}

fn remove(buf: &mut Vec<Node>, empty_slots: &mut Vec<usize>, id: usize) -> i64 {
    //if empty_slots.contains(&id) {
    //panic!("");
    //}
    let prev = buf[id].prev;
    let next = buf[id].next;
    buf[prev].next = next;
    buf[next].prev = prev;
    empty_slots.push(id);
    buf[id].dat
}

fn print_cups(cups: &[Node], mut cur: usize) {
    let start = cur;
    loop {
        cur = nth_fwd(&cups, cur, 1);
        if start == cur {
            break;
        }
        print!("{}", cups[cur].dat);
    }
    println!();
}

fn main_loop(cups: &mut Vec<Node>, empty_slots: &mut Vec<usize>, iters: usize) -> usize {
    let mut cur = 0;

    for _ in 0..iters {
        let cup = cups[cur].dat;

        let a = nth_fwd(&cups, cur, 1);
        let b = nth_fwd(&cups, cur, 2);
        let c = nth_fwd(&cups, cur, 3);

        let held = [
            remove(cups, empty_slots, a),
            remove(cups, empty_slots, b),
            remove(cups, empty_slots, c),
        ];

        let mut dest = cup - 1;
        if dest == 0 {
            dest = cups.len() as i64;
        }
        while held.contains(&dest) {
            dest -= 1;
            if dest == 0 {
                dest = cups.len() as i64;
            }
        }

        let mut leftc_i = cur;
        let mut rightc_i = cur;

        let d_pos = 'outer: loop {
            let hints = [
                leftc_i, rightc_i,
            ];

            for &h in &hints {
                if cups[h].dat == dest {
                    break 'outer h;
                }
            }

            rightc_i = (rightc_i + 1) % cups.len();
            leftc_i = ((leftc_i + cups.len()) - 1) % cups.len();
        };

        insert_after(cups, empty_slots, d_pos, held[2]);
        insert_after(cups, empty_slots, d_pos, held[1]);
        insert_after(cups, empty_slots, d_pos, held[0]);

        cur = nth_fwd(&cups, cur, 1);
    }

    cur
}

fn p1() {
    let input = "219748365";

    let mut cups = Vec::new();
    let mut empty_slots = Vec::new();
    let mut iter = input.chars().map(|c| c.to_digit(10).unwrap() as i64);

    cups.push(Node {
        dat: iter.next().unwrap(),
        next: 0,
        prev: 0,
    });

    let mut ptr = 0;
    for val in iter {
        ptr = insert_after(&mut cups, &mut empty_slots, ptr, val);
    }

    let mut cur = main_loop(&mut cups, &mut empty_slots, 100);

    loop {
        if cups[cur].dat == 1 {
            print!("Part 1: ");
            print_cups(&cups, cur);
            break;
        }

        cur = nth_fwd(&cups, cur, 1);
    }
}

fn p2() {
    let input = "219748365";
    let mut cups = Vec::new();
    let mut empty_slots = Vec::with_capacity(1_000_000);

    let mut iter = input.chars().map(|c| c.to_digit(10).unwrap() as i64);

    cups.push(Node {
        dat: iter.next().unwrap(),
        next: 0,
        prev: 0,
    });

    let mut ptr = 0;
    for val in iter.chain(10..=1_000_000) {
        ptr = insert_after(&mut cups, &mut empty_slots, ptr, val);
    }

    let mut cur = main_loop(&mut cups, &mut empty_slots, 10_000_000);

    loop {
        if cups[cur].dat == 1 {
            let a = nth_fwd(&cups, cur, 1);
            let b = nth_fwd(&cups, cur, 2);
            let a = cups[a].dat;
            let b = cups[b].dat;
            println!("Part 2: {}", a * b);
            return;
        }

        cur = nth_fwd(&cups, cur, 1);
    }
}

fn main() {
    p1();
    p2();
}
