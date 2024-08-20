use iced::executor;
use iced::highlighter;
use iced::keyboard;
use iced::theme::{self, Theme};
use iced::widget::{
    button, column, container, horizontal_space, row, text, tooltip, Column,
};
use iced::Color;
use iced::{
    Alignment, Application, Command, Element, Font, Length, Settings, Subscription,
};

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub fn main() -> iced::Result {
    let mut flags = Flags::default();

    if std::env::args().len() > 1 {
        flags.filename = Some(std::env::args().nth(1).unwrap().into());
    }

    println!("Opening: {:?}", flags.filename);

    Editor::run(Settings {
        flags,
        fonts: vec![include_bytes!("../fonts/icons.ttf").as_slice().into()],
        default_font: Font::MONOSPACE,
        ..Settings::default()
    })
}

#[derive(Debug, Clone, Default)]
pub struct Flags {
    filename: Option<PathBuf>,
}

struct Editor {
    file: Option<PathBuf>,
    content: Module,
    theme: highlighter::Theme,
    is_loading: bool,
    is_dirty: bool,
}

#[derive(Debug, Clone)]
enum Message {
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    Expand(Section),
}

#[derive(Debug, Clone)]
enum Section {
    Type,
}

impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        let is_loading = flags.filename.is_none();
        (
            Self {
                file: flags.filename.clone(),
                content: Module::default(),
                theme: highlighter::Theme::SolarizedDark,
                is_loading,
                is_dirty: false,
            },
            match flags.filename {
                Some(path) => Command::perform(load_file(path), Message::FileOpened),
                None => Command::none(),
            },
        )
    }

    fn title(&self) -> String {
        String::from("WASM Visualizer")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Expand(section) => {
                match section {
                    Section::Type => {
                        println!("Expand Type");
                    }
                }
                Command::none()
            }
            Message::OpenFile => {
                if self.is_loading {
                    Command::none()
                } else {
                    self.is_loading = true;

                    Command::perform(open_file(), Message::FileOpened)
                }
            }
            Message::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    self.content = wasm_decoder(contents.as_str());
                }

                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| match key.as_ref() {
            keyboard::Key::Character("s") if modifiers.command() => {
                // Some(Message::SaveFile)
                None
            }
            _ => None,
        })
    }

    fn view(&self) -> Element<Message> {
        let controls = row![
            action(
                open_icon(),
                "Open file",
                (!self.is_loading).then_some(Message::OpenFile)
            ),
            horizontal_space(),
        ]
        .spacing(10)
        .align_items(Alignment::Center);

        let status = row![
            text(if let Some(path) = &self.file {
                let path = path.display().to_string();
                if path.len() > 60 {
                    format!("...{}", &path[path.len() - 40..])
                } else {
                    path
                }
            } else {
                String::from("New file")
            }),
            horizontal_space()
        ]
        .spacing(10);

        let title_style: theme::Text = theme::Text::Color(Color::from_rgb(1.0, 0.0, 0.0));
        #[rustfmt::skip]
        let col = {
            let mut col = column![];
            col = create_label(col, &self.content.header,    "Header",       title_style, Section::Type);
            col = create_label(col, &self.content.types,     "01 Types",     title_style, Section::Type);
            col = create_label(col, &self.content.imports,   "02 Imports",   title_style, Section::Type);
            col = create_label(col, &self.content.functions, "03 Functions", title_style, Section::Type);
            col = create_label(col, &self.content.tables,    "04 Tables",    title_style, Section::Type);
            col = create_label(col, &self.content.memories,  "05 Memories",  title_style, Section::Type);
            col = create_label(col, &self.content.globals,   "06 Globals",   title_style, Section::Type);
            col = create_label(col, &self.content.exports,   "07 Exports",   title_style, Section::Type);
            col = create_label(col, &self.content.start,     "08 Start",     title_style, Section::Type);
            col = create_label(col, &self.content.elements,  "09 Elements",  title_style, Section::Type);
            col = create_label(col, &self.content.code,      "0A Code",      title_style, Section::Type);
            col = create_label(col, &self.content.data,      "0B Data",      title_style, Section::Type);
            col = create_label(col, &self.content.data_count,"0C Data Count",title_style, Section::Type);
            col
        };

        column![
            controls,
            container(col).width(Length::Fill).height(Length::Fill),
            horizontal_space(),
            status,
        ]
        .spacing(10)
        .padding(10)
        .into()
    }

    fn theme(&self) -> Theme {
        if self.theme.is_dark() {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}

async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(picked_file.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

async fn save_file(path: Option<PathBuf>, contents: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}

fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).width(30).center_x());

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(theme::Container::Box)
        .into()
    } else {
        action.style(theme::Button::Secondary).into()
    }
}

