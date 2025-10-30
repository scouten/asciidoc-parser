use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
};

pub(crate) struct DebugSliceReference<'a, T: Debug>(pub(crate) &'a [T]);

impl<'a, T: Debug> Debug for DebugSliceReference<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("&")?;
        f.debug_list().entries(self.0.iter()).finish()
    }
}

pub(crate) struct DebugHashMapFrom<'a, K: Debug, V: Debug>(pub(crate) &'a HashMap<K, V>);

impl<'a, K: Debug, V: Debug> Debug for DebugHashMapFrom<'a, K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str("HashMap::from(&")?;

        let mut sorted: Vec<_> = self.0.iter().collect();
        sorted.sort_by_key(|(k, _)| format!("{:?}", k));

        f.debug_list()
            .entries(sorted.iter().map(|(k, v)| (k, v)))
            .finish()?;

        f.write_str(")")
    }
}
