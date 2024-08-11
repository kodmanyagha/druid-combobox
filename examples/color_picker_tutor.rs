// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]
#![allow(unused_imports)]

use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use std::sync::Mutex;

use druid::kurbo::BezPath;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::{prelude::*, Container, Flex, Label, Split};
use druid::{
    Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout,
    WindowDesc,
};
use log::info;

struct ColorPickerTutor {
    size: Size,
}

impl ColorPickerTutor {
    pub fn new() -> Self {
        ColorPickerTutor {
            size: Size::new(100_f64, 50_f64),
        }
    }
}

fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    let mut t = t;
    if t < 0. {
        t += 1.
    }
    if t > 1. {
        t -= 1.
    };
    if t < 1. / 6. {
        return p + (q - p) * 6. * t;
    }
    if t < 1. / 2. {
        return q;
    }
    if t < 2. / 3. {
        return p + (q - p) * (2. / 3. - t) * 6.;
    }

    p
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let r;
    let g;
    let b;

    if s == 0.0 {
        r = l;
        g = l;
        b = l; // achromatic
    } else {
        let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };

        let p = 2. * l - q;
        r = hue_to_rgb(p, q, h + 1. / 3.);
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - 1. / 3.);
    }

    (
        (r * 255.).round() as u8,
        (g * 255.).round() as u8,
        (b * 255.).round() as u8,
    )
}

impl Widget<AppState> for ColorPickerTutor {
    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle, //
        data: &AppState,
        env: &Env,
    ) {
        info!("lifecycle fn invoked");
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, env: &Env) {
        info!("update fn invoked");
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &AppState,
        env: &Env,
    ) -> Size {
        info!("layout");

        self.size.clone()
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        // info!("event, {:?} {:?}", event, data);

        match event {
            Event::WindowConnected => info!("Event::WindowConnected occured"),

            Event::WindowCloseRequested => info!("Event::WindowCloseRequested occured"),

            Event::WindowDisconnected => info!("Event::WindowDisconnected occured"),

            Event::ImeStateChange => info!("Event::ImeStateChange occured"),

            Event::WindowScale(window_scale) => info!(
                "Event::WindowScale occured, window_scale: {:?}",
                window_scale
            ),

            Event::WindowSize(window_size) => {
                info!("Event::WindowSize occured, window_size: {:?}", window_size)
            }

            Event::MouseDown(mouse_down) => {
                info!("Event::MouseDown occured, mouse_down: {:?}", mouse_down)
            }

            Event::MouseUp(mouse_up) => info!("Event::MouseUp occured, mouse_up: {:?}", mouse_up),

            Event::MouseMove(mouse_move) => {
                info!("Event::MouseMove occured, mouse_move: {:?}", mouse_move);

                *data.mouse_location.borrow_mut() = Rc::new(Box::new(Some(Point2d(
                    mouse_move.pos.x as u8,
                    mouse_move.pos.y as u8,
                ))));
            }

            Event::Wheel(wheel) => info!("Event::Wheel occured, wheel: {:?}", wheel),

            Event::KeyDown(key_down) => info!("Event::KeyDown occured, key_down: {:?}", key_down),

            Event::KeyUp(key_up) => info!("Event::KeyUp occured, key_up: {:?}", key_up),

            Event::Paste(paste) => info!("Event::Paste occured, paste: {:?}", paste),

            Event::Zoom(zoom) => info!("Event::Zoom occured, zoom: {:?}", zoom),

            Event::Timer(timer) => info!("Event::Timer occured, timer: {:?}", timer),

            Event::AnimFrame(anim_frame) => {
                info!("Event::AnimFrame occured, anim_frame: {:?}", anim_frame)
            }

            Event::Command(command) => info!("Event::Command occured, command: {:?}", command),

            Event::Notification(notification) => info!(
                "Event::Notification occured, notification: {:?}",
                notification
            ),

            Event::Internal(internal) => info!("Event::Internal occured, internal: {:?}", internal),
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let rgb = hsl_to_rgb(0.5, 0.5, 0.5);
        let rect = Rect::from_origin_size(Point::ORIGIN, self.size);
        ctx.fill(rect, &Color::rgb8(rgb.0, rgb.1, rgb.2));
    }
}

#[derive(Clone, Data, Debug)]
struct Point2d(u8, u8);

#[derive(Clone, Data, Default, Debug)]
struct AppState {
    pub mouse_location: Rc<Box<Option<Point2d>>>,
    pub clicks: Rc<Box<Vec<Point2d>>>,
}

fn build_ui() -> impl Widget<AppState> {
    Split::columns(
        Container::new(
            Flex::column()
                .with_flex_child(Label::new("Example label 1"), 1.0)
                .with_flex_child(Label::new("Example label 2"), 1.0),
        )
        .border(Color::grey(0.6), 2.0),
        Container::new(
            Flex::column()
                .with_flex_child(
                    Label::dynamic(|data: &AppState, _| {
                        let mouse_location = *(*(data.mouse_location.clone())).clone();

                        if let Some(loc) = mouse_location {
                            return format!("Location: {:?}", loc);
                        }

                        "Mouse is out of the area.".to_string()
                    }),
                    2.0,
                )
                .with_flex_child(ColorPickerTutor::new(), 2.0),
        ),
    )
}

pub fn main() -> anyhow::Result<()> {
    println!("Current dir: {:?}", env::current_dir());

    dotenvy::dotenv()?;
    env_logger::init();

    let window = WindowDesc::new(build_ui()).title(LocalizedString::new("Color picker tutor"));

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(AppState::default())
        .expect("launch failed");

    Ok(())
}
