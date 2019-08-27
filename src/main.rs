use clap::{clap_app, crate_version};

fn main() {
    let _clap = clap_app!(
		mdrend =>
		(version:crate_version!())
		(author:"Nelson Úsuga")
		(about:"Curso de rust")
	).get_matches();

	println!("done");
}
