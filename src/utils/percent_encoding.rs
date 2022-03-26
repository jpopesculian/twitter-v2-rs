pub use percent_encoding::{AsciiSet, PercentEncode, CONTROLS};

const URL: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'!')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b',')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b']');

pub fn percent_encode<T>(input: &T) -> PercentEncode
where
    T: AsRef<[u8]>,
{
    percent_encoding::percent_encode(input.as_ref(), URL)
}
