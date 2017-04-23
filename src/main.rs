#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
// #![deny(clippy_pedantic)]
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate serde_json;
extern crate termion;

mod core;
mod input;
mod line;
mod update;
mod screen;

use std::env;

use core::Core;
use input::Input;
use screen::Screen;

fn main() {
    log4rs::init_file("log_config.yaml", Default::default()).unwrap();
    let xi = clap_app!(xi =>
        (about: "The Xi Editor")
        (@arg core: -c --core +takes_value "Specify binary to use for the backend")
        (@arg file: +required "File to edit")
    );
    let matches = xi.get_matches();
    let core_exe = matches.value_of("core").unwrap_or("xi-core");
    let file = matches.value_of("file").unwrap();
    let mut core = Core::new(core_exe, file);
    let mut screen = Screen::new();
    let mut input = Input::new();
    input.run();
    screen.init();
    core.scroll(0, screen.size.1 as u64 - 2);

    loop {
        if let Ok(event) = input.try_recv() {
            match event {
                termion::event::Event::Key(key) => {
                    match key {
                        termion::event::Key::Char(c) => {
                            core.char(c);
                        },
                        termion::event::Key::Ctrl(c) => {
                            match c {
                                'c' => {
                                    info!("received ^C: exiting");
                                    return;
                                },
                                'w' => {
                                    info!("received ^W: writing current file");
                                    core.save();
                                },
                                _ => {}
                            }
                        },
                        termion::event::Key::Backspace => {
                            core.del();
                        },
                        termion::event::Key::Left => {
                            core.left();
                        },
                        termion::event::Key::Right => {
                            core.right();
                        },
                        termion::event::Key::Up => {
                            core.up();
                        },
                        termion::event::Key::Down => {
                            core.down();
                        },
                        termion::event::Key::PageUp => {
                            core.page_up();
                        },
                        termion::event::Key::PageDown => {
                            core.page_down();
                        },
                        _ => {
                            error!("unsupported key event");
                        }
                    }
                },
                termion::event::Event::Mouse(e) => {
                    match e {
                        termion::event::MouseEvent::Press(_, y, x) => {
                            core.click(x as u64 - 1, y as u64 - 1);
                        },
                        termion::event::MouseEvent::Release(_, _) => {},
                        termion::event::MouseEvent::Hold(y, x) => {
                            core.drag(x as u64 - 1, y as u64 - 1);
                        },
                    }
                },
                _ => {
                    error!("unsupported event");
                }
            }
        } else {
            screen.update(&mut core);
        }
    }
}
