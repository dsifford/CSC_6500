pub fn processor_cost(p1: usize, p2: usize, p3: usize, p4: usize, p5: usize) -> u32 {
    let modules = vec![
        vec![4, 5, 20, 8, 12],
        vec![9, 5, 2, 10, 15],
        vec![15, 9, 5, 6, 3],
        vec![20, 6, 7, 4, 10],
        vec![8, 6, 12, 20, 17],
    ];
    modules[p1][0] + modules[p2][1] + modules[p3][2] + modules[p4][3] + modules[p5][4]
}

pub fn communication_cost(p1: usize, p2: usize, p3: usize, p4: usize, p5: usize) -> u32 {
    let comms = vec![
        vec![0, 6, 10, 3, 8],
        vec![6, 0, 8, 10, 9],
        vec![10, 8, 0, 9, 7],
        vec![3, 10, 9, 0, 5],
        vec![8, 9, 7, 5, 0],
    ];
    comms[p1][p2]
        + comms[p1][p3]
        + comms[p1][p4]
        + comms[p1][p5]
        + comms[p2][p3]
        + comms[p2][p4]
        + comms[p2][p5]
        + comms[p3][p4]
        + comms[p3][p5]
        + comms[p4][p5]
}
