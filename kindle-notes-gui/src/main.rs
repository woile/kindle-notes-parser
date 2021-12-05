extern crate dirs;
use druid::widget::{Button, Controller, Flex, Label, TextBox};
use druid::{
    commands, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Event, EventCtx,
    FileDialogOptions, FileSpec, Handled, Lens, LocalizedString, PlatformError, Target, UpdateCtx,
    Widget, WidgetExt, WindowDesc,
};
use kindle_notes_core::Config;
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
}

impl AppData {
    fn new() -> AppData {
        AppData {
            notes_path: String::from("My Clippings.txt"),
            output_path: String::from(
                dirs::document_dir()
                    .expect("Couldn't find Documents directory")
                    .to_str()
                    .unwrap(),
            ),
            status: Status::Idle,
            index: 0,
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
            _ => return None,
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
            println!("Vos queres ESTA {:?}", file_info.path());
            println!("Que es este string? {:?}", data);
            data.notes_path = file_info.path().to_string_lossy().to_string();

            return Handled::Yes;
        }
        Handled::No
    }
}

fn main() -> Result<(), PlatformError> {
    let data = AppData::new();

    let main_window =
        WindowDesc::new(ui_builder()).title(LocalizedString::new("Kindle Notes parser"));

    AppLauncher::with_window(main_window)
        .delegate(AppData::new())
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<AppData> {
    // The label text will be computed dynamically based on the current locale and count
    const INPUT_PADDING: (f64, f64, f64, f64) = (20.0, 5.0, 20.0, 0.0);
    const LABEL_PADDING: (f64, f64, f64, f64) = (20.0, 20.0, 20.0, 0.0);

    let txt = FileSpec::new("Text file", &["txt"]);
    let default_open_name = String::from("My Clippings.txt");
    let open_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![txt])
        .default_type(txt)
        .default_name(default_open_name)
        .name_label("Source")
        .title("Select your kindle notes")
        .button_text("Open file");
    let open = Button::new(LocalizedString::new("Choose file")).on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
    });

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

    let output_folder_text = LocalizedString::new("Output folder");
    let output_folder_label = Label::new(output_folder_text)
        .padding(LABEL_PADDING)
        .align_left();
    let output_folder_input = TextBox::new()
        .lens(AppData::output_path)
        .expand_width()
        .padding(INPUT_PADDING)
        .controller(TboxControl);

    let button = Button::new("Run")
        .on_click(|_, data: &mut AppData, _: &_| submit(data))
        .padding(5.0)
        .controller(BtnController);

    Flex::column()
        .with_child(notes_path_label)
        .with_child(notes_path_input)
        .with_child(open)
        .with_spacer(SPACING)
        .with_child(output_folder_label)
        .with_child(output_folder_input)
        .with_spacer(SPACING)
        .with_child(button)
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

    println!("Data result: {:?}", data);
    let args = Vec::from_iter(data);
    println!("args are: {:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = kindle_notes_core::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
