use parsenic::result::LenResult;
use traitful::extend;

#[extend]
pub(crate) trait Read: parsenic::Read {
    /// Get a nul terminated String out of a reader
    fn strz(&mut self) -> LenResult<String> {
        let mut bytes = [0u8; 4];
        let mut index = 0;
        let mut out = String::new();

        loop {
            let byte = self.u8()?;

            if byte == 0 {
                break;
            }

            bytes[index] = byte;
            index += 1;

            match std::str::from_utf8(&bytes[0..index]) {
                Ok(c) => {
                    out.push_str(c);
                    index = 0;
                }
                Err(e) => {
                    if e.error_len().is_some() {
                        out.push(std::char::REPLACEMENT_CHARACTER);
                        index = 0;
                    }
                }
            }
        }

        Ok(out)
    }
}
