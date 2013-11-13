
#[deriving(Clone)]
pub enum ConsoleWriter {
    MockWriter { printed_str: ~str },
    RealWriter
}

impl ConsoleWriter {
    pub fn get_printed_str(&self) -> ~str {
        match *self {
            MockWriter { printed_str: ref printed_str } => printed_str.clone(),
            RealWriter => ~""
        }
    }

    pub fn println(&self, str_to_print: ~str) -> @ConsoleWriter {
        match *self {
            MockWriter { printed_str: _ } =>
                @MockWriter { printed_str: str_to_print },

            RealWriter => self.real_println(str_to_print)
        }
    }

    fn real_println(&self, str_to_print: ~str) -> @ConsoleWriter {
        println(str_to_print);

        @(self.clone())
    }
}


