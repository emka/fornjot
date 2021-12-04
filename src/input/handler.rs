use std::time::Instant;

use nalgebra::Translation2;
use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode,
    },
};

use crate::{camera::Camera, math::Point};

use super::{movement::Movement, rotation::Rotation, zoom::Zoom};

pub struct Handler {
    cursor: Option<PhysicalPosition<f64>>,

    movement: Movement,
    rotation: Rotation,
    zoom: Zoom,
}

impl Handler {
    pub fn new(now: Instant) -> Self {
        Self {
            cursor: None,

            movement: Movement::new(),
            rotation: Rotation::new(),
            zoom: Zoom::new(now),
        }
    }

    pub fn cursor(&self) -> Option<PhysicalPosition<f64>> {
        self.cursor
    }

    pub fn handle_keyboard_input(
        &mut self,
        input: KeyboardInput,
        actions: &mut Actions,
    ) {
        if let KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(virtual_key_code),
            ..
        } = input
        {
            match virtual_key_code {
                VirtualKeyCode::Escape => actions.exit = true,

                VirtualKeyCode::Key1 => actions.toggle_model = true,
                VirtualKeyCode::Key2 => actions.toggle_mesh = true,

                _ => (),
            }
        }
    }

    pub fn handle_cursor_moved(
        &mut self,
        cursor: PhysicalPosition<f64>,
        camera: &mut Camera,
    ) {
        if let Some(previous) = self.cursor {
            let diff_x = cursor.x - previous.x;
            let diff_y = cursor.y - previous.y;

            let f = 0.005;

            let angle_x = diff_y * f;
            let angle_y = diff_x * f;

            self.rotation.apply(angle_x, angle_y, camera);

            if self.movement.started {
                // TASK: Moving feels good, if you're dragging the model exactly
                //       where your mouse goes. It feels weird, if the mouse
                //       cursor moves faster or slower than the model you're
                //       moving.
                //
                //       The following factor achieves this good-feeling move
                //       for relatively small models at the default distance
                //       between camera and model origin. It breaks down when
                //       moving the camera closer or away from the model, which
                //       is the far more common case.
                //
                //       It would be nicer to have a zoom factor that depends on
                //       the distance between camera and model origin, or even
                //       the distance between the camera and the part of the
                //       model the mouse is currently pointing at (or more
                //       precisely, the distance between the camera and a plane
                //       that touches the surface of the model where the mouse
                //       is pointing, and whose normal is parallel to the
                //       camera's viewing direction).
                let f = 0.2;

                let x_trans = diff_x * f;
                let y_trans = -diff_y * f;

                let translation = Translation2::new(x_trans, y_trans);

                camera.translation = translation * camera.translation;
            }
        }

        self.cursor = Some(cursor);
    }

    pub fn handle_mouse_input(
        &mut self,
        button: MouseButton,
        state: ElementState,
        focus_point: Option<Point>,
    ) {
        match (button, state) {
            (MouseButton::Left, ElementState::Pressed) => {
                self.rotation.start(focus_point);
            }
            (MouseButton::Left, ElementState::Released) => {
                self.rotation.stop();
            }
            (MouseButton::Right, ElementState::Pressed) => {
                self.movement.start();
            }
            (MouseButton::Right, ElementState::Released) => {
                self.movement.stop();
            }
            _ => {}
        }
    }

    pub fn handle_mouse_wheel(
        &mut self,
        delta: MouseScrollDelta,
        now: Instant,
    ) {
        let delta = match delta {
            MouseScrollDelta::LineDelta(_, y) => y as f64 * 10.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => y,
        };

        self.zoom.push_input_delta(delta, now);
    }

    pub fn update(&mut self, delta_t: f64, now: Instant, camera: &mut Camera) {
        self.zoom.discard_old_events(now);
        self.zoom.update_speed(now, delta_t);

        camera.distance += self.zoom.speed();
    }
}

pub struct Actions {
    pub exit: bool,

    pub toggle_model: bool,
    pub toggle_mesh: bool,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            exit: false,

            toggle_model: false,
            toggle_mesh: false,
        }
    }
}