// Copyright 2019 the Druid Authors
// SPDX-License-Identifier: Apache-2.0

//! An example of a custom drawing widget.
//! We draw an image, some text, a shape, and a curve.

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use std::env;

use druid::kurbo::BezPath;
use druid::piet::{FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
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

    return (
        (r * 255.).round() as u8,
        (g * 255.).round() as u8,
        (b * 255.).round() as u8,
    );
}

impl Widget<String> for ColorPickerTutor {
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &String, env: &Env) {
        info!("lifecycle");
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &String, data: &String, env: &Env) {
        info!("update");
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &String,
        env: &Env,
    ) -> Size {
        info!("layout");

        self.size.clone()
    }

    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
        info!("event, {:?} {:?}", event, data);

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
                info!("Event::MouseMove occured, mouse_move: {:?}", mouse_move)
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

    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
        //info!("paint");

        let rgb = hsl_to_rgb(0.5, 0.5, 0.5);
        let rect = Rect::from_origin_size(Point::ORIGIN, self.size);
        ctx.fill(rect, &Color::rgb8(rgb.0, rgb.1, rgb.2));
    }
}

pub fn main() -> anyhow::Result<()> {
    println!("Current dir: {:?}", env::current_dir());

    dotenvy::dotenv()?;
    env_logger::init();

    let window = WindowDesc::new(ColorPickerTutor::new()) //
        .title(LocalizedString::new("Color picker tutor"));

    AppLauncher::with_window(window)
        .log_to_console()
        .launch("Druid + Piet".to_string())
        .expect("launch failed");

    Ok(())
}
