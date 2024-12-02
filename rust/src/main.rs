/* [[[cog
import cog
from pathlib import Path
for day in sorted([p.name.replace(".rs", "") for p in list(Path("./rust/src").glob("day*.rs"))]):
   cog.outl(f"mod {day};")
]]]*/
mod day01;
mod day02;
/*[[[end]]] (checksum: d569f5bb526ffeafb69825d9a26f2f2d) */

fn read_input(day: &str) -> String {
    let path = format!("./inputs/{}_{}.txt", &day[..3], &day[3..]);
    match std::fs::read_to_string(path) {
        Ok(input) => input,
        Err(e) => panic!("Error reading input for {}: {}", day, e),
    }
}

fn main() {
    let day = std::env::args().nth(1).expect("Please provide a day");
    let input = read_input(&day);

    let (part1, part2) = match day.as_str() {
        /* [[[cog
        import cog
        from pathlib import Path
        for day in sorted([p.name.replace(".rs", "") for p in list(Path("./rust/src").glob("day*.rs"))]):
            cog.outl(f'"{day}" => ({day}::part1(&input), {day}::part2(&input)),')
        ]]]*/
        "day01" => (day01::part1(&input), day01::part2(&input)),
        "day02" => (day02::part1(&input), day02::part2(&input)),
        /*[[[end]]] (checksum: c0eed7951dc0dfa04d55d17494e5e15b)*/
        _ => panic!("Unknown day"),
    };

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
