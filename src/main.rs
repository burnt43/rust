extern crate testing;
extern crate libc;
extern crate ncurses;
use ncurses::Window;

fn main() {
    let window: Window = ncurses::initscr();
    window.printw("TEST");
    window.getch();
    ncurses::endwin();
}
