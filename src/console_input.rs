use std::vec::*;
use std::io::*;

use console_reader::*;
mod console_reader;

struct ConsoleInput {
    reader: ConsoleReader
}

impl ConsoleInput {

    pub fn new(input: ConsoleReader) -> ConsoleInput {
        ConsoleInput { reader: input }
    }

    pub fn get_int(&self) -> Option<int> {
        let input = self.reader.read_line();

        from_str::<int>(input.trim())
    }

    pub fn clone(&self) -> ConsoleInput {
        ConsoleInput::new(self.reader.clone())
    }
}

#[cfg(test)]
mod test__input {
    use super::*;
    use console_reader::*;

    fn create_io_with_mocks(fake_input: ~str) -> ConsoleInput {
        let fake_reader = MockReader { str_in_stdin: fake_input };
        ConsoleInput::new(fake_reader)
    }

    #[test]
    fn gets_an_integer_from_its_reader() {
        let io = create_io_with_mocks(~"  1  \n");
        let io_with_invalid_input = create_io_with_mocks(~"wazzup\n");

        assert_eq!(Some(1), io.get_int());
        assert_eq!(None, io_with_invalid_input.get_int());
    }
}

