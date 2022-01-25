use serialport::SerialPort;
use crate::cli::*;
pub struct Ctx {
    pub port: Box<dyn SerialPort>,
    pub sdl_context: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
    pub event_pump: sdl2::EventPump,
}

impl Ctx {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let Cli { baud_rate, port } = Cli::parse();
        let mut port = serialport::new(port, baud_rate).open()?;
        port.set_timeout(std::time::Duration::from_secs(5))?;

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem.window("KbGrab", 400, 300)
            .position_centered()
            .build()?;

        let canvas = window.into_canvas().build()?;
        let event_pump = sdl_context.event_pump()?;

        Ok(Self {
            port,
            sdl_context,
            video_subsystem,
            canvas,
            event_pump
        })
    }
}