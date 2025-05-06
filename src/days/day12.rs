use std::fmt::Display;

struct Vector {
    x: i32,
    y: i32,
    z: i32,
}
impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

struct Moon {
    pos: Vector,
    vel: Vector,
}
impl Display for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "    [{}, {}]", self.pos, self.vel)
    }
}

pub fn solve() {
    #[cfg(ex)] let input = include_str!("../../data/day12.example");
    #[cfg(not(ex))] let input = include_str!("../../data/day12.input");

    let mut moons: Vec<_> = input.trim().lines()
        .map(|l| {
            let l = &l[1..l.len()-1];
            let (xstr, rest) = l.split_once(", ").unwrap();
            let (_, xstr) = xstr.split_once("=").unwrap();

            let (ystr, zstr) = rest.split_once(", ").unwrap();
            let (_, ystr) = ystr.split_once("=").unwrap();
            let (_, zstr) = zstr.split_once("=").unwrap();
            //println!("    '{}', '{}', '{}'", xstr, ystr, zstr);

            let pos = Vector{
                x: xstr.parse().unwrap(),
                y: ystr.parse().unwrap(),
                z: zstr.parse().unwrap()
            };
            let vel = Vector{ x: 0, y: 0, z: 0 };
            Moon { pos, vel }
        })
        //.inspect(|moon| println!("{}", moon))
        .collect();

    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j { continue; }

            //TODO: Fix this, 2 mutable borrows
            let a = &mut moons[i];
            let b = &mut moons[j];
            if a.pos.x > b.pos.x { a.vel.x += 1 }
            if a.pos.x < b.pos.x { a.vel.x -= 1 }
            if b.pos.x > a.pos.x { b.vel.x += 1 }
            if b.pos.x < a.pos.x { b.vel.x -= 1 }

            if a.pos.y > b.pos.y { a.vel.y += 1 }
            if a.pos.y < b.pos.y { a.vel.y -= 1 }
            if b.pos.y > a.pos.y { b.vel.y += 1 }
            if b.pos.y < a.pos.y { b.vel.y -= 1 }

            if a.pos.z > b.pos.z { a.vel.z += 1 }
            if a.pos.z < b.pos.z { a.vel.z -= 1 }
            if b.pos.z > a.pos.z { b.vel.z += 1 }
            if b.pos.z < a.pos.z { b.vel.z -= 1 }
        }
    }

    moons.iter().for_each(|moon| println!("    {}", moon));
}
