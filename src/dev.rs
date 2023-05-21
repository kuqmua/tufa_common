// #[derive(Clone, Debug, valuable::Valuable)]
// struct User {
//     name: String,
//     age: u32,
//     something: Vec<bool>,
//     address: Address,
// }

// #[derive(Clone, Debug, valuable::Valuable)]
// struct Address {
//     country: String,
//     city: String,
//     street: String,
// }

pub fn dev() {
    // let user = User {
    //     name: "Arwen Undomiel".to_string(),
    //     age: 3000,
    //     something: vec![true, false],
    //     address: Address {
    //         country: "Middle Earth".to_string(),
    //         city: "Rivendell".to_string(),
    //         street: "leafy lane".to_string(),
    //     },
    // };
    // tracing::error!(valuable = false, user = ?user);
}


///builder pattern example
// #[derive(Debug)]
// pub struct PortHandle {
// 	port: u16,
// }
// #[derive(Default, Clone)]
// pub struct Sealed;
// #[derive(Default, Clone)]
// pub struct NotSealed;

// #[derive(Default, Clone)]
// pub struct NoPort;
// #[derive(Default, Clone)]
// pub struct Port(u16);

// #[derive(Default, Clone)]
// pub struct PortHandleBuilder<PortGeneric, SealGeneric> {
// 	port: PortGeneric,
// 	marker_seal: core::marker::PhantomData<SealGeneric>,
// }
// impl PortHandleBuilder<NoPort, NotSealed> {
// 	pub fn new() -> Self {
// 		PortHandleBuilder::default()
// 	}
// }
// impl<PortGeneric> PortHandleBuilder<PortGeneric, NotSealed> {
// 	pub fn seal(self) -> PortHandleBuilder<PortGeneric, Sealed> {
// 		PortHandleBuilder {
// 			port: self.port,
// 			marker_seal: core::marker::PhantomData,
// 		}
// 	}
// }
// impl<SealedGeneric> PortHandleBuilder<Port, SealedGeneric> {
// 	pub fn build(self) -> Result<PortHandle, std::string::String> {
// 		Ok(PortHandle {
// 			port: self.port.0,
// 		})
// 	}
// }
// impl<PortGeneric> PortHandleBuilder<PortGeneric, NotSealed> {
// 	pub fn port(
// 		self,
// 		port: u16,
// 	) -> PortHandleBuilder<Port, NotSealed> {
// 		PortHandleBuilder {
// 			port: Port(port),
// 			marker_seal: core::marker::PhantomData,
// 		}
// 	}
// }

// fn something() {
// 	let req_builder = PortHandleBuilder::new()
// 		.port(3000);

// 	let req_builder = req_builder
// 		.seal();

// 	let req = req_builder.build().expect("cannot build1");
// 	println!("{req:#?}");
// }