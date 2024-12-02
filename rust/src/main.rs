/* [[[cog
import cog
from pathlib import Path
for day in sorted([p.name.replace(".rs", "") for p in list(Path("./rust/src").glob("day*.rs"))]):
   cog.outl(f"mod {day};")
]]]*/
mod day01;
mod day02;
/*[[[end]]] (checksum: d569f5bb526ffeafb69825d9a26f2f2d) */

fn main() {
    println!("Hello, world!");
}
