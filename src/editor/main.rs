#![deny(warnings)]

extern crate orbclient;
extern crate orbtk;

use orbtk::{Action, Button, Menu, Point, Rect, Separator, TextBox, Window};
use orbtk::traits::{Click, Place, Text};

use std::{cmp, env};
use std::fs::File;
use std::io::{Read, Write};

fn main(){
    let path_option = env::args().nth(1);

    let title = if let Some(ref path) = path_option {
        format!("{} - Editor", path)
    } else {
        format!("Editor")
    };

    let (display_width, display_height) = orbclient::get_display_size().expect("viewer: failed to get display size");
    let (width, height) = (cmp::min(800, display_width * 4/5), cmp::min(576, display_height * 4/5));

    let mut window = Window::new(Rect::new(-1, -1, width, height), &title);

    let text_box = TextBox::new();
    text_box.position(0, 16)
        .size(width, height - 16);
    window.add(&text_box);

    if let Some(ref path) = path_option {
        match File::open(path) {
            Ok(mut file) => {
                let mut text = String::new();
                match file.read_to_string(&mut text) {
                    Ok(_) => text_box.text.set(text),
                    Err(err) => println!("Failed to read {}: {}", path, err)
                }
            },
            Err(err) => println!("Failed to open {}: {}", path, err)
        }
    }

    let menu = Menu::new("File");
    menu.position(0, 0).size(32, 16);

    let open_action = Action::new("Open");
    open_action.on_click(|_action: &Action, _point: Point| {
        println!("Open");
    });
    menu.add(&open_action);

    menu.add(&Separator::new());

    let save_action = Action::new("Save");
    let save_path_option = path_option.clone();
    let save_text_box = text_box.clone();
    save_action.on_click(move |_action: &Action, _point: Point| {
        println!("Save");
        if let Some(ref path) = save_path_option {
            println!("Create {}", path);
            match File::create(path) {
                Ok(mut file) => {
                    let text = save_text_box.text.borrow();
                    match file.write(&text.as_bytes()) {
                        Ok(_) => match file.set_len(text.len() as u64) {
                            Ok(_) => println!("Successfully saved {}", path),
                            Err(err) => println!("Failed to truncate {}: {}", path, err)
                        },
                        Err(err) => println!("Failed to write {}: {}", path, err)
                    }
                },
                Err(err) => println!("Failed to open {}: {}", path, err)
            }
        } else {
            println!("Need to create file!");
        }
    });
    menu.add(&save_action);

    let save_as_action = Action::new("Save As");
    let save_as_path_option = path_option.clone();
    save_as_action.on_click(move |_action: &Action, _point: Point| {
        println!("Save As");
        let mut window = Window::new(Rect::new(100, 100, 320, 32), "Save As");

        let text_box = TextBox::new();
        text_box.position(0, 0)
            .size(320, 16);
        window.add(&text_box);

        if let Some(ref path) = save_as_path_option {
            text_box.text.set(path.clone());
        }

        {
            let window_cancel = &mut window as *mut Window;
            let button = Button::new();
            button.position(0, 16)
                .size(320/2, 16)
                .text("Cancel")
                .on_click(move |_button: &Button, _point: Point| {
                    unsafe { (&mut *window_cancel).close(); }
                });
            window.add(&button);
        }

        {
            let window_save_as = &mut window as *mut Window;
            let button = Button::new();
            button.position(320/2, 16)
                .size(320/2, 16)
                .text("Save As")
                .on_click(move |_button: &Button, _point: Point| {
                    println!("Save {}", text_box.text.get());
                    unsafe { (&mut *window_save_as).close(); }
                });
            window.add(&button);
        }

        window.exec();
    });
    menu.add(&save_as_action);

    menu.add(&Separator::new());

    let close_action = Action::new("Close");
    let window_close = &mut window as *mut Window;
    close_action.on_click(move |_action: &Action, _point: Point| {
        println!("Close");
        unsafe { (&mut *window_close).close(); }
    });
    menu.add(&close_action);

    window.add(&menu);

    window.exec();
}
