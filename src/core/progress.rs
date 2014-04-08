use std::io;
use std::cmp::max;

pub struct ProgressReporter {
    total_work: uint,
    work_done: uint,
    plusses_printed: uint,
    total_plusses: uint,
    title: ~str
}

impl ProgressReporter {
    pub fn new(tw: uint, bar_length: uint, title: ~str) -> ProgressReporter {
        let total_plusses = max(2, bar_length - title.len());
        let plusses_printed = 0;
        let work_done = 0;

        let mut buf = format!("\r{}: [", title);

        for i in range(0, total_plusses) {
            buf.push_char(' ');
        }

        buf.push_str("] ");

        print!("{}", buf);

        return ProgressReporter { total_work: tw, work_done: 0, plusses_printed: 0, total_plusses: total_plusses, title: title }
    }

    pub fn update(&mut self, num: uint) {
        self.work_done += num;
        let percent_done = (self.work_done as f32) / (self.total_work as f32);
        let mut plusses_needed = (self.total_plusses as f32 * percent_done) as uint;

        if plusses_needed > self.total_plusses {
            plusses_needed = self.total_plusses;
        }

        let mut buf = format!("\r{}: [", self.title);
        let mut i = 0;

        while i < plusses_needed {
            buf.push_char('+');
            i += 1;
        }

        while i < self.total_plusses {
            buf.push_char(' ');
            i += 1;
        }

        buf.push_str("] ");

        print!("{}", buf);
        io::stdio::flush();
    }

    pub fn done(&mut self) {
        let mut buf = format!("\r{}: [", self.title);

        for _ in range(0, self.total_plusses) {
            buf.push_char('+');
        }

        buf.push_str("] ");

        println!("{}", buf);
        io::stdio::flush();
    }
}