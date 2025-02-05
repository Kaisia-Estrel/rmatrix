use std::{
    collections::{HashMap, HashSet},
    io,
    time::Duration,
};

use crossterm::style::Stylize;
use crossterm::{
    cursor::{self, MoveTo},
    event::{self, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{rngs::StdRng, Rng, SeedableRng};

struct Tracer {
    row: f32,
    col: u16,
    len: usize,
    speed: f32,
}

impl Tracer {
    fn new(row: f32, col: u16, len: usize, speed: f32) -> Self {
        Self {
            row,
            col,
            len,
            speed,
        }
    }
}

fn main() -> io::Result<()> {
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut tracers: HashMap<usize, Tracer> = HashMap::new();
    let mut tracer_count = 0;
    let (mut cols, mut rows) = terminal::size()?;
    let mut screenbuf: Vec<Vec<char>>;
    execute!(io::stdout(), cursor::Hide)?;
    execute!(io::stdout(), terminal::DisableLineWrap)?;
    loop {
        screenbuf = vec![vec![' '; cols as usize]; rows as usize];
        tracers.insert(
            tracer_count,
            Tracer::new(
                0.0,
                rand::random::<u16>() % cols,
                rand::random::<usize>() % 10 + 5,
                rand::random::<f32>() % 1.0 + 0.3,
            ),
        );
        tracers.insert(
            tracer_count,
            Tracer::new(
                0.0,
                rand::random::<u16>() % cols,
                rand::random::<usize>() % 10 + 5,
                rand::random::<f32>() % 1.0 + 0.3,
            ),
        );
        tracer_count += 1;

        if event::poll(Duration::from_millis(20))? {
            match event::read()? {
                event::Event::Key(KeyEvent {
                    code: KeyCode::Esc | KeyCode::Char('q'),
                    ..
                }) => break,
                event::Event::Resize(new_cols, new_rows) => {
                    cols = new_cols;
                    rows = new_rows;
                    tracers.clear();
                }
                _ => (),
            }
        }

        let mut to_remove: Vec<usize> = Vec::new();
        let mut heads: HashSet<(u16, u16)> = HashSet::new();
        for (i, tracer) in tracers.iter_mut() {
            tracer.row += tracer.speed;

            let row = tracer.row as u16;

            if row >= (tracer.len as u16) && (row - tracer.len as u16) >= rows {
                to_remove.push(*i);
            } else {
                heads.insert((tracer.col, row));
                for j in 0..tracer.len {
                    if (j == 0 || row > 0 && j <= row.into()) && row as usize - j < rows as usize {
                        screenbuf[row as usize - j][tracer.col as usize] =
                            make_char(*i, tracer.col, row - j as u16);
                    }
                }
            }
        }

        let bufstr = screenbuf
            .iter()
            .enumerate()
            .map(|(row, x)| {
                x.into_iter()
                    .enumerate()
                    .map(|(col, c)| {
                        if heads.contains(&(col as u16, row as u16)) {
                            c.white().bold().to_string()
                        } else {
                            c.green().to_string()
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        execute!(io::stdout(), SetForegroundColor(Color::Green), MoveTo(0, 0), Print(bufstr))?;

        for i in to_remove {
            tracers.remove(&i);
        }
    }
    execute!(io::stdout(), LeaveAlternateScreen)
}

fn make_char(i: usize, col: u16, row: u16) -> char {
    let mut rng = StdRng::seed_from_u64(row as u64 * col as u64 * i as u64);
    let char_set = ('0'..'9')
        .chain('a'..'z')
        .chain('A'..'Z')
        .chain('ﾀ'..'ﾏ')
        .chain("@#$%&".chars())
        .collect::<Vec<char>>();
    char_set[rng.gen_range(0..char_set.len())]
}
