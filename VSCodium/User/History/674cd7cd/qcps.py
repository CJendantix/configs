import curses as curses

stdscr = curses.initscr()
curses.noecho()
curses.cbreak()
stdscr.keypad(True)

stdscr.addstr(0,0,"this sucks")
stdscr.getkey()

curses.nocbreak()
stdscr.keypad(False)
curses.echo()
curses.endwin()