use std::path::PathBuf;
use std::thread;

use crate::fs::fs::{
    choose_folder, extract_binaries, get_7zip_executable, get_ant_executable,
    get_extract_binaries_size,
};
use crate::utils::commands::{kill_process, spawn_7zip, spawn_ant_build};
use crate::utils::output::{format_output, is_7zip_successful, is_build_successful};
use iced::futures::channel::oneshot;
use iced::widget::{Container, button, column, row, text_input};
use iced::window::Settings;
use iced::{Alignment, Element, Length, Size, Task, Theme};

pub struct App {
    source: String,
    destination: String,
    theme: Theme,
    process_id: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    WriteSource(String),
    WriteDestination(String),
    ChooseSourceBegin,
    ChooseSourceFinished(String),
    ChooseDestinationBegin,
    ChooseDestinationFinished(String),
    Execute,
    Cancel,
    ExecuteCompleted(Events),
}

#[derive(Debug, Clone)]
pub enum Events {
    UNKNOWN_ERROR,
    ANT_ERROR,
    SEVEN_ZIP_ERROR,
    SUCCESS,
}

impl App {
    pub fn new() -> (Self, iced::Task<Message>) {
        extract_binaries();
        (
            Self {
                source: String::new(),
                destination: String::new(),
                theme: iced::Theme::TokyoNight,
                process_id: 0,
            },
            iced::Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WriteSource(content) => {
                self.source = content;
                Task::none()
            }
            Message::WriteDestination(content) => {
                self.destination = content;
                Task::none()
            }

            Message::ChooseSourceBegin => {
                Task::perform(choose_folder(), |v| Message::ChooseSourceFinished(v))
            }

            Message::ChooseDestinationBegin => {
                Task::perform(choose_folder(), |v| Message::ChooseDestinationFinished(v))
            }

            Message::ChooseSourceFinished(path) => {
                if !path.is_empty() {
                    self.source = path;
                }
                Task::none()
            }

            Message::ChooseDestinationFinished(path) => {
                if !path.is_empty() {
                    self.destination = path;
                }
                Task::none()
            }
            Message::Execute => {
                let (tx, rx) = oneshot::channel::<Events>();
                let source_project = self.source.clone();

                Task::perform(
                    async move {
                        thread::spawn(move || {
                            let ant_path = get_ant_executable();
                            let seven_zip_path = get_7zip_executable();

                            let child = spawn_ant_build(&ant_path, &source_project);

                            let formatted_output = format_output(child.wait_with_output().unwrap());
                            println!("{}", formatted_output);

                            if !is_build_successful(&formatted_output) {
                                let _ = tx.send(Events::ANT_ERROR);
                                return; // Encerra a thread mais cedo em caso de erro
                            }

                            let build_file_path =
                                PathBuf::from(&source_project).join("dist/SIGP_INT.jar");
                            let child =
                                spawn_7zip(&seven_zip_path, build_file_path.to_str().unwrap());

                            //let pid = child.id();

                            let formatted_output = format_output(child.wait_with_output().unwrap());
                            println!("{}", formatted_output);
                            if !is_7zip_successful(&formatted_output) {
                                let _ = tx.send(Events::SEVEN_ZIP_ERROR);
                                return;
                            }

                            let _ = tx.send(Events::SUCCESS);
                        });

                        rx.await.unwrap_or_else(|_| Events::UNKNOWN_ERROR)
                    },
                    Message::ExecuteCompleted,
                )
            }
            Message::Cancel => {
                get_extract_binaries_size();
                kill_process(&self.process_id);
                Task::none()
            }

            Message::ExecuteCompleted(message) => Task::none(),
            _ => Task::none(),
        }
    }

    pub fn view<'a>(&self) -> Element<'_, Message> {
        let row_source = row![
            text_input("Origem", &self.source).on_input(Message::WriteSource),
            button("Escolher").on_press(Message::ChooseSourceBegin)
        ]
        .spacing(20);

        let row_destination = row![
            text_input("Destino", &self.destination).on_input(Message::WriteDestination),
            button("Escolher").on_press(Message::ChooseDestinationBegin)
        ]
        .spacing(20);

        let column = column![
            row_source,
            row_destination,
            button("Executar").on_press(Message::Execute),
            button("Cancelar").on_press(Message::Cancel)
        ]
        .spacing(20)
        .padding(10)
        .align_x(Alignment::Center);

        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .into()
    }

    pub fn theme(&self) -> iced::Theme {
        self.theme.clone()
    }

    pub fn settings() -> Settings {
        Settings {
            min_size: Some(Size {
                height: 300.0,
                width: 400.0,
            }),
            ..Settings::default()
        }
    }
}
