/// `LubyRestart`
pub struct LubyRestart {
    /// The initial restart limit. (default 100)
    first: i32,
    /// The factor with which the restart limit is multiplied in each restart. (default 2.5)
    inc: f64,
}
impl Default for LubyRestart {
    fn default() -> Self {
        LubyRestart {
            first: 100,
            inc: 2.5,
        }
    }
}

impl LubyRestart {
    /// `seq` returns
    pub fn seq(&mut self, mut restart_cnt: i32) -> f64 {
        let mut size = 1;
        let mut seq = 0;
        while size < restart_cnt + 1 {
            seq += 1;
            size = 2 * size + 1;
        }
        while size - 1 != restart_cnt {
            size = (size - 1) >> 1;
            seq -= 1;
            restart_cnt %= size;
        }
        f64::powi(self.inc, seq) * self.first as f64
    }
}
