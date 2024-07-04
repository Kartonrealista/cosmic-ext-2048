// SPDX-License-Identifier: GPL-3.0-only
use cosmic::widget::list::container;
use rand::seq::IteratorRandom;
use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashMap;
use std::time;

use crate::fl;
use cosmic::app::{Command, Core};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::{event, keyboard, Alignment, Event, Length, Subscription};
use cosmic::widget::{self, button, menu, text, text_input, Grid, Row, Text};
use cosmic::{cosmic_theme, theme, Application, ApplicationExt, Element, Renderer, Theme};
mod widget_colors;

const REPOSITORY: &str = "https://github.com/Kartonrealista/cosmic-ext-2024";

struct Game {
    menu: Menu,
    board: Board,
    has_ended: bool,
}
impl Game {
    fn new() -> Game {
        let game = Game {
            board: Board::new(4, 4),
            has_ended: false,
            menu: Menu {
                width_inptut: String::from("4"),
                height_inptut: String::from("4"),
                width: 4,
                height: 4,
                start_pressed: false,
            },
        };
        game
    }
}
fn pair_to_index(i: usize, j: usize, width: usize) -> usize {
    j + i * width
}

struct Menu {
    width_inptut: String,
    height_inptut: String,
    width: usize,
    height: usize,
    start_pressed: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct Board(Vec<Tile>);
impl Board {
    const TWO_OR_FOUR: [usize; 10] = [2, 2, 2, 2, 2, 2, 2, 2, 2, 4];
    fn new(height: usize, width: usize) -> Board {
        let mut out: Vec<Tile> = (0..(width * height))
            .map(|id| Tile {
                tilecontent: None,
                id,
            })
            .collect();
        let mut ids: Vec<usize> = (0..(width * height)).collect();
        rand::seq::SliceRandom::shuffle(ids.as_mut_slice(), &mut thread_rng());
        ids.iter().take(2).for_each(|&id| {
            out[id].tilecontent = Some(*Self::TWO_OR_FOUR.choose(&mut thread_rng()).unwrap())
        });
        Board(out)
    }
    fn move_tile_content(&mut self, direction: keyboard::Key, height: usize, width: usize) {
        let old_board = self.clone();
        let mut previous = Tile {
            tilecontent: None,
            id: 0,
        };
        match direction {
            keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => {
                self.collapse_left(height, width);
                (0..height).for_each(|h| {
                    (0..width).for_each(|w| {
                        self.merge_neighbouring(h, w, &mut previous, width, height, &direction);
                    });
                });
                self.collapse_left(height, width);
            }
            keyboard::Key::Named(keyboard::key::Named::ArrowRight) => {
                self.collapse_right(height, width);
                (0..height).for_each(|h| {
                    (0..width).rev().for_each(|w| {
                        self.merge_neighbouring(h, w, &mut previous, width, height, &direction)
                    });
                });
                self.collapse_right(height, width);
            }
            keyboard::Key::Named(keyboard::key::Named::ArrowUp) => {
                self.collapse_up(height, width);
                (0..width).for_each(|w| {
                    (0..height).for_each(|h| {
                        self.merge_neighbouring(h, w, &mut previous, width, height, &direction)
                    });
                });
                self.collapse_up(height, width);
            }
            keyboard::Key::Named(keyboard::key::Named::ArrowDown) => {
                self.collapse_down(height, width);
                (0..width).for_each(|w| {
                    (0..height).rev().for_each(|h| {
                        self.merge_neighbouring(h, w, &mut previous, width, height, &direction)
                    });
                });
                self.collapse_down(height, width);
            }
            _ => {}
        }
        std::thread::sleep(time::Duration::from_secs_f64(0.05));
        if old_board != *self {
            let empty_ids = self
                .0
                .iter()
                .filter(|&&tile| tile.tilecontent.is_none())
                .map(|&tile| tile.id);
            let chosen_id = empty_ids.choose(&mut thread_rng()).unwrap();
            self.0[chosen_id].tilecontent =
                Some(*Self::TWO_OR_FOUR.choose(&mut thread_rng()).unwrap())
        }
    }
    fn collapse_left(&mut self, height: usize, width: usize) {
        (0..height).for_each(|h| {
            let mut row = vec![];
            (0..width).for_each(|w| row.push(self.0[pair_to_index(h, w, width)]));
            let collapsed = row.iter().filter(|&tile| tile.tilecontent.is_some());
            if collapsed.clone().copied().collect::<Vec<Tile>>() == row {
                return;
            }
            collapsed
                .clone()
                .copied()
                .enumerate()
                .for_each(|(w, tile)| {
                    self.0[pair_to_index(h, w, width)].tilecontent = tile.tilecontent;
                });
            (collapsed.count()..width).for_each(|w| {
                self.0[pair_to_index(h, w, width)].tilecontent = None;
            })
        });
    }
    fn collapse_right(&mut self, height: usize, width: usize) {
        (0..height).for_each(|h| {
            let mut row = vec![];
            (0..width)
                .rev()
                .for_each(|w| row.push(self.0[pair_to_index(h, w, width)]));
            let collapsed = row.iter().filter(|&tile| tile.tilecontent.is_some());
            if collapsed.clone().copied().collect::<Vec<Tile>>() == row {
                return;
            }
            collapsed
                .clone()
                .copied()
                .enumerate()
                .for_each(|(w, tile)| {
                    self.0[pair_to_index(h, width - w - 1, width)].tilecontent = tile.tilecontent;
                });
            (collapsed.count()..width).for_each(|w| {
                self.0[pair_to_index(h, width - w - 1, width)].tilecontent = None;
            })
        });
    }
    fn collapse_up(&mut self, height: usize, width: usize) {
        (0..width).for_each(|w| {
            let mut column = vec![];
            (0..height).for_each(|h| column.push(self.0[pair_to_index(h, w, width)]));
            let collapsed = column.iter().filter(|&tile| tile.tilecontent.is_some());
            if collapsed.clone().copied().collect::<Vec<Tile>>() == column {
                return;
            }
            collapsed.clone().enumerate().for_each(|(h, tile)| {
                self.0[pair_to_index(h, w, width)].tilecontent = tile.tilecontent;
            });
            (collapsed.count()..height).for_each(|h| {
                self.0[pair_to_index(h, w, width)].tilecontent = None;
            })
        });
    }
    fn collapse_down(&mut self, height: usize, width: usize) {
        (0..width).for_each(|w| {
            let mut column = vec![];
            (0..height)
                .rev()
                .for_each(|h| column.push(self.0[pair_to_index(h, w, width)]));
            let collapsed = column.iter().filter(|&tile| tile.tilecontent.is_some());
            if collapsed.clone().copied().collect::<Vec<Tile>>() == column {
                return;
            }
            collapsed.clone().enumerate().for_each(|(h, tile)| {
                self.0[pair_to_index(height - h - 1, w, width)].tilecontent = tile.tilecontent;
            });
            (collapsed.count()..height).for_each(|h| {
                self.0[pair_to_index(height - h - 1, w, width)].tilecontent = None;
            })
        });
    }
    fn merge_neighbouring(
        &mut self,
        h: usize,
        w: usize,
        previous: &mut Tile,
        width: usize,
        height: usize,
        direction: &keyboard::Key,
    ) {
        match (h, w) {
            (_, 0) if direction == &keyboard::Key::Named(keyboard::key::Named::ArrowLeft) => {
                *previous = self.0[pair_to_index(h, w, width)];
            }
            (_, w)
                if direction == &keyboard::Key::Named(keyboard::key::Named::ArrowRight)
                    && w == width - 1 =>
            {
                *previous = self.0[pair_to_index(h, w, width)];
            }
            (0, _) if direction == &keyboard::Key::Named(keyboard::key::Named::ArrowUp) => {
                *previous = self.0[pair_to_index(h, w, width)];
            }
            (h, _)
                if direction == &keyboard::Key::Named(keyboard::key::Named::ArrowDown)
                    && h == height - 1 =>
            {
                *previous = self.0[pair_to_index(h, w, width)];
            }
            _ => {
                let mut current = self.0[pair_to_index(h, w, width)];
                if previous.tilecontent == current.tilecontent {
                    if let Some(content) = previous.tilecontent {
                        self.0[previous.id].tilecontent = Some(content * 2);
                        self.0[current.id].tilecontent = None;
                        current.tilecontent = None
                    }
                }
                *previous = current;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Tile {
    tilecontent: Option<usize>,
    id: usize,
}

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
pub struct YourApp {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// Display a context drawer with the designated page if defined.
    context_page: ContextPage,
    /// Key bindings for the application's menu bar.
    key_binds: HashMap<menu::KeyBind, MenuAction>,
    game: Game,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    LaunchUrl(String),
    ToggleContextPage(ContextPage),
    Reset,
    InputWidth(String),
    InputHeight(String),
    StartPressed,
    GotoMenu,
    Event(Event),
}

/// Identifies a context page to display in the context drawer.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum ContextPage {
    #[default]
    About,
}

impl ContextPage {
    fn title(&self) -> String {
        match self {
            Self::About => fl!("about"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MenuAction {
    About,
}

impl menu::action::MenuAction for MenuAction {
    type Message = Message;

    fn message(&self) -> Self::Message {
        match self {
            MenuAction::About => Message::ToggleContextPage(ContextPage::About),
        }
    }
}

/// Implement the `Application` trait for your application.
/// This is where you define the behavior of your application.
///
/// The `Application` trait requires you to define the following types and constants:
/// - `Executor` is the async executor that will be used to run your application's commands.
/// - `Flags` is the data that your application needs to use before it starts.
/// - `Message` is the enum that contains all the possible variants that your application will need to transmit messages.
/// - `APP_ID` is the unique identifier of your application.
impl Application for YourApp {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.example.CosmicAppTemplate";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// This is the entry point of your application, it is where you initialize your application.
    ///
    /// Any work that needs to be done before the application starts should be done here.
    ///
    /// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
    /// - `flags` is used to pass in any data that your application needs to use before it starts.
    /// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut app = YourApp {
            core,
            context_page: ContextPage::default(),
            key_binds: HashMap::new(),
            game: Game::new(),
        };

        let command = app.update_titles();

        (app, command)
    }

    /// Elements to pack at the start of the header bar.
    fn header_start(&self) -> Vec<Element<Self::Message>> {
        let menu_bar = menu::bar(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.key_binds,
                vec![menu::Item::Button(fl!("about"), MenuAction::About)],
            ),
        )]);

        vec![menu_bar.into()]
    }

    /// This is the main view of your application, it is the root of your widget tree.
    ///
    /// The `Element` type is used to represent the visual elements of your application,
    /// it has a `Message` associated with it, which dictates what type of message it can send.
    ///
    /// To get a better sense of which widgets are available, check out the `widget` module.
    fn view(&self) -> Element<Message> {
        match self.game.menu.start_pressed {
            false => menu(&self.game),
            true => playfield(&self.game),
        }
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        event::listen().map(Message::Event)
    }

    /// Application messages are handled here. The application state can be modified based on
    /// what message was received. Commands may be returned for asynchronous execution on a
    /// background thread managed by the application's executor.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::LaunchUrl(url) => {
                let _result = open::that_detached(url);
            }

            Message::ToggleContextPage(context_page) => {
                if self.context_page == context_page {
                    // Close the context drawer if the toggled context page is the same.
                    self.core.window.show_context = !self.core.window.show_context;
                } else {
                    // Open the context drawer to display the requested context page.
                    self.context_page = context_page;
                    self.core.window.show_context = true;
                }

                // Set the title of the context drawer.
                self.set_context_title(context_page.title());
            }
            Message::GotoMenu => {
                self.game = Game::new();
            }
            Message::StartPressed => {
                self.game.menu.width = self.game.menu.width_inptut.parse().unwrap();
                self.game.menu.height = self.game.menu.height_inptut.parse().unwrap();

                self.game.board = Board::new(self.game.menu.width, self.game.menu.height);
                self.game.menu.start_pressed = true;
            }
            Message::InputWidth(input) => self.game.menu.width_inptut = input,
            Message::InputHeight(input) => self.game.menu.height_inptut = input,
            Message::Reset => {
                self.game.board = Board::new(self.game.menu.width, self.game.menu.height);
                self.game.has_ended = false;
            }
            Message::Event(Event::Keyboard(keyboard::Event::KeyPressed { key, .. })) => self
                .game
                .board
                .move_tile_content(key, self.game.menu.height, self.game.menu.width),
            _ => {}
        }
        Command::none()
    }

