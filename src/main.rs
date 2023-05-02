use crossterm::{
    cursor, queue,
    style::{self, Stylize},
    terminal, ExecutableCommand, Result,
};
use std::io::{stdout, Write};

#[derive(Debug)]
struct Points {
    x: u16,
    y: u16,
}

impl Points {
    fn new(x: u16, y: u16) -> Self {
        return Self { x, y };
    }
}

#[derive(Debug)]
struct Center {
    center_col: u16,
    center_row: u16,
}

impl Center {
    fn new(col: u16, row: u16) -> Self {
        return Self {
            center_col: col / 2,
            center_row: row / 2,
        };
    }
}

fn main() -> Result<()> {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let (col, row) = terminal::size().unwrap();
    let row = row - 1;
    for y in 0..row {
        for x in 0..col {
            if (y == 0 || y == row - 1) || (x == 0 || x == col - 1) {
                queue!(
                    stdout,
                    cursor::MoveTo(x, y),
                    style::PrintStyledContent(".".magenta())
                )?;
            }
        }
    }

    let center_pos: Center = Center::new(col, row);
    for i in 0..col {
        queue!(
            stdout,
            cursor::MoveTo(i, center_pos.center_row),
            style::PrintStyledContent(".".yellow())
        )?;
    }

    for i in 0..row {
        queue!(
            stdout,
            cursor::MoveTo(center_pos.center_col, i),
            style::PrintStyledContent(".".yellow())
        )?;
    }
    draw_circle(center_pos, 20)?;

    let from: Points = Points::new(12, 10);
    let to: Points = Points::new(30, 10);
    draw_line(from, to)?;

    //move cursor to bottom aswell
    queue!(stdout, cursor::MoveTo(col, row - 1))?;
    stdout.flush()?;
    Ok(())
}

fn draw_circle(center: Center, r: u16) -> Result<()> {
    let circle_radius_pos = center.center_row + r;
    queue!(
        stdout(),
        cursor::MoveTo(center.center_col + circle_radius_pos, center.center_row),
        style::PrintStyledContent("x".yellow())
    )?;
    Ok(())
}

fn draw_line(from: Points, to: Points) -> Result<()> {
    let mut x = from.x;
    let mut y = from.y;

    let dx: i32 = i32::from(to.x) - i32::from(from.x);
    let dy: i32 = i32::from(to.y) - i32::from(from.y);

    let mut decision_p: i32 = i32::from(2) * dy - dx;

    while x <= to.x {
        queue!(
            stdout(),
            cursor::MoveTo(x, y),
            style::PrintStyledContent(".".blue())
        )?;

        if decision_p < 0 {
            decision_p = decision_p + 2 * dy
        } else {
            decision_p = decision_p + 2 * dy - 2 * dx;
            y = y + 1;
        }
        x = x + 1;
    }

    Ok(())
}
