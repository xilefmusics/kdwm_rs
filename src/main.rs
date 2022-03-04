pub mod backend;
pub mod c;

use backend::{Backend, Event, Input, Key};

fn main() -> Result<(), String> {
    let backend = Backend::new()?;
    let (x, y) = backend.screen_dimensions()?;

    println!("{:?}", backend);
    println!("dimensions: {}x{}", x, y);

    backend.select_inputs(vec![
        Input::SubstructureNotify,
        Input::SubstructureRedirect,
        Input::KeyPress,
    ])?;

    backend.run(|backend, event| {
        println!("Event: {:?}", event);
        if let Event::KeyPress(event) = event {
            if let Key::Q = event.key {
                backend.stop();
            }
        }
    })?;

    backend.close()?;

    Ok(())
}
