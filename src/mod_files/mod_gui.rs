use crate::mod_files::mod_guess_number::guess_number_game_gui;
use druid::widget::{Button, Container, Flex, Label, ViewSwitcher};
use druid::{AppLauncher, Color, Data, Lens, Widget, WindowDesc};
use rand::Rng;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    current_view: StateGui,
    secret_number: Option<u32>,
}

#[derive(Clone, Data, PartialEq, Copy)]
pub enum StateGui {
    Menu,
    Game,
}

fn build_ui() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |data: &AppState, _env| data.current_view,
        |selector, _data, _env| match selector {
            StateGui::Menu => Box::new(menu_ui()),
            StateGui::Game => Box::new(game_ui()),
        },
    )
}

fn menu_ui() -> impl Widget<AppState> {
    Flex::column().with_flex_child(
        Container::new(
            Flex::column()
                .with_flex_child(Label::new("Hello world"), 1.0)
                .with_flex_child(
                    Button::new("1. Guess number game").on_click(
                        |_ctx, data: &mut AppState, _env| {
                            data.current_view = StateGui::Game;
                            data.secret_number = Some(rand::thread_rng().gen_range(1..=100));
                        },
                    ),
                    1.0,
                ),
        )
        .border(Color::grey(0.6), 2.0),
        1.0,
    )
}

fn game_ui() -> impl Widget<AppState> {
    let secret_label = Label::new(|data: &AppState, _env: &_| {
        format!("The secret number is: {:?}", data.secret_number)
    });

    Flex::column()
        .with_flex_child(
            Container::new(
                Flex::column()
                    .with_flex_child(Label::new("Hello world"), 1.0)
                    .with_flex_child(
                        Button::new("Menu").on_click(|_ctx, data: &mut AppState, _env| {
                            data.current_view = StateGui::Menu
                        }),
                        1.0,
                    ),
            )
            .border(Color::grey(0.6), 2.0),
            1.0,
        )
        .with_flex_child(secret_label, 1.0)
}

pub fn launch_gui() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My first Druid App");
    let initial_data = AppState {
        current_view: StateGui::Menu,
        secret_number: None,
    };

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
