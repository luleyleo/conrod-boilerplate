#[macro_use]
extern  crate conrod;
#[macro_use]
extern  crate conrod_derive;
extern  crate ttf_noto_sans;

mod eventloop;
mod boiler;
mod components;

fn main() {
	boiler::boil();
}
