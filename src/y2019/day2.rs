fn run_program(opcodes: &mut Vec<usize>) {
    let mut pointer: usize = 0;
    loop {
        match opcodes.get(pointer) {
            Some(1) => {
                if let [x, y, z] = opcodes[pointer + 1..=pointer + 3] {
                    let (xx, yy, zz) = (opcodes[x], opcodes[y], &mut opcodes[z]);
                    *zz = xx + yy;
                }
                pointer = pointer + 4;
            }
            Some(2) => {
                if let [x, y, z] = opcodes[pointer + 1..=pointer + 3] {
                    let (xx, yy, zz) = (opcodes[x], opcodes[y], &mut opcodes[z]);
                    *zz = xx * yy;
                }
                pointer = pointer + 4;
            }
            Some(99) => return,
            _ => unreachable!(),
        }
    }
}

pub fn solve(input: &str) -> Option<usize> {
    let mut opcodes = input
        .split(',')
        .map(|d| d.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    opcodes[1] = 12;
    opcodes[2] = 2;
    run_program(&mut opcodes);
    opcodes.get(0).cloned()
}

pub fn solve2(input: &str) -> Option<(usize, usize)> {
    let opcodes = input
        .split(',')
        .map(|d| d.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    (0usize..=99usize)
        .flat_map(|noun| {
            (0usize..=99usize)
                .filter_map(|verb| {
                    let mut _opcodes = opcodes.clone();
                    _opcodes[1] = noun;
                    _opcodes[2] = verb;
                    run_program(&mut _opcodes);
                    _opcodes
                        .get(0)
                        .filter(|d| **d == 19690720)
                        .map(|_| (noun, verb))
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .next()
}

pub const INPUT: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,9,19,1,19,5,23,2,6,23,27,1,6,27,31,2,31,9,35,1,35,6,39,1,10,39,43,2,9,43,47,1,5,47,51,2,51,6,55,1,5,55,59,2,13,59,63,1,63,5,67,2,67,13,71,1,71,9,75,1,75,6,79,2,79,6,83,1,83,5,87,2,87,9,91,2,9,91,95,1,5,95,99,2,99,13,103,1,103,5,107,1,2,107,111,1,111,5,0,99,2,14,0,0";
