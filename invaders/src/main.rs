use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use invaders::frame::{new_frame, Drawable};
use invaders::player::Player;
use invaders::{render};
use mpsc::channel;
use rusty_audio::Audio;
use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};
use invaders::invaders::Invaders;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize
    let mut audio = Audio::new();

    audio.add("explode", "./assets/sounds/explode.wav");
    audio.add("lose", "./assets/sounds/lose.wav");
    audio.add("move", "./assets/sounds/move.wav");
    audio.add("pew", "./assets/sounds/pew.wav");
    audio.add("startup", "./assets/sounds/startup.wav");
    audio.add("win", "./assets/sounds/win.wav");

    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        while let Ok(curr_frame) = render_rx.recv() {
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();

    // Game loop
    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    _ => {}
                }
            }
        }

        // Updates
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }
        if player.detect_hits(&mut invaders) {
            audio.play("explode");
        }

        // Draw & render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
        
        // Win condition
        if invaders.all_killed() {
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom() {
            audio.play("lose");
            break 'gameloop;
        }
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();

    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
