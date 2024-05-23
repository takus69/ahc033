use proconio::input;

#[derive(Clone)]
struct Input {
    n: usize,
    a: Vec<usize>,
}

fn parse_input() -> Input {
    input! {
        n: usize,
        a: [usize; n*n],
    }

    Input { n, a }
}


#[derive(Debug, Clone)]
struct Container {
    n: usize,
    id: usize,
    in_i: usize,
    in_j: usize,
    out_i: usize,
    out_j: usize,
    pos_i: usize,
    pos_j: usize,
    is_in: bool,
    is_board: bool,
    is_out: bool,
}

impl Container {
    fn new(n: usize, a: usize, in_i: usize, in_j: usize) -> Container {
        let id = a;
        let out_i = id / n;
        let out_j = id % n;
        let pos_i = n;
        let pos_j = n;
        let is_in = true;
        let is_board = false;
        let is_out = false;

        Container { n, id, in_i, in_j, out_i, out_j, pos_i, pos_j, is_in, is_board, is_out }
    }

    fn get_pos(&self) -> (usize, usize) {
        (self.pos_i, self.pos_j)
    }

    fn get_out_pos(&self) -> (usize, usize) {
        (self.out_i, self.n-1)
    }
}

struct Monitor {
    n: usize,
    containers: Vec<Container>,  // すべてのコンテナをコンテナIDの順で保持
    in_orders: Vec<usize>, // 搬入される順のコンテナIDを保持.一次元配列で保持
    in_cnt: Vec<usize>,  // i搬入口から搬入された個数(次に搬入される添え字)を保持.搬入口に配置されると次の添え字になる
    out_cnt: Vec<usize>,  // i搬出口から搬出された個数を保持
    board: Vec<usize>,  // マスに一時配置されたコンテナを保持
    cranes: Vec<Crane>,  // すべてのクレーンを保持
}

impl Monitor {
    fn new(input: Input) -> Monitor {
        let n = input.n;
        let mut containers = Vec::new();
        for in_i in 0..n {
            for in_j in 0..n {
                let ai = input.a[in_i*n+in_j];
                containers.push(Container::new(n, ai, in_i, in_j));
            }
        }
        containers.sort_by(|a, b| a.id.cmp(&b.id));
        let mut in_orders = Vec::new();
        for ai in input.a.into_iter() {
            in_orders.push(ai);
        }
        let in_cnt = vec![0; n];
        let out_cnt = vec![0; n];
        let board: Vec<usize> = vec![n*n; n*n];

        // クレーンの初期化
        let mut cranes = Vec::new();
        for in_i in 0..n {
            cranes.push(Crane::new(in_i));
        } 

        Monitor { n, containers, in_orders, in_cnt, out_cnt, board, cranes }
    }

    fn get_container_id(&mut self, ai: usize) -> &mut Container {
        &mut self.containers[ai]
    }
    fn get_container(&mut self, out_i: usize, out_j: usize) -> &mut Container {
        self.get_container_id(out_i*self.n + out_j)
    }

    fn get_board(&self, i: usize, j: usize) -> usize {
        self.board[i*self.n + j]
    }

    fn set_board(&mut self, ai: usize, pos: (usize, usize)) {
        self.board[pos.0*self.n + pos.1] = ai;
        self.containers[ai].pos_i = pos.0;
        self.containers[ai].pos_j = pos.1;
        self.containers[ai].is_board = true;
        self.containers[ai].is_in = false;
        self.containers[ai].is_out = false;
    }

    fn set_out(&mut self, ai: usize, out_pos: (usize, usize)) {
        self.board[out_pos.0*self.n + out_pos.1] = self.n*self.n;
        self.containers[ai].pos_i = self.n;
        self.containers[ai].pos_j = self.n;
        self.containers[ai].is_board = false;
        self.containers[ai].is_in = false;
        self.containers[ai].is_out = true;
        self.out_cnt[out_pos.0] += 1;
    }

    fn is_board(&self, i: usize, j: usize) -> bool {
        let mut ret = false;
        if self.board[i*self.n + j] < self.n*self.n {
            ret = true;
        }
        ret
    }

    fn remove_board(&mut self, pos: (usize, usize)) {
        self.board[pos.0*self.n + pos.1] = self.n*self.n;
    }

    fn get_container_from_in(&mut self, in_i: usize, in_j: usize) -> &mut Container {
        self.get_container_id(self.in_orders[in_i*self.n + in_j])
    }

    fn move_cnt(&mut self, out_i: usize) -> usize {
        let mut cnt = 0;
        for out_j in 0..self.n {
            let container = self.get_container(out_i, out_j);
            if container.is_in {
                let in_i = container.in_i;
                let in_j = container.in_j;
                let in_cnt = self.in_cnt[in_i];
                if in_j > in_cnt {
                    cnt += in_j - in_cnt;
                }
            }
        }

        cnt
    }

