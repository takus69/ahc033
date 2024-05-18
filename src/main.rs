use proconio::input;

struct Input {
    n: usize,
    a: Vec<usize>,
}

fn parse_input() -> Input {
    input! {
        n: usize,
        a: [usize; n*n],
    }

    Input {n, a}
}

struct Container {
    n: usize,
    id: usize,
    out_i: usize,
    out_order: usize,
    pos_i: usize,
    pos_j: usize,
    is_in: bool,
    is_out: bool,
}

impl Container {
    fn new(n: usize, a: usize) -> Container {
        let id = a;
        let out_i = id / n;
        let out_order = id % n;
        let pos_i = n + 1;
        let pos_j = n + 1;
        let is_in = true;
        let is_out = false;

        Container { n, id, out_i, out_order, pos_i, pos_j, is_in, is_out }
    }

}

struct Monitor {
    n: usize,
    containers: Vec<Container>,  // 搬入される順のコンテナを保持.一次元配列で保持
    in_cnt: Vec<usize>,  // i搬入口から搬入された個数(次に搬入される添え字)を保持.搬入口に配置されると次の添え字になる
    out_cnt: Vec<usize>,  // i搬出口から搬出された個数を保持
    mass: Vec<Option<Container>>,  // マスに一時配置されたコンテナを保持
}

impl Monitor {
    fn new(input: Input) -> Monitor {
        let n = input.n;
        let mut containers = Vec::new();
        for ai in input.a.iter() {
            containers.push(Container::new(n, *ai));
        }
        let in_cnt = vec![0; n];
        let out_cnt = vec![0; n];
        let mass: Vec<Option<Container>> = (0..n).map(|_| None).collect();

        Monitor { n, containers, in_cnt, out_cnt, mass }
    }
}

struct Solver {
    n: usize,
    monitor: Monitor,
    ans: Vec<String>,
}

impl Solver {
    fn new(input: Input) -> Solver {
        let n = input.n;
        let monitor = Monitor::new(input);
        let ans = vec![String::from(""); n];

        Solver { n, monitor, ans }
    }

    fn solve(&mut self) {
        for i in 0..self.n {
            self.ans[i] += "B";
        }
    }

    fn ans(&self) {
        for a in self.ans.iter() {
            println!("{}", a);
        }
    }
}

fn main() {
    let input = parse_input();
    let mut solver = Solver::new(input);
    solver.solve();
    solver.ans();
}
