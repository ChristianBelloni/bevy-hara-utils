use bevy::{prelude::*, ui::InteractionDisabled};

pub fn on_click_send_message<M: Message + Clone>(message: M) -> impl Scene {
    on(
        move |trigger: On<Pointer<Click>>,
              mut writer: MessageWriter<M>,
              disabled: Query<Has<InteractionDisabled>>| {
            if disabled.get(trigger.entity).ok().is_none_or(|value| !value) {
                writer.write(message.clone());
            }
        },
    )
}
