use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result;

/// Joins strings, just like `.join(" ")` but with [`Display`]
pub(crate) struct JoinDisplay<'s, 'a, S: AsRef<str>>(pub &'s str, pub &'a [S]);

impl<'s, 'a, S: AsRef<str>> Display for JoinDisplay<'s, 'a, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut sep = false;
        for s in self.1 {
            if sep {
                f.write_str(self.0)?;
            }
            sep = true;
            f.write_str(s.as_ref())?;
        }
        Ok(())
    }
}

/// Join extension trait for `&[&str]`
#[cfg(feature = "error-context")]
pub(crate) trait JoinDisplayExt<'s, 'a, S: AsRef<str>> {
    /// Joins strings, just like `.join(" ")` but with [`Display`]
    fn join_display(&'a self, sep: &'s str) -> JoinDisplay<'s, 'a, S>;
}

#[cfg(feature = "error-context")]
impl<'s, 'a, S: AsRef<str>> JoinDisplayExt<'s, 'a, S> for &[S] {
    fn join_display(&'a self, sep: &'s str) -> JoinDisplay<'s, 'a, S> {
        JoinDisplay(sep, self)
    }
}

/// Joins strings, just like `.join(" ")` but with [`Display`]
pub(crate) struct JoinDisplayVec<'s, S: AsRef<str>>(pub &'s str, pub Vec<S>);

impl<'s, S: AsRef<str>> Display for JoinDisplayVec<'s, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        JoinDisplay(self.0, &self.1).fmt(f)
    }
}

/// Join extension trait for [`Vec<&str>`]
pub(crate) trait JoinDisplayVecExt<'s, S: AsRef<str>> {
    /// Joins strings, just like `.join(" ")` but with [`Display`]
    fn join_display(self, sep: &'s str) -> JoinDisplayVec<'s, S>;
}

impl<'s, S: AsRef<str>> JoinDisplayVecExt<'s, S> for Vec<S> {
    fn join_display(self, sep: &'s str) -> JoinDisplayVec<'s, S> {
        JoinDisplayVec(sep, self)
    }
}
