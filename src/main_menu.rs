use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::prelude::Color::Rgba;

use crate::main_game::MainGameState;

const DEFAULT_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.21, 0.21, 0.21);

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_state(MainMenuState::Main)
            .add_system_set(
                SystemSet::on_enter(MainMenuState::Main)
                    .with_system(main_menu_setup_system)
            )
            .add_system_set(
                SystemSet::on_update(MainGameState::Menu)
                    .with_system(main_menu_button_action_system)
                    .with_system(main_menu_button_interaction_system)
            )
        ;
    }
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub(crate) enum MainMenuState {
    Main
}

#[derive(Component)]
pub(crate) enum MainMenuButtonAction {
    Play,
    Settings,
    Quit
}

// Tag component used to mark which component is currently selected
#[derive(Component)]
struct SelectedOption;

fn main_menu_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("main_menu_setup_system");
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    // Common style for all buttons on the screen
    let button_style = Style {
        size: Size::new(Val::Px(250.0), Val::Px(65.0)),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.0,
        color: DEFAULT_TEXT_COLOR,
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::all(Val::Auto),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen
        ))
        .with_children(|parent| {
            // Display the game name
            parent.spawn(
                TextBundle::from_section(
                    "RimWorld-rs",
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.0,
                        color: DEFAULT_TEXT_COLOR,
                    },
                ).with_style(Style {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                }),
            );

            // Display three buttons for each action available from the main menu:
            // - new game
            // - settings
            // - quit
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MainMenuButtonAction::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "New Game",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MainMenuButtonAction::Settings,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Settings",
                        button_text_style.clone(),
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: button_style,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MainMenuButtonAction::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        button_text_style
                    ));
                });
        });
    return;
}

fn main_menu_button_action_system(
    interaction_query: Query<
        (&Interaction, &MainMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<State<MainMenuState>>,
    mut game_state: ResMut<State<MainGameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MainMenuButtonAction::Quit => {
                    app_exit_events.send(AppExit)
                },
                MainMenuButtonAction::Play => {
                    // TODO
                },
                MainMenuButtonAction::Settings => {
                    // TODO
                },
            }
        }
    }
}

// This system handles changing all buttons color based on mouse interaction
fn main_menu_button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (
        interaction,
        mut color,
        selected
    ) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}