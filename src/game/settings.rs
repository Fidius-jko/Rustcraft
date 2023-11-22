use bevy::{prelude::*, utils::HashMap};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyBinds>()
            .register_type::<KeyBinds>()
            .add_systems(Startup, init_keybinds);
    }
}

#[derive(Resource, Default)]
pub struct Settings;

#[derive(Clone, Reflect)]
pub enum Key {
    Key(KeyCode),
    MouseButton(MouseButton),
}

#[derive(Reflect, Resource, Default)]
#[reflect(Resource)]
pub struct KeyBinds {
    keys: HashMap<String, Key>,
}

impl KeyBinds {
    pub fn get(&self, name: &str) -> Key {
        self.keys
            .get(name)
            .expect(format!("Cloud not found key bind: {}", name).as_str())
            .clone()
    }
    pub fn is_pressed(
        &self,
        name: &str,
        keys: &Res<Input<KeyCode>>,
        mouse: &Res<Input<MouseButton>>,
    ) -> bool {
        match self.get(name) {
            Key::Key(k) => keys.pressed(k),
            Key::MouseButton(b) => mouse.pressed(b),
        }
    }
    pub fn is_just_pressed(
        &self,
        name: &str,
        keys: &Res<Input<KeyCode>>,
        mouse: &Res<Input<MouseButton>>,
    ) -> bool {
        match self.get(name) {
            Key::Key(k) => keys.just_pressed(k),
            Key::MouseButton(b) => mouse.just_pressed(b),
        }
    }
}

fn init_keybinds(mut binds: ResMut<KeyBinds>) {
    binds
        .keys
        .insert("move_forward".to_string(), Key::Key(KeyCode::W));
    binds
        .keys
        .insert("move_backward".to_string(), Key::Key(KeyCode::S));
    binds
        .keys
        .insert("move_left".to_string(), Key::Key(KeyCode::A));
    binds
        .keys
        .insert("move_right".to_string(), Key::Key(KeyCode::D));
    binds
        .keys
        .insert("move_ascent".to_string(), Key::Key(KeyCode::Space));
    binds
        .keys
        .insert("move_descent".to_string(), Key::Key(KeyCode::ShiftLeft));
    binds.keys.insert(
        "grab_cursor".to_string(),
        Key::MouseButton(MouseButton::Right),
    );
    binds
        .keys
        .insert("ungrab_cursor".to_string(), Key::Key(KeyCode::Escape));
}
