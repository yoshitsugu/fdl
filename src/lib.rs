mod circle;
mod note;

use circle::Circle;
use note::Note;
use seed::{prelude::*, *};
use web_sys::MouseEvent;

struct Model {
    notes: Vec<Note>,
}

// Setup a default here, for initialization later.
impl Default for Model {
    fn default() -> Self {
        Self { notes: Vec::new() }
    }
}

// Update
#[allow(clippy::enum_variant_names)]
#[derive(Clone)]
enum Msg {
    AddNote,
    EditNote(usize, String),
    StartDraggingNote(usize, MouseEvent),
    DragNote(usize, MouseEvent),
    EndDraggingNote(usize, (Circle, Circle, Circle)),
    DeleteNote(usize),
}

/// How we update the model
#[allow(clippy::cast_possible_wrap)]
fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AddNote => {
            let count = model.notes.len() as isize;
            model
                .notes
                .push(Note::new("".to_string(), 10 + count, 100 + count));
        }
        Msg::EditNote(index, string) => {
            let old_note = model.notes[index].clone();
            model.notes[index] = Note {
                description: string,
                ..old_note
            }
        }
        Msg::StartDraggingNote(index, ev) => {
            let old_note = model.notes[index].clone();
            model.notes[index] = Note {
                original_x: old_note.x,
                original_y: old_note.y,
                client_x: ev.client_x() as isize,
                client_y: ev.client_y() as isize,
                dragging: true,
                ..old_note
            }
        }
        Msg::DragNote(index, ev) => {
            if model.notes[index].dragging {
                let old_note = model.notes[index].clone();
                model.notes[index] = Note {
                    x: std::cmp::max(
                        -70,
                        old_note.original_x + ev.x() as isize - old_note.client_x,
                    ),
                    y: std::cmp::max(
                        -70,
                        old_note.original_y + ev.y() as isize - old_note.client_y,
                    ),
                    ..old_note
                }
            }
        }
        Msg::EndDraggingNote(index, fdl) => {
            let count = model.notes.len();
            for i in 0..count {
                let old_note = model.notes[i].clone();
                model.notes[i] = Note {
                    client_x: 0,
                    client_y: 0,
                    dragging: false,
                    ..old_note
                };
            }
            model.notes[index].set_fdl(fdl);
        }
        Msg::DeleteNote(index) => {
            model.notes.remove(index);
        }
    }
}

// View
/// The top-level component we pass to the virtual dom.
fn view(model: &Model) -> impl View<Msg> {
    let fun = Circle::new(795, 520, 500);
    let done = Circle::new(520, 996, 500);
    let learn = Circle::new(1020, 996, 500);
    div![
        div![
            class! { "background" },
            background_style(learn),
            div![
                circle_style(fun),
                class! { "fun" },
                div![circle_text_style(), fun_text_style(), "Fun"]
            ],
            div![
                circle_style(done),
                class! { "done" },
                div![circle_text_style(), done_text_style(), "Done"]
            ],
            div![
                circle_style(learn),
                class! { "learn" },
                div![circle_text_style(), learn_text_style(), "Learn"]
            ]
        ],
        button![
            new_button_style(),
            simple_ev(Ev::Click, Msg::AddNote),
            "Add Note"
        ],
        model.notes.iter().enumerate().map(|(i, note)| {
            div![
                note_style(note),
                style! {
                    St::Top => px(note.y)
                    St::Left => px(note.x)
                },
                textarea![
                    style! {
                        St::Width => "100%",
                        St::Height => "100%",
                        St::Border => "none",
                        St::VerticalAlign => "center"
                        St::Background => "transparent",
                        St::Resize => "none"
                    },
                    attrs! {
                        At::Placeholder => "Note"
                        At::Value => note.description
                    },
                    input_ev(Ev::Input, move |input: String| Msg::EditNote(i, input)),
                ],
                div![
                    icons_style(),
                    if note.fdl.fun {
                        div![icon_style(), "F"]
                    } else {
                        empty![]
                    },
                    if note.fdl.done {
                        div![icon_style(), "D"]
                    } else {
                        empty![]
                    },
                    if note.fdl.learn {
                        div![icon_style(), "L"]
                    } else {
                        empty![]
                    },
                    div![
                        delete_note_style(),
                        "x",
                        mouse_ev(Ev::Click, move |_| Msg::DeleteNote(i))
                    ]
                ],
                mouse_ev("mousedown", move |ev| Msg::StartDraggingNote(i, ev)),
                mouse_ev("mousemove", move |ev| Msg::DragNote(i, ev)),
                mouse_ev("mouseup", move |_| Msg::EndDraggingNote(
                    i,
                    (fun, done, learn)
                )),
            ]
        }),
    ]
}

// Styles
fn background_style(learn: Circle) -> Style {
    style! {
      St::Width => px(learn.center.x + learn.radius + 40),
      St::Height => px(learn.center.y + learn.radius + 40),
      St::Padding => px(20)
      St::Position => "relative"
    }
}

fn circle_style(circle: Circle) -> Style {
    style! {
      St::Width => px(circle.radius * 2),
      St::Height => px(circle.radius * 2),
      St::BorderRadius => px(circle.radius),
      St::Border => "2px solid #999",
      St::Position => "absolute"
      St::Left => px(circle.center.x - circle.radius);
      St::Top => px(circle.center.y - circle.radius);
    }
}

fn new_button_style() -> Style {
    style! {
        St::Position => "absolute";
        St::Top => px(5);
        St::Left => px(5);
    }
}

fn circle_text_style() -> Style {
    style! {
      St::FontSize => px(24),
      St::FontWeight => "bold"
    }
}

fn fun_text_style() -> Style {
    style! {
       St::TextAlign => "center"
    }
}

fn done_text_style() -> Style {
    style! {
       St::TextAlign => "center"
       St::PaddingBottom => "5px"
    }
}

fn learn_text_style() -> Style {
    style! {
       St::TextAlign => "center"
       St::PaddingBottom => "5px"
    }
}

fn note_style(note: &Note) -> Style {
    style! {
       St::Position => "absolute",
       St::Background => "#ffeb71"
       St::Border => "none"
       St::Width => px(note.width)
       St::Height => px(note.height)
       St::Padding => px(10)
       St::Filter => "drop-shadow(0.5px 0.5px 0.7px rgba(0,0,0,.2))"
    }
}

fn icons_style() -> Style {
    style! {
        St::Position => "absolute",
        St::Display => "flex",
        St::AlignItems => "center",
        St::Top => 0,
        St::Right => 0,
    }
}

fn icon_style() -> Style {
    style! {
        St::Width => px(16),
        St::Height => px(16),
        St::BorderRadius => px(8),
        St::Background => "#FF8866"
        St::FontSize => px(11),
        St::Display => "flex",
        St::AlignItems => "center",
        St::JustifyContent => "center",
        St::Color => "#fff",
        St::FontWeight => "bold",
    }
}

fn delete_note_style() -> Style {
    style! {
        St::Width => px(16),
        St::Height => px(16),
        St::Cursor => "pointer",
        St::FontSize => px(12),
        St::Display => "flex",
        St::AlignItems => "center",
        St::JustifyContent => "center",
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