    fn r#move(&mut self, ai: usize, pos: (usize, usize)) {
        let container = self.get_container_id(ai);
        let start = container.get_pos();
        // println!("remove ai: {}, start: {:?}, pos: {:?}", ai, start, pos);
        self.cranes[0].r#move(start);
        self.cranes[0].hold();
        self.cranes[0].r#move(pos);
        self.cranes[0].set();

        // board更新
        self.remove_board(start);
        self.set_board(ai, pos);
    }

    fn out(&mut self, ai: usize) {
        let container = self.get_container_id(ai);
        let out_pos = container.get_out_pos();
        self.r#move(ai, out_pos);
    }

    fn is_done(&self, out_i: usize) -> bool {
        if self.out_cnt[out_i] == self.n {
            return true
        }
        false
    }

    fn free_space(&self, out_i: usize) -> (usize, usize) {
        let mut target_i = vec![out_i];
        for i in 1..self.n {
            if out_i + i < self.n {
                target_i.push(out_i+i);
            }
            if out_i >= i {
                target_i.push(out_i-i);
            }
        }
        for i in target_i {
            for j in (1..(self.n-1)).rev() {
                let ai = self.get_board(i, j);
                if ai < self.n*self.n {
                    continue;
                } else {
                    return (i, j);
                }
            }
        }
        (0, 0)
    }

    fn get_out_cnt(&self, out_i: usize) -> usize {
        self.out_cnt[out_i]
    }

    fn turn(&mut self) {
        // 搬入口に設置
        for in_i in 0..self.n {
            if !self.is_board(in_i, 0) && self.in_cnt[in_i] < self.n {
                let in_cnt = self.in_cnt[in_i];
                let container = self.get_container_from_in(in_i, in_cnt);
                let ai = container.id;
                self.set_board(ai, (in_i, 0));
                self.in_cnt[in_i] += 1;
            }
        }

        // 搬出口から搬出
        for out_i in 0..self.n {
            if self.is_board(out_i, self.n-1) {
                let ai = self.get_board(out_i, self.n-1);
                let container = self.get_container_id(ai);
                assert_eq!(container.out_i, out_i, "Wrong out_i: ai: {}, correct out_i: {}, wrong out_i: {}", ai, container.out_i, out_i);
                self.set_out(ai, (out_i, self.n-1));
            }
        }
    }
}

struct Crane {
    is_big: bool,
    pos_i: usize,
    pos_j: usize,
    hold: bool,
    move_s: String,
}

impl Crane {
    fn new(in_i: usize) -> Crane {
        let is_big = in_i == 0;
        let pos_i = in_i;
        let pos_j = 0;
        let hold = false;
        let move_s = String::from("");

        Crane { is_big, pos_i, pos_j, hold, move_s }
    }

    fn hold(&mut self) {
        self.move_s += "P";
        self.hold = true;
    }

    fn set(&mut self) {
        self.move_s += "Q";
        self.hold = false;
    }

    fn bomb(&mut self) {
        self.move_s += "B";
    }

    fn r#move(&mut self, pos: (usize, usize)) {
        // 上下の移動
        if self.pos_i > pos.0 {
            self.move_s += &"U".repeat(self.pos_i - pos.0);
        } else {
            self.move_s += &"D".repeat(pos.0 - self.pos_i);
        }
        // 左右の移動
        if self.pos_j > pos.1 {
            self.move_s += &"L".repeat(self.pos_j - pos.1);
        } else {
            self.move_s += &"R".repeat(pos.1 - self.pos_j);
        }
        self.pos_i = pos.0;
        self.pos_j = pos.1;
    }
}

struct Solver {
    n: usize,
    monitor: Monitor,
}

impl Solver {
    fn new(input: Input) -> Solver {
        let n = input.n;
        let monitor = Monitor::new(input);

        Solver { n, monitor }
    }

