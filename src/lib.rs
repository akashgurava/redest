use redb::RedbValue;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not store table")]
    TableSaveError,
}

#[derive(Debug)]
pub struct Table {}

impl Table {
    pub fn save(&self) -> Result<(), Error> {
        Ok(())
    }

    pub fn save_all(rows: Vec<Table>) -> Result<(), Error> {
        Ok(())
    }
}

impl RedbValue for Table {
    type SelfType<'a> = Table where Self: 'a;

    type AsBytes<'a> = &'a [u8] where Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        todo!()
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        todo!()
    }

    fn type_name() -> redb::TypeName {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
