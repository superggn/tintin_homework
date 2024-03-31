mod sub_module_1;
mod sub_module_2;

use sub_module_1::*;
use sub_module_2::sub_module_3::*;

fn main() {
    print_a_2_z();
    println!("now sub module 1 is done");
    print_a_2_z_ver2();
    let i = 0;
    let s = 2;
}
