#[derive(Clone, Copy, Debug, Default)]
pub struct Time {
    dt_millis: u32,
    t_millis: u64,
}

impl Time {
    pub fn advance_dt(&mut self, dt: u32) {
        self.dt_millis = dt;
        self.t_millis += u64::from(dt);
    }

    pub fn dt_millis(self) -> u32 {
        self.dt_millis
    }

    pub fn t_millis(self) -> u64 {
        self.t_millis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advance_dt_increments_both_dt_and_t() {
        let mut time = Time::default();
        time.advance_dt(1000);
        assert_eq!(1000, time.dt_millis());
        assert_eq!(1000, time.t_millis());
        time.advance_dt(2000);
        assert_eq!(2000, time.dt_millis());
        assert_eq!(3000, time.t_millis());
        time.advance_dt(250);
        assert_eq!(250, time.dt_millis());
        assert_eq!(3250, time.t_millis());
    }
}
