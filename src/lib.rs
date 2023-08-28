// use redb::RedbValue;

// #[derive(thiserror::Error, Debug)]
// pub enum Error {
//     #[error("Could not store table")]
//     TableSaveError,
// }

// #[derive(Debug)]
// pub struct Row {}

// impl Row {
//     pub fn save(&self) -> Result<(), Error> {
//         Ok(())
//     }
// }

// impl RedbValue for Row {
//     type SelfType<'a> = Row where Self: 'a;

//     type AsBytes<'a> = &'a [u8] where Self: 'a;

//     fn fixed_width() -> Option<usize> {
//         None
//     }

//     fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
//     where
//         Self: 'a,
//     {
//         todo!()
//     }

//     fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
//     where
//         Self: 'a,
//         Self: 'b,
//     {
//         todo!()
//     }

//     fn type_name() -> redb::TypeName {
//         todo!()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use redb::Database;

//     #[test]
//     fn save() {
//         let db = Database::create("./hello.redb").unwrap();
//     }
// }
