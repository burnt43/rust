extern crate testing;
extern crate ncurses;
use ncurses::Window;

fn main() {
    /*
    testing::time_tests::execute();
    testing::debug_and_fmt_tests::execute();
    testing::lifetime_tests::execute();
    testing::ownership_and_borrowing_tests::execute();
    testing::unknown_tests_1::execute();
    testing::parser_tests::execute();
    testing::macro_tests::execute();
    testing::box_tests::execute();
    testing::hash_map_tests::execute();
    testing::iter_tests::execute();
    testing::drop_tests::execute();
    testing::if_let_tests::execute();
    testing::closure_tests::execute();
    testing::any_tests::execute();
    testing::fmt_tests::execute();
    testing::ffi_tests::execute();
    testing::raw_pointer_tests::execute();
    testing::struct_mem_tests::execute();
    */
    let window: Window = ncurses::initialize_screen();
    window.printw("Hello, World!");
    //window.getch();
    ncurses::end_window();
}
