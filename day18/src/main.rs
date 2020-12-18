#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Token {
    Num(u64),
    Add,
    Mul,
    ParenL,
    ParenR,
}

fn tokenize(txt: &str) -> Vec<Token> {
    txt.chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            c if c.is_numeric() => Token::Num(c.to_digit(10).unwrap() as u64),
            '+' => Token::Add,
            '*' => Token::Mul,
            '(' => Token::ParenL,
            ')' => Token::ParenR,
            _ => panic!(),
        })
        .collect()
}

// Input is reversed
fn eval1(tokens: &[Token], depth: usize) -> (u64, usize) {
    let t = tokens[0];

    let (l, llen) = match t {
        Token::Num(n) => (n, 1),
        Token::ParenR => {
            let (r, rlen) = eval1(&tokens[1..], depth + 1);
            (r, rlen + 1)
        }
        _ => panic!("invalid"),
    };

    match tokens.get(llen) {
        None => (l, llen + 1),
        Some(Token::Add) => {
            let (r, rlen) = eval1(&tokens[(llen + 1)..], depth);
            (l + r, llen + rlen + 1)
        }
        Some(Token::Mul) => {
            let (r, rlen) = eval1(&tokens[(llen + 1)..], depth);
            (l * r, llen + rlen + 1)
        }
        Some(Token::ParenL) => (l, llen + 1),
        _ => panic!("invalid"),
    }
}

fn eval2(tokens: &[Token]) -> u64 {
    // Look for * and + to divide the tree at lowest depth
    let mut depth = 0;
    let mut mul_pos = None;
    let mut add_pos = None;

    for (i, t) in tokens.iter().enumerate() {
        if *t == Token::ParenL {
            depth += 1;
        } else if *t == Token::ParenR {
            depth -= 1;
        } else if *t == Token::Mul && depth == 0 {
            mul_pos = Some(i);
            break;
        } else if *t == Token::Add && depth == 0 {
            add_pos = Some(i);
        }
    }

    if let Some(d) = mul_pos {
        // Split at d and evaluate both sides
        let (left, right) = (&tokens[0..d], &tokens[(d + 1)..]);
        let l = eval2(left);
        let r = eval2(right);
        l * r
    } else if let Some(d) = add_pos {
        // Split at d and evaluate both sides
        let (left, right) = (&tokens[0..d], &tokens[(d + 1)..]);
        let l = eval2(left);
        let r = eval2(right);
        l + r
    } else if tokens.len() == 1 {
        // Terminate singular Num, otherwise invalid input
        if let Token::Num(n) = tokens[0] {
            n
        } else {
            panic!("invalid");
        }
    } else {
        // No operators and not a singular Num, has to be a parenthesis node. Evaluate to inner
        assert_eq!(tokens[0], Token::ParenL);
        assert_eq!(*tokens.last().unwrap(), Token::ParenR);
        eval2(&tokens[1..tokens.len() - 1])
    }
}

fn p1() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut sum = 0;

    for line in instr.lines() {
        let mut tokens = tokenize(line);
        tokens.reverse();
        let e = eval1(&tokens, 0);
        sum += e.0;
    }
    println!("Part 1: {}", sum);
}

fn p2() {
    let instr = std::fs::read_to_string("input").unwrap();

    let mut sum = 0;

    for line in instr.lines() {
        let tokens = tokenize(line);
        let e = eval2(&tokens);
        sum += e;
    }
    println!("Part 2: {}", sum);
}

fn main() {
    p1();
    p2();
}
