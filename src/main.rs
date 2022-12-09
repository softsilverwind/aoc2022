// Expands to:
//
// mod d1;
// mod d2;
// ...
// fn dispatch(arg: &str)
// {
//     match arg {
//         "1a" => d1::simple(),
//         "1b" => d1::complex(),
//         ...
//     }
// }
macros::gen_aoc_dispatch!{days = 9} 

fn main()
{
    let arg = std::env::args().skip(1).next().expect("Missing problem argument");
    aoc_dispatch(arg.as_ref());
}
