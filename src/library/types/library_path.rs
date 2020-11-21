use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use diesel::sql_types::Text;
use diesel::backend::Backend;
use diesel::serialize;
use diesel::deserialize;

/// LibraryPath is a simple wrapper around PathBuf that allows us to use PathBuf inside of Diesel.
/// We can't implement Diesel's traits on builtin types like PathBuf, so this simple wrapper allows
/// us to implement the traits we need to on LibraryPath instead, while Deref allows the type to
/// act just like PathBuf for most purposes.
#[derive(Debug, Clone, PartialEq, AsExpression, FromSqlRow)]
#[sql_type="Text"]
pub struct LibraryPath(pub PathBuf);

impl Deref for LibraryPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LibraryPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0       
    }
}

// Allow us to use From and Into for LibraryPath just like we do for PathBuf
impl<T> From<T> for LibraryPath
where
    T: Into<PathBuf>
{
    fn from(s: T) -> Self {
        LibraryPath(s.into())
    }
}

impl<DB> serialize::ToSql<Text, DB> for LibraryPath
where
    DB: Backend,
    str: serialize::ToSql<Text, DB>,
{
    fn to_sql<W: std::io::Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
        self
            .to_str()
            .unwrap()
            .to_sql(out)
    }
}

impl<DB> deserialize::FromSql<Text, DB> for LibraryPath
where
    DB: Backend,
    String: deserialize::FromSql<Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
        String::from_sql(bytes).map(|s| LibraryPath::from(s))
    }
}
