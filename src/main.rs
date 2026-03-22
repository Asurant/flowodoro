use eframe::egui;
use std::time::{Instant, Duration};

struct Todo{
    text: String,
    done: bool,
}

struct TodoApp{
    todos: Vec<Todo>,
    new_todo_text: String,
    work_time: u64,
    break_time: u64,
    running: bool,
    button_click: u32,
    start: Option<Instant>,
}

impl TodoApp{
    fn new(_cc: &eframe::CreationContext<'_>) -> Self{
        Self{
            todos: Vec::new(),
            new_todo_text: String::new(),
            work_time: 0,
            break_time: 0,
            running: false,
            button_click: 1,
            start: None,
        }
    }
}

impl eframe::App for TodoApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Le Flowodoro");

            ui.add_space(10.0);
            let work_running = self.button_click%2 == 1;
            let mut display_time = 0;
            let mut progress = 1.0;
            if self.running{
                if let Some(start) = self.start{
                    if work_running{
                        display_time = start.elapsed().as_secs();

                        progress = 1.0;
                    }else{
                        let break_length = self.work_time/5;
                        if start.elapsed().as_secs() >= break_length{
                            display_time = 0;
                            progress = 0.0;
                            self.running = false;
                            self.work_time = 0;
                            self.button_click += 1;
                        }else{
                            display_time = break_length - start.elapsed().as_secs();
                            self.break_time = display_time;
                            progress = display_time as f32 / break_length as f32;
                        }
                    }
                }
            }
            ui.label(format!("{display_time} seconds"));
            ui.add(egui::ProgressBar::new(progress).text(format!("{display_time} seconds")));
            ui.horizontal(|ui|{
                if ui.button(if self.running{"Break"} else {"Start"}).clicked(){
                    if self.running && work_running{
                        self.running = true;
                        self.button_click += 1;
                        self.start = Some(Instant::now());
                        self.break_time = display_time / 5;
                        self.work_time = display_time;
                    }else{
                        self.running = true;
                        self.start = Some(Instant::now());
                    }
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(5.0);

            //.horizontal means the stuff will stay in the same line
            ui.horizontal(|ui|{
                //.text_edit_singleline is for single text inputs.
                let text_input = ui.text_edit_singleline(&mut self.new_todo_text);

                if(ui.button("Add").clicked() || (text_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))) && !self.new_todo_text.is_empty(){
                    self.todos.push(Todo{
                        text:self.new_todo_text.clone(),
                        done:false,
                    });

                    self.new_todo_text.clear();

                    text_input.request_focus();

                    
                    
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(5.0);

            let total = self.todos.len();
            let done_count = self.todos.iter().filter(|t| t.done).count();
            ui.label(format!("{done_count} of {total} tasks completed"));

            ui.add_space(5.0);

            let mut to_remove: Option<usize> = None;

            for(i, todo) in self.todos.iter_mut().enumerate(){
                ui.horizontal(|ui|{
                    ui.checkbox(&mut todo.done, "");

                    if todo.done{
                        ui.label(egui::RichText::new(&todo.text).strikethrough(),
                        );
                    }else{
                        ui.label(&todo.text);
                    }

                    if ui.button("X").clicked(){
                        to_remove = Some(i);
                    }
                });
            }

            if let Some(index) = to_remove{
                self.todos.remove(index);
            }

            if self.todos.iter().any(|t| t.done){
                ui.add_space(10.0);
                if ui.button("Clear completed").clicked(){
                    self.todos.retain(|t| !t.done);
                }
            }
        });
    }
}

fn main() -> eframe::Result{
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "To-Do List",
        native_options,
        Box::new(|cc| Ok(Box::new(TodoApp::new(cc)))),
    )
}