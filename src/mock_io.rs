
pub struct MockReaderInfo {
    str_in_stdin: ~str,
    read_line_call_count: int
}

pub enum IReader {
    MockReader(@MockReaderInfo),
    RealReader(@Reader)
}

pub enum IWriter {
    MockWriter(~[~str]),
    RealWriter(@Writer)
}

impl IReader {
    pub fn read_line(&self) -> ~str {
        match *self {
            MockReader(mock_info) => self.fake_read_line(mock_info.str_in_stdin.clone(),
                                                         mock_info.read_line_call_count.clone()),

            RealReader(r)         => r.read_line()
        }
    }

    fn fake_read_line(&self, fake_input: ~str, call_count: int) -> ~str {
        let v: ~[&str] = fake_input.line_iter().collect();

        v[call_count].to_owned()
    }
}

