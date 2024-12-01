/* [[[cog
import cog
from pathlib import Path
for day in sorted([p.name.replace(".rs", "") for p in list(Path("./rust/src").glob("day*.rs"))]):
   cog.outl(f"mod {day};")
]]]*/
mod day01;
/*[[[end]]] (checksum: 909f8627da7e00e70db3ecbb5ee3ebb0) */

fn main() {
    println!("Hello, world!");
}
