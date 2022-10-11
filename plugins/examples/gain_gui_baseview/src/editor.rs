use atomic_float::AtomicF32;
use baseview::{gl::GlConfig, Event, EventStatus, Window, WindowHandler, WindowScalePolicy};
use nih_plug::prelude::{util, Editor};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::time::Duration;

use crate::GainParams;

/// VIZIA uses points instead of pixels for text
const POINT_SCALE: f32 = 0.75;

const STYLE: &str = r#""#;

#[derive(Lens)]
struct Data {
    params: Arc<GainParams>,
    peak_meter: Arc<AtomicF32>,
}

impl Model for Data {}

struct BaseviewEditor {}

struct MemoryLeakExample {}

impl WindowHandler for MemoryLeakExample {
    fn on_frame(&mut self, _window: &mut Window) {}

    fn on_event(&mut self, _window: &mut Window, _event: Event) -> EventStatus {
        EventStatus::Ignored
    }
}

impl Editor for BaseviewEditor {
    fn spawn(
        &self,
        parent: nih_plug::prelude::ParentWindowHandle,
        context: Arc<dyn nih_plug::prelude::GuiContext>,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        let window_open_options = baseview::WindowOpenOptions {
            title: "memory leak example".into(),
            size: baseview::Size::new(100.0, 100.0),
            scale: WindowScalePolicy::SystemScaleFactor,
            gl_config: Some(GlConfig::default()),
        };

        let window = Window::open_parented(&parent, window_open_options, |window| {
            let ctx = window.gl_context();
            MemoryLeakExample {}
        });

        Box::new([0.0; 10000000])
    }

    fn size(&self) -> (u32, u32) {
        (10, 10)
    }

    fn set_scale_factor(&self, factor: f32) -> bool {
        false
    }

    fn param_values_changed(&self) {}
}

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::from_size(200, 150)
}

pub(crate) fn create(
    params: Arc<GainParams>,
    peak_meter: Arc<AtomicF32>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    let editor = BaseviewEditor {};
    Some(Box::new(editor))
}
