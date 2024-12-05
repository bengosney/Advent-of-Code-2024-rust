#[cfg(test)]
const EXAMPLE_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

#[test]
fn test_part1_example() {
    let part1 = part1(EXAMPLE_INPUT);
    assert_eq!(part1, Result::Ok(143));
}

#[test]
fn test_part2_example() {
    let part2 = part2(EXAMPLE_INPUT);
    assert_eq!(part2, Result::Ok(123));
}

type Rule = (usize, usize);
type Book = Vec<usize>;

fn process_input(input: &str) -> (Vec<Rule>, Vec<Book>) {
    let raw: Vec<&str> = input.trim().split("\n\n").collect();
    let rules: Vec<Rule> = raw[0]
        .lines()
        .map(|line| {
            let mut parts = line.split("|");
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let books: Vec<Book> = raw[1]
        .lines()
        .map(|line| line.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, books)
}

fn in_order(rules: &Vec<Rule>, book: &Book) -> bool {
    for (pos, page) in book.iter().enumerate() {
        for pair in rules.iter() {
            if page == &pair.1 && book[pos + 1..].contains(&pair.0) {
                return false;
            }
        }
    }

    return true;
}

fn filter_rules(rules: &Vec<Rule>, book: &Book) -> Vec<Rule> {
    rules
        .iter()
        .filter(|(a, b)| book.contains(a) && book.contains(b))
        .cloned()
        .collect()
}

fn fix_order(rules: &Vec<Rule>, book: &Book) -> Book {
    let mut ordered_book = book.clone();

    while !in_order(rules, &ordered_book) {
        let cloned_book = ordered_book.clone();
        for (pos, page) in cloned_book.iter().enumerate() {
            for pair in rules.iter() {
                let idx = match ordered_book.iter().position(|&x| x == pair.0) {
                    Some(i) => i,
                    None => 0,
                };
                if page == &pair.1 && idx > pos {
                    ordered_book.swap(pos, idx);
                }
            }
        }
    }

    return ordered_book;
}

pub fn part1(input: &str) -> Result<i32, &'static str> {
    let (rules, books) = process_input(input);

    let mut total: i32 = 0;

    for book in books.iter() {
        let rules = filter_rules(&rules, book);
        if in_order(&rules, book) {
            total += book[book.len() / 2] as i32;
        }
    }

    Ok(total)
}

pub fn part2(input: &str) -> Result<i32, &'static str> {
    let (rules, books) = process_input(input);

    let mut total: i32 = 0;

    for book in books.iter() {
        let rules = filter_rules(&rules, book);
        if !in_order(&rules, book) {
            let ordered_book = fix_order(&rules, book);
            total += ordered_book[book.len() / 2] as i32;
        }
    }

    Ok(total)
}
