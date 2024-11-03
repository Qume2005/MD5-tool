use iced::alignment::Horizontal;
use iced::widget::text::Shaping;
use iced::widget::{Column, Container, Text, TextInput};
use iced::{Color, Element};
use iced::Length;

use super::message::Message;
use super::state::State;

pub fn view(state: &State) -> Element<Message> {
    match state.get_edit_mode_status() {
        false => non_edit_mode_view(state),
        true => edit_mode_view(state),
    }
}

fn non_edit_mode_view(state: &State) -> Element<Message> {
    let content = Column::new()
        .spacing(10) 
        .align_x(Horizontal::Center)
        .push(
            Text::new(state.get_target())
                .shaping(Shaping::Advanced)
                .size(48),
        )
        .push(
            Text::new(state.get_result())
                .shaping(Shaping::Advanced)
                .size(16),
        )
        .push(
            Text::new(state.get_prev_result() + "(prev)")
                .shaping(Shaping::Advanced)
                .size(16)
                .color(match state.get_result() == state.get_prev_result() {
                    false => Color::from_rgb(5., 0., 0.),
                    true => Color::from_rgb(0., 5., 0.),
            }),
        );

    let container = Container::new(content)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(20);

    container.into()
}

fn edit_mode_view(state: &State) -> Element<Message> {
    let content = Column::new()
        .spacing(10) 
        .align_x(Horizontal::Center)
        .push(
            TextInput::new("input...", state.get_target().as_str())
                .on_input(|text| Message::SetTarget(super::Target::Text(text)))
                .on_submit(Message::DisableEditMode)
                .size(48)
        )
        .push(
            Text::new(state.get_result())
                .shaping(Shaping::Advanced)
                .size(16)
        )
        .push(
            Text::new(state.get_prev_result() + "(prev)")
                .shaping(Shaping::Advanced)
                .size(16)
                .color(match state.get_result() == state.get_prev_result() {
                    false => Color::from_rgb(5., 0., 0.),
                    true => Color::from_rgb(0., 5., 0.),
            }),
        );

    let container = Container::new(content)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .padding(20);

    container.into()
}
