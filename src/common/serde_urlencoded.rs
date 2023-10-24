pub trait SerdeUrlencodedParameter {
    fn serde_urlencoded_parameter(self) -> std::string::String;
}

impl SerdeUrlencodedParameter for std::string::String {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        self
    }
}

impl SerdeUrlencodedParameter for Vec<std::string::String> {
    fn serde_urlencoded_parameter(self) -> std::string::String {
        let mut handle = self.into_iter().fold(std::string::String::default(), |mut acc, element| {
            acc.push_str(&format!("{element},"));
            acc
        });
        handle.pop();
        handle
    }
}