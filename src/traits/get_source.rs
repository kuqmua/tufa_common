pub trait GetSource {
    fn get_source(&self) -> String;
}

// impl<T> GetSource<Self> for T
// where
//     Self: GetLogWhereWas,
// {
//     fn get_source(
//         &self,
//     ) -> String {

//     }
// }
