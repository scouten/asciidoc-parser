use std::fmt::{Debug, Error, Formatter};

pub(crate) struct DebugSliceReference<'a, T: Debug>(pub(crate) &'a [T]);

impl<'a, T: Debug> Debug for DebugSliceReference<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("&")?;
        f.debug_list().entries(self.0.iter()).finish()
    }
}
