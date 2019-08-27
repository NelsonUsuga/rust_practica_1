use clap::{clap_app, crate_version};

fn main() {
    let _clap = clap_app!(
		rust_practica_1 =>
		(version:crate_version!())
		(author:"Nelson Úsuga")
		(about:"Curso de rust")
		(@arg input: +required "Mande un valor")
	).get_matches();

	println!("Input = {:?}", _clap.value_of("input"));

	println!("done");
}
