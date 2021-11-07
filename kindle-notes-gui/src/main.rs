use druid::widget::{Button, Controller, Flex, Label, TextBox};
use druid::{
    AppLauncher, Data, Env, Event, EventCtx, Lens, LocalizedString, PlatformError, Widget,
    WidgetExt, WindowDesc, UpdateCtx
};

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
    output_folder: String,
    status: Status,
}

impl AppData {
    fn new() -> AppData {
        AppData {
            notes_path: String::from(""),
            output_folder: String::from("~/"),
            status: Status::Idle,
        }
    }
    fn update_states(&mut self) {
        println!("Here I can update stuff like disabled, invalid input etc");
        println!("Here's yur app data: {:?}", self);
    }
}

fn main() -> Result<(), PlatformError> {
    let data = AppData::new();

    let main_window =
        WindowDesc::new(ui_builder()).title(LocalizedString::new("Kindle Notes parser"));

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<AppData> {
    // The label text will be computed dynamically based on the current locale and count
    const INPUT_PADDING: (f64, f64, f64, f64) = (20.0, 5.0, 20.0, 0.0);
    const LABEL_PADDING: (f64, f64, f64, f64) = (20.0, 20.0, 20.0, 0.0);

    let notes_path_text = LocalizedString::new("Notes path (txt file)");
    let notes_path_label = Label::new(notes_path_text)
        .padding(LABEL_PADDING)
        .align_left();
    let notes_path_input = TextBox::new()
        .lens(AppData::notes_path)
        .expand_width()
        .padding(INPUT_PADDING)
        .controller(TboxControl);

    let output_folder_text = LocalizedString::new("Output folder");
    let output_folder_label = Label::new(output_folder_text)
        .padding(LABEL_PADDING)
        .align_left();
    let output_folder_input = TextBox::new()
        .lens(AppData::output_folder)
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

impl <W: Widget<AppData>> Controller<AppData, W> for BtnController {
    fn update(
        &mut self,
        child: &mut W,
        ctx: &mut UpdateCtx,
        old: &AppData,
        data: &AppData,
        env: &Env
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
}
