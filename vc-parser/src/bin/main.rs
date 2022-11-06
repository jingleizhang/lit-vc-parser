fn main() {
    let re = vc_parser::run("return 1-2", None);
    println!("{:?}", re);
}
