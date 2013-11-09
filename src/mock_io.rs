
pub enum IReader {
    MockReader { str_in_stdin: ~str, read_line_call_count: int },
    RealReader(@Reader)
}

impl IReader {
    pub fn read_line(&self) -> ~str {
        match *self {
            MockReader { str_in_stdin: ref fake_input,
                         read_line_call_count: ref call_count } =>
                self.fake_read_line(fake_input.clone(), *call_count),

            RealReader(r) =>
                r.read_line()
        }
    }

    fn fake_read_line(&self, fake_input: ~str, call_count: int) -> ~str {
        let v: ~[&str] = fake_input.line_iter().collect();

        v[call_count].to_owned()
    }
}

