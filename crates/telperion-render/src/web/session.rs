//! The timing protocol as a page runs it.
//!
//! The same conditioning, warmup and measured frames the native session runs,
//! awaited one readback at a time - and, for an orbit, a window of the
//! browser's own animation clock afterwards. The two are measured apart on
//! purpose: a frame that carries a timestamp readback is paced by the readback
//! and not by the display, so the cadence a viewer would have seen is read off
//! frames drawn exactly the way the page draws them, with nothing between one
//! and the next but the frame itself.
use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::{prelude::*, JsCast};

use super::{borrow, js_error, Live};
use crate::{
    orbit_pose,
    timing::{Hardware, Report, Session, CONDITIONING, MEASURED, WARMUP},
    Camera,
};

/// How long the page is asked to hold its frame rate: ten seconds of turning
/// at whatever the display refreshes at, which is the window the browser's
/// frame budget is judged over.
const ORBIT_SECONDS: f64 = 10.0;

/// The plain session, with the camera held where the page left it.
pub(super) async fn measure(live: &Rc<RefCell<Option<Live>>>) -> Result<String, JsError> {
    let held = borrow(live)?.camera;
    collect(live, move |_| held, false).await
}

/// The orbit session: the protocol first, then ten seconds of one full turn
/// about the pose the page is at, timed by the browser's animation clock.
pub(super) async fn orbit(live: &Rc<RefCell<Option<Live>>>) -> Result<String, JsError> {
    let hero = borrow(live)?.camera;
    collect(live, move |turn| orbit_pose(&hero, turn), true).await
}

/// One session: conditioning frames, then the GPU protocol, then - when the
/// camera turns - the wall-clock window. `pose` is given how far through the
/// turn its frame is, so a still session simply ignores it.
async fn collect(
    live: &Rc<RefCell<Option<Live>>>,
    pose: impl Fn(f64) -> Camera,
    turning: bool,
) -> Result<String, JsError> {
    let hardware = Hardware::from(&borrow(live)?.renderer.gpu().adapter);
    let session = Session::new(borrow(live)?.renderer.gpu());
    for _ in 0..CONDITIONING {
        draw(live, pose(0.0))?;
    }

    let report = match session {
        Ok(session) => sampled(live, &pose, hardware, &session).await?,
        // No timestamps is not no measurement. The frames still run, and on an
        // orbit the page's own clock is the number the budget is judged on.
        Err(reason) => Report::unavailable(hardware, reason),
    };
    if !turning {
        return Ok(report.to_json());
    }

    let waits = window(live, &pose).await?;
    // The canvas is left at the pose it was asked to measure, not wherever the
    // turn happened to end.
    draw(live, pose(0.0))?;
    Ok(report.with_wall(&waits).to_json())
}

/// The GPU half: warmup frames thrown away, then the measured frames, each
/// awaited on its own readback. The camera advances through the turn across
/// them, so what is timed is an orbit's frames and not a still's.
async fn sampled(
    live: &Rc<RefCell<Option<Live>>>,
    pose: &impl Fn(f64) -> Camera,
    hardware: Hardware,
    session: &Session,
) -> Result<Report, JsError> {
    let mut vegetation = Vec::with_capacity(MEASURED);
    let mut selection = Vec::with_capacity(MEASURED);
    for index in 0..WARMUP + MEASURED {
        // The borrow is put down before the await, so nothing holds the canvas
        // while the browser is carrying the readback.
        {
            let mut canvas = borrow(live)?;
            canvas.camera = pose(index.saturating_sub(WARMUP) as f64 / MEASURED as f64);
            canvas
                .draw(Some((session.writes(), session.selection_writes())))
                .map_err(js_error)?;
            session.resolve(canvas.renderer.gpu());
        }
        let (pass, select) = session.sample_ms().await.map_err(js_error)?;
        if index >= WARMUP {
            vegetation.push(pass);
            selection.push(select);
        }
    }
    Ok(Report::measured(hardware, &vegetation).with_selection(&vegetation, &selection))
}

/// The wall-clock half: frames drawn the way the page draws them, for the
/// length of the window, timed by the animation clock the browser hands its
/// own loops. What comes back is how long the page waited from each frame to
/// the next, in milliseconds.
async fn window(
    live: &Rc<RefCell<Option<Live>>>,
    pose: &impl Fn(f64) -> Camera,
) -> Result<Vec<f64>, JsError> {
    let span = ORBIT_SECONDS * 1e3;
    let start = animation_frame().await?;
    let mut previous = start;
    let mut waits = Vec::new();
    loop {
        draw(live, pose((previous - start) / span))?;
        let now = animation_frame().await?;
        waits.push(now - previous);
        previous = now;
        if now - start >= span {
            return Ok(waits);
        }
    }
}

/// One untimed frame at a pose, with the canvas borrowed no longer than the
/// draw itself.
fn draw(live: &Rc<RefCell<Option<Live>>>, camera: Camera) -> Result<(), JsError> {
    let mut canvas = borrow(live)?;
    canvas.camera = camera;
    canvas.draw(None).map_err(js_error)
}

/// The next animation frame's timestamp, in milliseconds on the page's own
/// clock: the number `requestAnimationFrame` hands any loop the browser paces.
async fn animation_frame() -> Result<f64, JsError> {
    let window = web_sys::window().ok_or_else(|| JsError::new("there is no window to draw in"))?;
    let (sender, receiver) = futures_channel::oneshot::channel();
    let callback = Closure::once_into_js(move |time: f64| {
        let _ = sender.send(time);
    });
    window
        .request_animation_frame(callback.unchecked_ref())
        .map_err(|_| JsError::new("the browser refused an animation frame"))?;
    receiver
        .await
        .map_err(|_| JsError::new("the animation frame never arrived"))
}
