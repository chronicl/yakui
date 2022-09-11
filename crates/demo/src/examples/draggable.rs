use yakui::{colored_box, draggable, offset, use_state, Color, Vec2};

use crate::ExampleState;

pub fn run(_state: &mut ExampleState) {
    let pos = use_state(|| Vec2::ZERO);

    offset(pos.get(), || {
        let res = draggable(|| {
            colored_box(Color::RED, Vec2::splat(100.0));
        });

        if let Some(new) = res.dragging {
            pos.set(new.current);
        }
    });
}
