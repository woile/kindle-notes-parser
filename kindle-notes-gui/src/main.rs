extern crate dirs;
use druid::widget::{Button, Controller, Flex, Label, TextBox, WidgetWrapper};
use druid::{
    commands, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Event, EventCtx,
    FileDialogOptions, FileSpec, Handled, Lens, LocalizedString, PlatformError, Target, UpdateCtx,
    Widget, WidgetExt, WindowDesc,
};
use kindle_notes_core::Config;
use std::fs;
use std::iter::FromIterator;
use std::process;

const SPACING: f64 = 15.;

#[derive(Clone, Data, Debug, PartialEq)]
enum Status {
    Idle,
    InProgress,
    Done,
    Error,
}

#[derive(Clone, Data, Lens, Debug)]
struct AppData {
    notes_path: String,
    output_path: String,
    status: Status,
    index: usize,
    disabled: bool,
    feedback_text: String,
}

impl AppData {
    fn new() -> AppData {
        let default_documents = dirs::document_dir().expect("Couldn't find Documents directory");
        let default_notes = default_documents.join("My Clippings.txt");

        AppData {
            notes_path: String::from(default_notes.to_str().unwrap()),
            output_path: String::from(default_documents.to_str().unwrap()),
            status: Status::Idle,
            index: 0,
            disabled: false,
            feedback_text: String::from(""),
        }
    }
    fn update_states(&mut self) {
        println!("Here I can update stuff like disabled, invalid input etc");
        println!("Here's yur app data: {:?}", self);
    }
}

impl Iterator for AppData {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => String::from(""),
            1 => self.notes_path.clone(),
            2 => self.output_path.clone(),
            _ => {
                self.index = 0;
                return None;
            }
        };
        self.index += 1;
        Some(result)
    }
}

impl AppDelegate<AppData> for AppData {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppData,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            let file_path = file_info.path();
            if file_path.is_dir() {
                data.output_path = file_path.to_string_lossy().to_string();
            } else {
                data.notes_path = file_path.to_string_lossy().to_string();
            }

            return Handled::Yes;
        }
        Handled::No
    }
}

fn main() -> Result<(), PlatformError> {
    let data = AppData::new();

    let main_window =
        WindowDesc::new(ui_builder()).title(LocalizedString::new("Kindle Notes Parser"));

    AppLauncher::with_window(main_window)
        .delegate(AppData::new())
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<AppData> {
    // The label text will be computed dynamically based on the current locale and count
    const INPUT_PADDING: (f64, f64, f64, f64) = (30.0, 5.0, 30.0, 0.0);
    const LABEL_PADDING: (f64, f64, f64, f64) = (30.0, 20.0, 30.0, 0.0);

    // EXPLANATION TEXT
    let explanation_text = LocalizedString::new("Choose the input file copied from your kindle,\nand the folder where you want the new files to be created");
    let explanation = Label::new(explanation_text)
        .padding((30.0, 10.0))
        .align_left();

    // FILE INPUT PATH
    let notes_path_text = LocalizedString::new("Notes path (txt file)");
    let notes_path_label = Label::new(notes_path_text)
        .padding(LABEL_PADDING)
        .align_left();
    let notes_path_input = TextBox::new()
        .lens(AppData::notes_path)
        // .disabled_if(|x| {
        // })
        .expand_width()
        .padding(INPUT_PADDING)
        .controller(TboxControl);

    // OPEN INPUT FILE BUTTON
    let txt = FileSpec::new("Text file", &["txt"]);
    let default_open_name = String::from("My Clippings.txt");
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![txt])
        .default_type(txt)
        .default_name(default_open_name)
        .name_label("Source")
        .title("Select your kindle notes")
        .button_text("Open file");
    let open = Button::new(LocalizedString::new("Choose file"))
        .on_click(move |ctx, _, _| {
            ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
        })
        .align_right()
        .padding((30.0, 15.0, 30.0, 0.0));

    // FOLDER OUTPUT PATH
    let output_folder_text = LocalizedString::new("Output folder");
    let output_folder_label = Label::new(output_folder_text)
        .padding(LABEL_PADDING)
        .align_left();
    let output_folder_input = TextBox::new()
        .lens(AppData::output_path)
        .expand_width()
        .padding(INPUT_PADDING)
        .controller(TboxControl);

    // OPEN OUTPUT FOLDER BUTTON
    let open_folder_dialog_options = FileDialogOptions::new()
        .select_directories()
        .name_label("Output")
        .title("Select output folder")
        .button_text("Open folder");
    let open_folder = Button::new(LocalizedString::new("Choose folder"))
        .on_click(move |ctx, _, _| {
            ctx.submit_command(
                druid::commands::SHOW_OPEN_PANEL.with(open_folder_dialog_options.clone()),
            )
        })
        .align_right()
        .padding((30.0, 15.0, 30.0, 0.0));

    // SUBMIT BUTTON
    let button = Button::new("Run")
        .on_click(|_, data: &mut AppData, _: &_| submit(data))
        .padding(5.0)
        .controller(BtnController)
        .disabled_if(|data, _| data.disabled);

    // FEEDBACK TEXT
    let feedback = Label::new(|data: &AppData, _: &Env| data.feedback_text.clone());

    Flex::column()
        .with_child(explanation)
        .with_child(notes_path_label)
        .with_child(notes_path_input)
        .with_child(open)
        .with_spacer(SPACING)
        .with_child(output_folder_label)
        .with_child(output_folder_input)
        .with_child(open_folder)
        .with_spacer(SPACING)
        .with_child(button)
        .with_child(feedback)
}

struct TboxControl;

impl<W: Widget<AppData>> Controller<AppData, W> for TboxControl {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppData,
        env: &Env,
    ) {
        // Pass to child first to save an update block
        child.event(ctx, event, data, env);

        if let Event::KeyDown(_) = event {
            data.update_states()
        }
    }
}

struct BtnController;

impl<W: Widget<AppData>> Controller<AppData, W> for BtnController {
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old: &AppData,
        data: &AppData,
        env: &Env,
    ) {
        child.update(ctx, old, data, env);
        // repaint widget everytime data changes
        ctx.request_paint()
    }
}

// Application login
fn submit(data: &mut AppData) {
    data.status = Status::InProgress;
    data.disabled = true;
    data.feedback_text = String::from("Running...");

    println!("Data result: {:?}", data);
    let args = Vec::from_iter(data.clone());
    println!("args are: {:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = kindle_notes_core::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
    data.feedback_text = String::from("Done");
    data.disabled = false;
}
