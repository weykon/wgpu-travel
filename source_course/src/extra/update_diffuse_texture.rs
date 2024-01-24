use crate::{state::State, texture};

pub fn exec(app_state: &mut State) {
    let diffuse_bytes = include_bytes!("../../../assets/happy-tree-cartoon.png");
    app_state.diffuse_texture = texture::Texture::from_bytes(
        &app_state.device,
        &app_state.queue,
        diffuse_bytes,
        "../../assets/happy-tree-cartoon.png",
    )
    .unwrap();
}
