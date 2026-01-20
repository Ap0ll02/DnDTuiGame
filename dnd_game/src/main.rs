use std::time::{Duration, Instant};

use crossterm::{cursor, event::KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    widgets::Padding,
};
mod styles;

enum Scene {
    MainMenu,
    Scene1(u8),
}

struct App {
    scene: Scene,
}

struct Input {
    text: String,
    finalized: bool,
    last_blink: Instant,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App {
        scene: Scene::MainMenu,
    };

    let mut input = Input {
        text: String::new(),
        finalized: false,
        last_blink: Instant::now(),
    };

    loop {
        let now = Instant::now();
        if now.duration_since(input.last_blink) >= Duration::from_millis(500) {
            // Put cursor shit here
            input.last_blink = Instant::now();
        }

        terminal.draw(|f| render(f, &app, &mut input))?;
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            match app.scene {
                Scene::MainMenu => {
                    if key.code == crossterm::event::KeyCode::Enter {
                        app.scene = Scene::Scene1(0);
                    }
                    if key.code == crossterm::event::KeyCode::Char('q') {
                        break Ok(());
                    }
                }
                Scene::Scene1(_) => {
                    if key.code == crossterm::event::KeyCode::Esc {
                        app.scene = Scene::MainMenu;
                    }
                    match key.code {
                        crossterm::event::KeyCode::Char(c) => {
                            if !input.finalized {
                                input.text.push(c)
                            }
                        }
                        crossterm::event::KeyCode::Backspace => {
                            input.text.pop();
                        }
                        crossterm::event::KeyCode::Enter => {
                            input.finalized = true;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn render(frame: &mut Frame, app: &App, input: &mut Input) {
    match app.scene {
        Scene::MainMenu => render_main_menu(frame),
        Scene::Scene1(_) => render_scene_one(frame, input),
    }
}

fn render_main_menu(frame: &mut Frame) {
    use ratatui::widgets::{Block, Borders, Paragraph};

    let block = Block::new()
        .borders(Borders::ALL)
        .title("Dungeons & Dragons")
        .padding(Padding::proportional(10));

    let text = Paragraph::new("Press Enter to Start\nPress Q to Quit")
        .block(block)
        .style(styles::TITLE)
        .centered();

    frame.render_widget(text, frame.area());
}

fn render_scene_one(frame: &mut Frame, input: &mut Input) {
    use ratatui::layout::Alignment;
    use ratatui::widgets::{Block, Borders, Paragraph};
    let block = Block::new()
        .borders(Borders::ALL)
        .title("Dungeons & Dragons -- Create A Character")
        .padding(Padding::proportional(10));
    let inner = block.inner(frame.area());
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Length(3)])
        .split(inner);

    let p = Paragraph::new("Create Your Character\nPress Esc For Main Menu")
        .alignment(Alignment::Center);

    let mut cursor_text = input.text.clone();
    let mut style = styles::TITLE;

    if !input.finalized {
        style = styles::NORMAL;
        cursor_text.push('|');
    }

    let name_field = Paragraph::new(cursor_text.as_str())
        .block(Block::default().style(style).title("Character Name"));

    frame.render_widget(block, frame.area());
    frame.render_widget(p, chunks[0]);
    frame.render_widget(name_field, chunks[1]);
}
