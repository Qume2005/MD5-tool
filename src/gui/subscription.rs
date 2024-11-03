use iced::{event, keyboard::{self, key::{Code, Physical}}, mouse, window::{self, Id}, Event, Subscription};

use super::{message::Message, state::State};

pub fn subscription(_: &State) -> Subscription<Message> {
    event::listen_with(|event, _, id| Some(Message::OnEvent(id, event)))
    
}

pub fn on_event(id: Id, event: Event) -> Option<Message> {
    match event {
        Event::Keyboard(event) => on_keyboard(event),
        Event::Window(event) => on_window(id, event),
        Event::Mouse(event) => on_mouse(event),
        _ => None,
    }
}

fn on_keyboard(event: keyboard::Event) -> Option<Message> {
    match event {
        keyboard::Event::KeyPressed { physical_key, .. } => match physical_key {
            Physical::Code(code) => match code {
                Code::Enter => Some(Message::EnterKeyPressed),
                Code::ArrowRight => Some(Message::ArrowRightPressed),
                _ => None,
            },
            _ => None,
        }
        _ => None,
    }
}

fn on_window(_: Id, event: window::Event) -> Option<Message> {
    match event {
        window::Event::Resized(..) => Some(Message::WindowResize),
        window::Event::FileDropped(path) => Some(Message::FileDropped(path)),
        _ => None,
    }
}

fn on_mouse(event: mouse::Event) -> Option<Message> {
    match event {
        mouse::Event::ButtonReleased(button) => match button {
            mouse::Button::Left => Some(Message::EnableEditMode),
            _ => None,
        }
        _ => None,
    }
}