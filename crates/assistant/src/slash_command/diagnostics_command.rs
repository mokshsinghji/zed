use std::sync::Arc;

use anyhow::Result;
use assistant_slash_command::{SlashCommand, SlashCommandOutput};
use gpui::{Task, WeakView};
use language::LspAdapterDelegate;
use ui::prelude::*;
use workspace::Workspace;

pub struct DiagnosticsSlashCommand;

impl SlashCommand for DiagnosticsSlashCommand {
    fn name(&self) -> String {
        "diagnostics".into()
    }

    fn description(&self) -> String {
        "insert project diagnostics".into()
    }

    fn menu_text(&self) -> String {
        "Insert Project Diagnostics".into()
    }

    fn complete_argument(
        &self,
        query: String,
        cancel: Arc<std::sync::atomic::AtomicBool>,
        workspace: gpui::WeakView<Workspace>,
        cx: &mut gpui::AppContext,
    ) -> Task<Result<Vec<String>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn run(
        self: Arc<Self>,
        argument: Option<&str>,
        workspace: WeakView<Workspace>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut WindowContext,
    ) -> Task<Result<SlashCommandOutput>> {
        let output = workspace.update(cx, |workspace, cx| {
            let mut buffers = Vec::new();
            workspace.project().update(cx, |project, cx| {
                for (path, _, summary) in
                    project.diagnostic_summaries(false, cx).collect::<Vec<_>>()
                {
                    // todo!("here we could exclude warnings")
                    //
                    buffers.push(project.open_buffer(path, cx));
                }
            });

            cx.spawn(|_, cx| async move {
                let buffers = futures::future::try_join_all(buffers).await?;
                let mut text = String::new();
                for buffer in buffers {
                    let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot())?;
                    for (_, group) in snapshot.diagnostic_groups(None) {}
                }

                todo!()
            })
        });

        output.unwrap_or_else(|error| Task::ready(Err(error)))
    }
}
