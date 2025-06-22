use std::time::{Duration, SystemTime};

use rand::{self, random_bool, rng, seq::SliceRandom};
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Sudoku {
    data: [[u32; 9]; 9],
    seen: [[u32; 9]; 9],
    diff: f64,
}

impl Sudoku {
    pub fn set_diff(&mut self, diff: f64) { self.diff = diff; }

    pub fn check(&self) -> bool {
        let g = &self.data;
        for row in 0..9 {
            let r = &g[row];
            let mut checker = [false; 10];
            for idx in 0..9 {
                let num = r[idx] as usize;
                if num == 0 { continue }
                if checker[num] { return false; }
                checker[num] = true;
            }
        }
        for col in 0..9 {
            let mut checker = [false; 10];
            for idx in 0..9 {
                let num = g[idx][col] as usize;
                if num == 0 { continue }
                if checker[num] { return false; }
                checker[num] = true;
            }
        }
        for row in 0..3 {
            for col in 0..3 {
                let mut checker = [false; 10];
                for r_idx in 0..3 {
                    for c_idx in 0..3 {
                        let x = row * 3 + r_idx; 
                        let y = col * 3 + c_idx;
                        let num = g[x][y] as usize;
                        if num == 0 { continue }
                        if checker[num] { return false; }
                        checker[num] = true;
                    }
                }
            }
        }
        true
    }

    pub fn flush(&mut self) {
        let s_timer = SystemTime::now();
        let fst_row = &mut self.data[0];
        let mut rng = rng();
        let mut t = (1..10).collect::<Vec<_>>();
        t.shuffle(&mut rng);
        fst_row.copy_from_slice(&t);

        let mut idx = 1;
        while idx < 9 {
            if idx == 8 {
                for id in 0..9 {
                    let mut tot = 45;
                    for r in 0..8 {
                        tot -= self.data[r][id];
                    }
                    self.data[8][id] = tot;
                    if tot == 0 || tot > 9 {
                        idx -= 1;
                        break;
                    }
                }
                if idx == 8 { break }
            }
            loop {
                let row = &mut self.data[idx];
                t.shuffle(&mut rng);
                row.copy_from_slice(&t);
                if self.check() { break }
                if s_timer.elapsed().unwrap() > Duration::from_millis(500) { 
                    self.clear();
                    debug_assert!({println!("Retry flush Sudoku."); true});
                    self.flush();
                    return 
                }
            }
            idx += 1;
        }
        
        debug_assert!({println!("{:?}", s_timer.elapsed().unwrap()); true});
        self.init_show();
    }

    fn init_show(&mut self) {
        self.clear_seen();
        for row in 0..9 {
            for col in 0..9 {
                if random_bool(1.0 - self.diff) {
                    self.seen[row][col] = self.data[row][col];
                }
            }
        }
    }

    pub fn check_idx(&mut self, num: u32, idx: u32) -> bool {
        let (x, y) = ((idx / 9) as usize, (idx % 9) as usize);
        if self.data[x][y] == num {
            self.seen[x][y] = num;
            true
        } else { false }
    }

    fn clear(&mut self) {
        for idx in 0..9 {
            self.data[idx].fill(0);
        }
    }
    
    fn clear_seen(&mut self) {
        for idx in 0..9 {
            self.seen[idx].fill(0);
        }
    }

    pub fn check_complete(&self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.data[row][col] != self.seen[row][col] {
                    return false;
                }
            }
        }
        true
    }

    #[allow(dead_code)]
    fn print(&self) {
        for idx in 0..9 {
            println!("{:?}", self.data[idx]);
        }
    }

    pub fn print_cur(&self) {
        let r = &self.seen;
        let st = String::from_utf16(&vec![0x005F; 25]).unwrap();
        let mid = String::from_utf16(&vec![0x2015; 25]).unwrap();
        let fl = String::from_utf16(&vec![0x203E; 25]).unwrap();
        let gun = String::from_utf16(&vec![0xFF5C]).unwrap();
        print!("   ");
        for idx in 1..10 {
            print!(" {}{}", idx, if idx % 3 == 0 {" "} else {""});
        }
        println!(" ");
        println!("{}", st);
        for idx in 0..9 {
            let t = &r[idx];
            print!("r{}", idx + 1);
            for off in 0..9 {
                print!("{}{}", if off % 3 == 0 {gun.as_str()} else {" "}, if t[off] == 0 {".".to_string()} else {t[off].to_string()});
            }
            println!("{}", gun);
            if idx < 8 && idx % 3 == 2 {
                println!("{}", mid);
            }
        }
        println!("{}", fl);
    }
}