    /// Display a context drawer if the context page is requested.
    fn context_drawer(&self) -> Option<Element<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(match self.context_page {
            ContextPage::About => self.about(),
        })
    }
}

impl YourApp {
    /// The about page for this app.
    pub fn about(&self) -> Element<Message> {
        let cosmic_theme::Spacing { space_xxs, .. } = theme::active().cosmic().spacing;

        let icon = widget::svg(widget::svg::Handle::from_memory(
            &include_bytes!("../res/icons/hicolor/128x128/apps/com.example.CosmicAppTemplate.svg")
                [..],
        ));

        let title = widget::text::title3(fl!("app-title"));

        let link = widget::button::link(REPOSITORY)
            .on_press(Message::LaunchUrl(REPOSITORY.to_string()))
            .padding(0);

        widget::column()
            .push(icon)
            .push(title)
            .push(link)
            .align_items(Alignment::Center)
            .spacing(space_xxs)
            .into()
    }

    /// Updates the header and window titles.
    pub fn update_titles(&mut self) -> Command<Message> {
        let window_title = fl!("app-title");
        let header_title = String::new();

        self.set_header_title(header_title);
        self.set_window_title(window_title)
    }
}

fn playfield(game: &Game) -> widget::Container<'_, Message, cosmic::Theme> {
    let tilebutton = |id: usize| match game.board.0[id] {
        Tile {
            tilecontent: Some(2),
            ..
        } => container(centralize_tile_content(text(format!("2")).size(16)))
            .center_x()
            .center_y()
            .style(theme::Container::custom(widget_colors::gray1theme))
            .height(50)
            .width(50),
        Tile {
            tilecontent: Some(4),
            ..
        } => container(centralize_tile_content(text(format!("4")).size(16)))
            .center_x()
            .center_y()
            .style(theme::Container::custom(widget_colors::gray2theme))
            .height(50)
            .width(50),
        Tile {
            tilecontent: Some(8),
            ..
        } => container(centralize_tile_content(text(format!("8")).size(16)))
            .center_x()
            .center_y()
            .style(theme::Container::custom(widget_colors::orange1theme))
            .height(50)
            .width(50),
        Tile {
            tilecontent: Some(16),
            ..
        } => container(centralize_tile_content(text(format!("16")).size(16)))
            .center_x()
            .center_y()
            .style(theme::Container::custom(widget_colors::orange2theme))
            .height(50)
            .width(50),
        Tile {
            tilecontent: Some(32),
            ..
        } => container(centralize_tile_content(text(format!("32")).size(16)))
            .center_x()
            .center_y()
            .style(theme::Container::custom(widget_colors::red1theme))
            .height(50)
            .width(50),
        Tile {
            tilecontent: Some(64),
            ..
        } => container(centralize_tile_content(text(format!("64")).size(16)))
            .center_x()
            .center_y()
            .style(theme::Container::custom(widget_colors::red2theme))
            .height(50)
            .width(50),
        Tile {
            tilecontent: Some(content),
            ..
        } => container(centralize_tile_content(text(format!("{content}")).size(16)))
            .center_x()
            .center_y()
            .style(theme::Container::custom(widget_colors::blacktheme))
            .height(50)
            .width(50),
        Tile {
            tilecontent: None, ..
        } => container("")
            .center_x()
            .center_y()
            .style(theme::Container::custom(
                widget_colors::secondary_with_rounder_corners,
            ))
            .height(50)
            .width(50),
    };
    let playboard = (0..game.menu.height).fold(Grid::new(), |acc, row| {
        let new_row = (0..game.menu.width).fold(Row::new(), |acc2, column| {
            acc2.push(tilebutton(pair_to_index(row, column, game.menu.height)))
        });
        acc.push(new_row.spacing(2).align_items(Alignment::Center))
            .insert_row()
    });
    let menu_button = button("MENU").on_press(Message::GotoMenu);
    let reset_button = button("RESET")
        .on_press(Message::Reset)
        .style(theme::Button::Destructive);
    container(
        widget::column()
            .push(
                widget::row()
                    .push(menu_button)
                    .push(reset_button)
                    .padding(20)
                    .spacing(20),
            )
            .align_items(Alignment::Center)
            .push(
                container(playboard.row_spacing(2).row_alignment(Alignment::Center))
                    .style(theme::Container::Primary)
                    .width((52 * game.menu.width + 2) as f32)
                    .height((52 * game.menu.height + 2) as f32)
                    .center_x()
                    .center_y()
                    .padding(0),
            ),
    )
    .padding(20)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
}

fn centralize_tile_content(tile_content: Text<Theme, Renderer>) -> Text<Theme, Renderer> {
    tile_content
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
}

fn menu(game: &Game) -> widget::Container<'_, Message, cosmic::Theme> {
    let width_box = text_input("", &game.menu.width_inptut).on_input(Message::InputWidth);
    let height_box = text_input("", &game.menu.height_inptut).on_input(Message::InputHeight);
    let start_game_button = button(centralize_tile_content(text("START")))
        .on_press(Message::StartPressed)
        .width(96)
        .height(55);
    container(
        widget::column()
            .push(
                widget::row()
                    .push(text("Width: "))
                    .push(width_box.width(40))
                    .align_items(Alignment::Center),
            )
            .push(
                widget::row()
                    .push(text("Height: "))
                    .push(height_box.width(40)),
            )
            .push(start_game_button)
            .align_items(Alignment::Center)
            .spacing(20),
    )
}
