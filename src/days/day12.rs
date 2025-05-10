use std::fmt::Display;

#[derive(Clone, PartialEq)]
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

#[derive(Clone)]
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
    let moons_base = moons.clone();
    let mut moons_p2 = moons.clone();

    const STEPS: i32 = 1000;
    for _ in 0..STEPS {
        step_system(&mut moons);
    }

    let p1 = moons.iter()
        .fold(0 as i32, |acc, moon| {
            acc + calc_energy(moon)
        });
    println!("    Part 1 = {}", p1);

    //TODO: Do we need to find GCD or LCD? Are they the same?
    let idx = 3;
    let mut count = 0;
    step_system(&mut moons_p2);
    while moons_base.get(idx).unwrap().pos != moons_p2.get(idx).unwrap().pos {
        step_system(&mut moons_p2);
        count += 1;
    }
    println!("{} == {} -> {}",
        moons_base.get(idx).unwrap().pos,
        moons_p2.get(idx).unwrap().pos,
        count
    );
}

fn step_system(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j { continue; }
            unsafe {
                let arr = moons.as_mut_ptr();
                let a = &mut *arr.add(i);
                let b = & *arr.add(j);
                a.vel.x += (b.pos.x - a.pos.x).signum();
                a.vel.y += (b.pos.y - a.pos.y).signum();
                a.vel.z += (b.pos.z - a.pos.z).signum();
            }
        }
    }

    for moon in moons.iter_mut() {
        moon.pos.x += moon.vel.x;
        moon.pos.y += moon.vel.y;
        moon.pos.z += moon.vel.z;
    }
}

fn calc_energy(moon: &Moon) -> i32 {
    let potential = moon.pos.x.abs() + moon.pos.y.abs() + moon.pos.z.abs();
    let kinetic = moon.vel.x.abs() + moon.vel.y.abs() + moon.vel.z.abs();

    //println!("    [MOON] {}", moon);
    //println!("    [ENERGY] {} * {}", potential, kinetic);
    return potential * kinetic;
}
