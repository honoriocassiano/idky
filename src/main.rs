use crate::build::Cmake;

mod build;

fn main() {

    Cmake::new("SDL")
        .build();
}