fn new_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e800}')
}

fn save_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e801}')
}

fn open_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0f115}')
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}

#[derive(Debug, Clone, Default)]
struct Module {
    header: Vec<u8>,
    custom: Vec<u8>,
    types: Vec<u8>,
    imports: Vec<u8>,
    functions: Vec<u8>,
    tables: Vec<u8>,
    memories: Vec<u8>,
    globals: Vec<u8>,
    exports: Vec<u8>,
    start: Vec<u8>,
    elements: Vec<u8>,
    code: Vec<u8>,
    data: Vec<u8>,
    data_count: Vec<u8>,
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Header:\n{}", into_hex(&self.header))?;
        write!(f, "Custom:\n{}", into_hex(&self.custom))?;
        write!(f, "Types:\n{}", into_hex(&self.types))?;
        write!(f, "Imports:\n{}", into_hex(&self.imports))?;
        write!(f, "Functions:\n{}", into_hex(&self.functions))?;
        write!(f, "Tables:\n{}", into_hex(&self.tables))?;
        write!(f, "Memeories:\n{}", into_hex(&self.memories))?;
        write!(f, "Globals:\n{}", into_hex(&self.globals))?;
        write!(f, "Exports:\n{}", into_hex(&self.exports))?;
        write!(f, "Start:\n{}", into_hex(&self.start))?;
        write!(f, "Elements:\n{}", into_hex(&self.elements))?;
        write!(f, "Code:\n{}", into_hex(&self.code))?;
        write!(f, "Data:\n{}", into_hex(&self.data))?;
        write!(f, "Data Count:\n{}", into_hex(&self.data_count))
    }
}

fn create_label<'a>(
    mut col: Column<'a, Message>,
    data: &[u8],
    label: &str,
    style: theme::Text,
    action: Section,
) -> Column<'a, Message> {
    if !data.is_empty() {
        col = col.push(
            button(column![text(label).style(style), text(into_hex(data)),])
                .on_press(Message::Expand(action)),
        );
    }
    col
}

fn into_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X} ", b))
        .collect::<String>()
        + "\n"
}

fn wasm_decoder(source: &str) -> Module {
    let b = source.as_bytes().iter();
    let mut bytes = b.peekable();
    let mut module = Module::default();
    module.header = take(&mut bytes, 8);
    while let Some(byte) = bytes.next() {
        match byte {
            // Custom
            0x00 => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.custom = take(&mut bytes, length);
            }
            // Type
            0x01 => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.types = take(&mut bytes, length);
            }
            // Import
            0x02 => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.imports = take(&mut bytes, length);
            }
            // Function
            0x03 => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.functions = take(&mut bytes, length);
            }
            // Table
            0x04 => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.tables = take(&mut bytes, length);
            }
            // Memory
            0x05 => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.memories = take(&mut bytes, length);
            }
            // Global
            0x06 => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.globals = take(&mut bytes, length);
            }
            // Export
            0x07 => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.exports = take(&mut bytes, length);
            }
            // Start
            0x08 => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.start = take(&mut bytes, length);
            }
            // Element
            0x09 => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.elements = take(&mut bytes, length);
            }
            // Code
            0x0A => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.code = take(&mut bytes, length);
            }
            // Data
            0x0B => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.data = take(&mut bytes, length);
            }
            // Data Count
            0x0C => {
                let length = ((**bytes.peek().unwrap_or(&&0)) as usize) + 1;
                module.data_count = take(&mut bytes, length);
            }
            // Unknown
            _ => {}
        }
    }
    module
}

fn take(iter: &mut std::iter::Peekable<std::slice::Iter<u8>>, length: usize) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut count = 0;
    while let Some(byte) = iter.next() {
        bytes.push(*byte);
        count += 1;
        if count == length {
            break;
        }
    }
    bytes
}
