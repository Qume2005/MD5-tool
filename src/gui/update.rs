use iced::Task;

use super::{lib::{calculate_md5, select_file}, message::Message, state::State, subscription::on_event, Target};

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::OnEvent(id, event) => {
            if let Some(message) = on_event(id, event) {
                return update(state, message);
            }
            Task::none()
        }
        Message::EnterKeyPressed => {
            state.disable_edit_mode();
            update(state, Message::CalculateMD5)
        }
        Message::CalculateMD5 => Task::perform(
            calculate_md5(state.get_data().unwrap_or("".into())), 
            |data| Message::MD5Calculated(data.unwrap_or("".to_string()))
        ),
        Message::MD5Calculated(md5) => {
            state.set_result(md5);
            Task::none()
        }
        Message::SetTarget(target) => {
            state.set_target(target);
            //update(state, Message::CalculateMD5)
            Task::none()
        }
        Message::WindowResize => {
            state.disable_edit_mode();
            Task::none()
        }
        Message::FileDropped(path) => {
            state.disable_edit_mode();
            state.set_target(Target::File(path));
            update(state, Message::CalculateMD5)
        }
        Message::EnableEditMode => {
            state.enable_edit_mode();
            Task::none()
        }
        Message::DisableEditMode => {
            state.disable_edit_mode();
            Task::none()
        }
        Message::ArrowRightPressed => {
            state.disable_edit_mode();
            Task::perform(
                select_file(), 
                |path| path.map_or(
                    Message::SetTarget(Target::Text(String::new())), 
                    |path| Message::SetTarget(Target::File(path))
                )
            )
                .chain(update(state, Message::CalculateMD5))
        }
    }
}