    fn solve(&mut self) {
        let n = self.n;

        self.monitor.turn();

        // 大きいクレーン以外は爆破
        for i in 1..n {
            self.monitor.cranes[i].bomb();
        }

        // 搬出口の数だけ実施
        for _ in 0..n {
            // 一時配置が一番少ない搬出口を検索
            let mut min_move = n*n;
            let mut min_move_i = n;
            for out_i in 0..n {
                if self.monitor.is_done(out_i) { continue; }
                let move_cnt = self.monitor.move_cnt(out_i);
                if min_move > move_cnt {
                    min_move = move_cnt;
                    min_move_i = out_i;
                }
            }

            // 一時配置が一番少ない搬出口のコンテナを順に搬出
            let out_cnt = self.monitor.get_out_cnt(min_move_i);
            // println!("min_move_i: {}, out_cnt: {}", min_move_i, out_cnt);
            for out_j in out_cnt..self.n {
                // println!("min_move_i: {}, out_j: {}", min_move_i, out_j);
                let out_container = self.monitor.get_container(min_move_i, out_j);
                // 処理する搬入口を特定
                let out_ai = out_container.id;
                let in_i = out_container.in_i;
                let out_in_j = out_container.in_j;
                let in_cnt = self.monitor.in_cnt[in_i];
                // 搬出したいコンテナの搬入口の前にあるコンテナを一時配置する
                // println!("out_ai: {}, in_i: {}, out_in_j: {}, in_cnt: {}", out_ai, in_i, out_in_j, in_cnt);
                // println!("in_cnt: {:?}, out_cnt: {:?}", self.monitor.in_cnt, self.monitor.out_cnt);
                for in_j in (in_cnt-1)..out_in_j {
                    self.monitor.turn();
                    // println!("in_cnt: {:?}, out_cnt: {:?}", self.monitor.in_cnt, self.monitor.out_cnt);
                    let container = self.monitor.get_container_from_in(in_i, in_j);
                    let ai = container.id;
                    let out_i = container.out_i;
                    // println!("ai: {}, in_i: {}, in_j: {}", ai, in_i, in_j);
                    let pos = self.monitor.free_space(out_i);
                    self.monitor.r#move(ai, pos);
                }
                self.monitor.turn();
                self.monitor.out(out_ai);
                self.monitor.turn();
                // println!("### out_ai: {}, out_cnt: {:?}", out_ai, self.monitor.out_cnt);
            }
        }
    }

    fn ans(&self) {
        let mut score = 0;
        for crane in self.monitor.cranes.iter() {
            println!("{}", crane.move_s);
        }
    }

    fn result(&self) {
        let mut score = 0;
        for crane in self.monitor.cranes.iter() {
            score = score.max(crane.move_s.len());
        }
        eprintln!("{{ \"score\": {} }}", score);
    }
}

fn main() {
    let input = parse_input();
    let mut solver = Solver::new(input);
    solver.solve();
    solver.ans();
    solver.result();
}


// tests
#[cfg(test)]
mod tests {
    use super::*;
    
    fn make_input() -> Input {
        let n = 5;
        let a: Vec<usize> = vec![
            24, 10, 17, 15, 13,
            14, 11,  2,  1,  5,
             7,  9,  6, 21, 20,
             8,  4, 19,  3, 16,
            18, 23, 22,  0, 12,
        ];
        Input { n, a }
    }

    #[test]
    fn test_monitor() {
        let input = make_input();
        let mut monitor = Monitor::new(input.clone());

        // new
        assert_eq!(monitor.containers[0].in_i, 4);
        assert_eq!(monitor.containers[0].in_j, 3);
        assert_eq!(monitor.containers[0].out_i, 0);
        assert_eq!(monitor.containers[0].out_j, 0);

        // is_done
        for i in 0..input.n {
            assert!(!monitor.is_done(i));
        }

        // free_space
        assert_eq!(monitor.free_space(4), (4, 3));
        assert_eq!(monitor.free_space(1), (1, 3));
    
        // move_cnt
        assert_eq!(monitor.move_cnt(0), 12);

        // get_container
        let container = monitor.get_container(1, 0);
        assert_eq!(container.id, 5);

        // turn
        let ai = monitor.get_board(0, 0);
        assert_eq!(ai, 25);
        monitor.turn();
        let ai = monitor.get_board(1, 0);
        assert_eq!(ai, 14);
        assert_eq!(monitor.containers[ai].get_pos(), (1, 0));
        monitor.r#move(ai, (3, 3));
        let ai = monitor.get_board(3, 3);
        assert_eq!(ai, 14);
        let crane = &monitor.cranes[0];
        let pos = (crane.pos_i, crane.pos_j);
        assert_eq!(pos, (3, 3));

        monitor.turn();
        assert_eq!(monitor.get_board(0, 0), 24);
        let container = monitor.get_container_from_in(0, 0);
        assert_eq!(container.pos_i, 0);
        assert_eq!(container.pos_j, 0);
        monitor.r#move(24, (1, 1));
        let container = monitor.get_container_id(24);
        assert_eq!(container.pos_i, 1);
        assert_eq!(container.pos_j, 1);
        monitor.turn();
        assert_eq!(monitor.get_board(0, 0), 10);
    }

    #[test]
    fn test_container() {
    }

}
