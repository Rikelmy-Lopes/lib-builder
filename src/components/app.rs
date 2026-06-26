use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::fs::fs::{choose_folder, extract_binaries, get_7zip_executable, get_ant_executable};
use crate::utils::commands::{kill_process, spawn_7zip, spawn_ant_build};
use crate::utils::output::{format_output, is_7zip_successful, is_build_successful};
use iced::futures::channel::oneshot;
use iced::widget::{Container, button, column, row, text, text_input};
use iced::window::Settings;
use iced::{Alignment, Element, Length, Size, Task, Theme};

pub struct App {
    source: String,
    destination: String,
    theme: Theme,
    process_id: Arc<Mutex<i32>>,
    is_running: bool,
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
    ExecuteCompleted(BuildEvents),
}

#[derive(Debug, Clone)]
pub enum BuildEvents {
    UnknownError,
    AntError,
    SevenZipError,
    Success,
}

impl App {
    pub fn new() -> (Self, iced::Task<Message>) {
        extract_binaries();
        (
            Self {
                source: String::new(),
                destination: String::new(),
                theme: iced::Theme::Oxocarbon,
                process_id: Arc::new(Mutex::new(-1)),
                is_running: false,
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
                let (tx, rx) = oneshot::channel::<BuildEvents>();
                let source = self.source.clone();
                self.is_running = true;

                let process_id = Arc::clone(&self.process_id);

                Task::perform(
                    async move {
                        thread::spawn(move || {
                            let ant_path = get_ant_executable();
                            let seven_zip_path = get_7zip_executable();

                            let child = spawn_ant_build(&ant_path, &PathBuf::from(&source));

                            {
                                if let Ok(mut guard) = process_id.lock() {
                                    *guard = child.id() as i32;
                                }
                            }

                            let formatted_output = format_output(child.wait_with_output().unwrap());
                            println!("{}", formatted_output);

                            if !is_build_successful(&formatted_output) {
                                let _ = tx.send(BuildEvents::AntError);
                                return; // Encerra a thread mais cedo em caso de erro
                            }

                            let build_file_path = PathBuf::from(&source).join("dist/SIGP_INT.jar");
                            let child = spawn_7zip(&seven_zip_path, &build_file_path);

                            {
                                if let Ok(mut guard) = process_id.lock() {
                                    *guard = child.id() as i32;
                                }
                            }

                            let formatted_output = format_output(child.wait_with_output().unwrap());
                            println!("{}", formatted_output);

                            if !is_7zip_successful(&formatted_output) {
                                let _ = tx.send(BuildEvents::SevenZipError);
                                return;
                            }

                            let _ = tx.send(BuildEvents::Success);
                        });

                        rx.await.unwrap_or_else(|_| BuildEvents::UnknownError)
                    },
                    Message::ExecuteCompleted,
                )
            }
            Message::Cancel => {
                if let Ok(guard) = self.process_id.lock() {
                    kill_process(&guard);
                }
                self.process_id = Arc::new(Mutex::new(-1));
                self.is_running = false;
                Task::none()
            }

            Message::ExecuteCompleted(message) => {
                self.is_running = false;
                Task::none()
            }
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

        let mut column = column![
            row_source,
            row_destination,
            button("Executar").on_press(Message::Execute),
            button("Cancelar").on_press(Message::Cancel)
        ]
        .spacing(20)
        .padding(10)
        .align_x(Alignment::Center);

        if self.is_running {
            column = column.push(text!("Executando..."));
        }

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
