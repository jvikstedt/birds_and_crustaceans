use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::{Query, Res},
    text::Text,
    ui::{Display, Style},
};

use crate::{component::DebugWindow, resource::Opt, rollback::RollbackDiagnostics};

pub fn update_debug_window(
    mut text_q: Query<(&DebugWindow, &mut Text, &mut Style)>,
    diagnostics: Res<Diagnostics>,
    rollback_diagnostics: Option<Res<RollbackDiagnostics>>,
    opt: Res<Opt>,
) {
    if let Ok((debug_window, mut text, mut style)) = text_q.get_single_mut() {
        if debug_window.visible {
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
            return;
        }

        let mut str: String = String::new();

        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                str.push_str(&format!("\nfps: {:.2}", average));
            }
        }

        if let Some(rollback_diagnostics) = rollback_diagnostics {
            let frame_diff = (rollback_diagnostics.target_frame as i32)
                - (rollback_diagnostics.last_confirmed_frame as i32);

            let latency = if rollback_diagnostics.update_frequency > 0 {
                (frame_diff - 2) * (1000 / rollback_diagnostics.update_frequency as i32)
            } else {
                0
            };
            str.push_str(&format!(
                "\
            \nlast_confirmed_frame: {:}\
            \nlocal_frame: {:}\
            \ntarget_frame: {:}\
            \nsnapshot_frame: {:}\
            \nframe_diff: {:}\
            \nlatency~: {:}\
            \ninput_lag: {:}\
            \nremote_player_delay: {:}\
            \nrun_speed: {:}\
            \nupdate_frequency: {:}\
            \nremote_frame_diff: {:}\
            ",
                rollback_diagnostics.last_confirmed_frame,
                rollback_diagnostics.local_frame,
                rollback_diagnostics.target_frame,
                rollback_diagnostics.snapshot_frame,
                frame_diff,
                latency,
                rollback_diagnostics.input_lag,
                opt.remote_player_delay,
                rollback_diagnostics.run_speed,
                rollback_diagnostics.update_frequency,
                rollback_diagnostics.remote_frame_diff,
            ));
        }

        text.sections[1].value = str;
    }
}
