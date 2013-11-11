
pub enum IReader {
    MockReader { str_in_stdin: ~str },
    RealReader(@Reader)
}

impl IReader {
    pub fn read_line(&self) -> ~str {
        match *self {
            MockReader { str_in_stdin: ref fake_input } =>
                self.fake_read_line(fake_input.clone()),

            RealReader(r) =>
                r.read_line()
        }
    }

    fn fake_read_line(&self, fake_input: ~str) -> ~str {
        let v: ~[&str] = fake_input.line_iter().collect();

        v[0].to_owned()
    }

    pub fn clone(&self) -> IReader {
        match *self {
            MockReader { str_in_stdin: ref fake_input } =>
                MockReader { str_in_stdin: fake_input.clone() },

            RealReader(r) => RealReader(r)
        }
    }
}

