use ncurses::*;

pub struct CursesHandle {}

impl CursesHandle {
    pub fn create() -> CursesHandle {
        initscr();
        cbreak();
        noecho();
        nodelay(stdscr(), true);
        keypad(stdscr(), true);

        return CursesHandle {};
    }
}
impl Drop for CursesHandle {
    fn drop(&mut self) {
        endwin();
    }
}
