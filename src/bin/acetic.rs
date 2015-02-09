#![feature(env)]
#![feature(core)]
extern crate acetic;

use std::env;

fn main() {
    for arg in env::args().skip(1) {
        println!("{}", arg);
        //println!("Procesing {}", arg);
        //let ref sess = parse::new_parse_sess();
        //let krate = parse::parse_crate_from_file(&Path::new(arg), Vec::new(), sess);
         //debug!("Crate: {:?}", krate);
        //let mut to_scala = ToScala::empty();
        //{
            //let mut printer = AstConverter  { conversion: &mut to_scala };
            //visit::walk_crate(&mut printer, &krate);
        //}
        //let main_path = Path::new("build_dir").join("main.scala");
        //let mut file = File::create(&main_path);
        //to_scala.write_to(&mut file);
        //try_scala_code().unwrap();
    }
}
