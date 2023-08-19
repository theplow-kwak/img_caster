use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time::{Duration, Instant},
};

#[derive(Debug, Default, Clone, Copy)]
struct Amount {
    latency: Duration,
    bytes: usize,
    counts: usize,
}

impl Add for Amount {
    type Output = Amount;
    fn add(self, rhs: Amount) -> Amount {
        Amount {
            latency: self.latency + rhs.latency,
            bytes: self.bytes + rhs.bytes,
            counts: self.counts + rhs.counts,
        }
    }
}

impl Add for &Amount {
    type Output = <Amount as Add>::Output;
    fn add(self, rhs: &Amount) -> Self::Output {
        Amount::add(*self, *rhs)
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rhs: Amount) {
        self.latency += rhs.latency;
        self.bytes += rhs.bytes;
        self.counts += rhs.counts;
    }
}

impl AddAssign<&Amount> for Amount {
    fn add_assign(&mut self, rhs: &Amount) {
        Amount::add_assign(self, *rhs);
    }
}

impl Sub for Amount {
    type Output = Amount;
    fn sub(self, rhs: Amount) -> Amount {
        Amount {
            latency: self.latency - rhs.latency,
            bytes: self.bytes - rhs.bytes,
            counts: self.counts - rhs.counts,
        }
    }
}

impl Sub for &Amount {
    type Output = <Amount as Sub>::Output;
    fn sub(self, rhs: &Amount) -> Self::Output {
        Amount::sub(*self, *rhs)
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rhs: Amount) {
        self.latency -= rhs.latency;
        self.bytes -= rhs.bytes;
        self.counts -= rhs.counts;
    }
}

impl SubAssign<&Amount> for Amount {
    fn sub_assign(&mut self, rhs: &Amount) {
        Amount::sub_assign(self, *rhs);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    time: Instant,
    disk_io: Amount,
    sock_io: Amount,
}

impl Default for State {
    fn default() -> Self {
        Self {
            time: Instant::now(),
            disk_io: Amount::default(),
            sock_io: Amount::default(),
        }
    }
}

impl State {
    pub fn now(&self) -> State {
        State {
            time: Instant::now(),
            disk_io: self.disk_io,
            sock_io: self.sock_io,
        }
    }

    pub fn add_io(&mut self, latency: Duration, bytes: usize) {
        self.disk_io += Amount {
            latency,
            bytes,
            counts: 1,
        }
    }

    pub fn add_net(&mut self, latency: Duration, bytes: usize) {
        self.sock_io += Amount {
            latency,
            bytes,
            counts: 1,
        }
    }
}

impl Sub for State {
    type Output = State;
    fn sub(self, rhs: State) -> State {
        State {
            time: Instant::now(),
            disk_io: self.disk_io - rhs.disk_io,
            sock_io: self.sock_io - rhs.sock_io,
        }
    }
}

pub struct Statistics {
    start_time: Instant,
    state: State,
    elaps: Vec<State>,
    written_elaps: u128,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            state: State::default(),
            elaps: Vec::new(),
            written_elaps: 0,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now()
    }

}
