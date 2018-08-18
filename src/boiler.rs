use conrod;
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use eventloop::EventLoop;

use ttf_noto_sans;

use app::App;
use conrod::Widget;

static TITLE: &str = "My cool app";

const INITIAL_WINDOW_WIDTH: u32 = 800;
const INITIAL_WINDOW_HEIGHT: u32 = 500;

widget_ids!(struct Ids {
    app
});

pub fn boil() {

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title(TITLE)
        .with_dimensions((INITIAL_WINDOW_WIDTH, INITIAL_WINDOW_HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([INITIAL_WINDOW_WIDTH as f64, INITIAL_WINDOW_HEIGHT as f64]).build();

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    //let font_path = Path::new("res/NotoSans/NotoSans-Regular.ttf");
    //ui.fonts.insert_from_file(font_path).unwrap();
    ui.fonts.insert(conrod::text::FontCollection::from_bytes(ttf_noto_sans::REGULAR).unwrap().into_font().unwrap());

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let app_id = Ids::new(ui.widget_id_generator()).app;

    // Poll events from the window.
    let mut event_loop = EventLoop::new();
    'main: loop {
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
                    glium::glutin::WindowEvent::CloseRequested
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // Instantiate all widgets in the GUI.
        {
            App::new()
                .set(app_id, &mut ui.set_widgets())
        }

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
