use conrod;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use misc;
use std;

pub const START_WIDTH: u32 = 800;
pub const START_HEIGHT: u32 = 600;

pub struct ConrodUi {}

impl ConrodUi {
    // Button matrix dimensions.
    const ROWS: usize = 10;
    const COLS: usize = 24;

    pub fn run_ui() {
        let mut events_loop = glium::glutin::EventsLoop::new();
        let title = format!("Tweleter v{}", misc::get_version_str());
        let window = glium::glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(START_WIDTH, START_HEIGHT);
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(
            window, context, &events_loop
        ).unwrap();
        let dimensions = [START_WIDTH as f64, START_HEIGHT as f64];
        let mut ui = conrod::UiBuilder::new(dimensions).build();

        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
        let ids = &mut Ids::new(ui.widget_id_generator());
        let mut event_loop = EventLoop::new();

        'uiloop: loop {

            // Handle all events.
            for event in event_loop.next(&mut events_loop) {

                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
                }

                match event {
                    glium::glutin::Event::WindowEvent { event, .. } => match event {
                        // Break from the loop upon `Escape`.
                        glium::glutin::WindowEvent::Closed |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => break 'uiloop,
                        _ => (),
                    },
                    _ => (),
                }
            }

            // Instantiate all widgets in the GUI.
            ConrodUi::set_widgets(ui.set_widgets(), ids);

            // Render the `Ui` and then display it on the screen.
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }

    fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids) {
        use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

        // Construct our main `Canvas` tree.
        widget::Canvas::new().flow_down(&[
            (ids.header, widget::Canvas::new().color(color::BLUE).pad_bottom(20.0)),
            (ids.body, widget::Canvas::new().length(300.0).flow_right(&[
                (ids.left_column, widget::Canvas::new().color(color::LIGHT_ORANGE).pad(20.0)),
                (ids.middle_column, widget::Canvas::new().color(color::ORANGE)),
                (ids.right_column, widget::Canvas::new().color(color::DARK_ORANGE).pad(20.0)),
            ])),
            (ids.footer, widget::Canvas::new().color(color::BLUE).scroll_kids_vertically()),
        ]).set(ids.master, ui);

        // A scrollbar for the `FOOTER` canvas.
        widget::Scrollbar::y_axis(ids.footer).auto_hide(true).set(ids.footer_scrollbar, ui);

        // Now we'll make a couple floating `Canvas`ses.
        let floating = widget::Canvas::new().floating(true).w_h(110.0, 150.0).label_color(color::WHITE);
        floating.middle_of(ids.left_column).title_bar("Blue").color(color::BLUE).set(ids.floating_a, ui);
        floating.middle_of(ids.right_column).title_bar("Orange").color(color::LIGHT_ORANGE).set(ids.floating_b, ui);

        // Here we make some canvas `Tabs` in the middle column.
        widget::Tabs::new(&[(ids.tab_foo, "FOO"), (ids.tab_bar, "BAR"), (ids.tab_baz, "BAZ")])
            .wh_of(ids.middle_column)
            .color(color::BLUE)
            .label_color(color::WHITE)
            .middle_of(ids.middle_column)
            .set(ids.tabs, ui);

        widget::Text::new("Fancy Title")
            .color(color::LIGHT_ORANGE)
            .font_size(48)
            .middle_of(ids.header)
            .set(ids.title, ui);
        widget::Text::new("Subtitle")
            .color(color::BLUE.complement())
            .mid_bottom_of(ids.header)
            .set(ids.subtitle, ui);

        widget::Text::new("Top Left")
            .color(color::LIGHT_ORANGE.complement())
            .top_left_of(ids.left_column)
            .set(ids.top_left, ui);

        widget::Text::new("Bottom Right")
            .color(color::DARK_ORANGE.complement())
            .bottom_right_of(ids.right_column)
            .set(ids.bottom_right, ui);

        fn text(text: widget::Text) -> widget::Text { text.color(color::WHITE).font_size(36) }
        text(widget::Text::new("Foo!")).middle_of(ids.tab_foo).set(ids.foo_label, ui);
        text(widget::Text::new("Bar!")).middle_of(ids.tab_bar).set(ids.bar_label, ui);
        text(widget::Text::new("BAZ!")).middle_of(ids.tab_baz).set(ids.baz_label, ui);

        let footer_wh = ui.wh_of(ids.footer).unwrap();
        let mut elements = widget::Matrix::new(ConrodUi::COLS, ConrodUi::ROWS)
            .w_h(footer_wh[0], footer_wh[1] * 2.0)
            .mid_top_of(ids.footer)
            .set(ids.button_matrix, ui);
        while let Some(elem) = elements.next(ui) {
            let (r, c) = (elem.row, elem.col);
            let n = c + r * c;
            let luminance = n as f32 / (ConrodUi::COLS * ConrodUi::ROWS) as f32;
            let button = widget::Button::new().color(color::BLUE.with_luminance(luminance));
            for _click in elem.set(button, ui) {
                println!("Hey! {:?}", (r, c));
            }
        }

        let button = widget::Button::new().color(color::RED).w_h(30.0, 30.0);
        for _click in button.clone().middle_of(ids.floating_a).set(ids.bing, ui) {
            println!("Bing!");
        }
        for _click in button.middle_of(ids.floating_b).set(ids.bong, ui) {
            println!("Bong!");
        }
    }
}

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    struct Ids {
        master,
        header,
        body,
        left_column,
        middle_column,
        right_column,
        footer,
        footer_scrollbar,
        floating_a,
        floating_b,
        tabs,
        tab_foo,
        tab_bar,
        tab_baz,

        title,
        subtitle,
        top_left,
        bottom_right,
        foo_label,
        bar_label,
        baz_label,
        button_matrix,
        bing,
        bong,
    }
}

/// In most of the examples the `glutin` crate is used for providing the window context and
/// events while the `glium` crate is used for displaying `conrod::render::Primitives` to the
/// screen.
///
/// This `Iterator`-like type simplifies some of the boilerplate involved in setting up a
/// glutin+glium event loop that works efficiently with conrod.
pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {
    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, events_loop: &mut glium::glutin::EventsLoop) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events_loop.poll_events(|event| events.push(event));

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}