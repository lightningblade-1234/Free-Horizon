//! The personal Local / Offline window (`horizon-launcher --local`). It uses the launcher's
//! version picker, instance folders, console and process monitor, with a session-free start.
use super::*;

pub(super) struct Local {
    name: line_edit::LineEdit,
    focus: FocusHandle,
}

impl Local {
    pub(super) fn new(name: String, cx: &mut App) -> Self {
        Self {
            // Leave room for invalid input too: report it instead of silently truncating a name.
            name: line_edit::LineEdit::new(name, 256, 32),
            focus: cx.focus_handle().tab_index(0).tab_stop(true),
        }
    }
}

impl Launcher {
    pub(super) fn local_topbar(&self, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let mut tabs = div().flex().flex_wrap().items_center().gap(px(8.));
        for (page, label) in [
            (Page::Play, "Local / Offline"),
            (Page::Versions, "Versions"),
            (Page::Settings, "Settings"),
        ] {
            let tab = if t.glass {
                glass_ui::tab(
                    format!("local-nav-{label}"),
                    tr(label),
                    self.page == page,
                    &t,
                )
            } else {
                shell::tab(
                    format!("local-nav-{label}"),
                    tr(label),
                    self.page == page,
                    &t,
                )
            };
            tabs = tabs.child(
                tab.on_click(cx.listener(move |this, _: &ClickEvent, _, cx| {
                    this.go(page);
                    cx.notify();
                })),
            );
        }
        div()
            .flex()
            .flex_none()
            .items_center()
            .gap(px(24.))
            .px(px(theme::PAGE_GUTTER))
            .py(px(16.))
            .child(mark(24., &t))
            .child(shell::text(theme::night::TITLE, t.ink).child(tr("Horizon")))
            .child(tabs)
    }

    /// All installed builds, including releases whose online release list is not fetched here.
    pub(super) fn local_versions_page(&self, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let mut page = div().flex().flex_col().gap(px(16.)).child(page_header(
            tr("Versions"),
            tr("Local / Offline"),
            &t,
        ));
        if self.installed.is_empty() {
            page = page.child(notice(tr("No version installed yet.").to_string(), &t));
        }
        for tag in self.installed.iter().chain(&self.removing) {
            page = page.child(self.local_build_row(tag, &|_| None, cx));
        }
        page
    }

    pub(super) fn local_page(&mut self, window: &Window, cx: &mut Context<Self>) -> Div {
        let t = self.theme;
        let focused = self.local.focus.is_focused(window);
        let invalid = backend::validate_local_name(self.local.name.text()).err();
        let base = if t.glass {
            glass_ui::field(focused, invalid.is_some())
        } else {
            div()
                .flex()
                .items_center()
                .h(px(theme::FIELD))
                .px(px(12.))
                .rounded(px(theme::RADIUS))
                .bg(t.surface)
                .border_1()
                .border_color(if invalid.is_some() {
                    t.danger
                } else if focused {
                    t.accent
                } else {
                    t.rule
                })
        };
        let field = base
            .id("local-player-name")
            .track_focus(&self.local.focus)
            .w_full()
            .max_w(px(360.))
            .cursor_text()
            .child(line_edit::content(
                &self.local.name,
                &t,
                focused,
                false,
                false,
                tr("Offline name"),
            ))
            .on_click(cx.listener(|this, _: &ClickEvent, window, cx| {
                window.focus(&this.local.focus, cx);
                cx.notify();
            }))
            .on_key_down(cx.listener(|this, ev: &KeyDownEvent, window, cx| {
                match line_edit::handle(&mut this.local.name, ev, cx) {
                    line_edit::Key::Pass => return,
                    line_edit::Key::Enter => this.play_local(cx),
                    line_edit::Key::Escape => window.blur(cx),
                    line_edit::Key::Edited => {
                        this.settings.local_player_name = this.local.name.text().to_string();
                        this.save_settings();
                        this.client_error = None;
                    }
                    _ => {}
                }
                cx.stop_propagation();
                cx.notify();
            }));
        let field = line_edit::mouse(field, cx, |this| &mut this.local.name);
        let starting =
            self.starting_client.is_some() || self.client.is_some() && self.play_pressed.is_some();
        let busy = self.client.is_some() || self.starting_client.is_some() || self.preparing;
        let status = if starting {
            "Starting"
        } else if self.client.is_some() {
            "Running"
        } else {
            "Play"
        };
        let play = button("local-play", tr(status).to_string(), Kind::Primary, &t)
            .h(px(theme::night::PLAY))
            .min_w(px(160.));
        let play = if busy {
            disabled(play)
        } else {
            play.on_click(cx.listener(|this, _: &ClickEvent, _, cx| {
                this.play_local(cx);
                cx.notify();
            }))
        };
        let tag = self.selected().cloned();
        let mut page = div()
            .flex()
            .flex_col()
            .gap(px(20.))
            .child(page_header(tr("Local / Offline"), tr("Offline · single player and LAN worlds"), &t))
            .child(shell::text(theme::night::UI, t.ink_soft).child(tr("Offline names play without a Microsoft account that owns the game. Minecraft still comes from your own install.")))
            .child(shell::text(theme::night::UI, t.ink).child(tr("Offline name")))
            .child(field)
            .children(invalid.map(|e| shell::text(theme::night::CAPTION, t.danger).child(e)))
            .child(div().flex().flex_wrap().items_center().gap(px(12.)).child(play).child(self.picker(cx)));
        match tag {
            Some(tag) if !backend::is_prepared(&tag) => {
                page = page.child(notice(backend::local_prepare_help(&tag), &t));
            }
            None => page = page.child(notice(tr("No version installed yet.").to_string(), &t)),
            _ => {}
        }
        if let Some(error) = &self.client_error {
            page = page.child(notice(error.clone(), &t));
        }
        if let Some(crash) = self.crash.clone() {
            page = page.child(self.crash_notice(crash, cx));
        }
        if starting {
            page = page.child(div().flex().child(progress_bar(None, &t)));
        }
        if let Some((header, console)) = self.console_section(cx) {
            page = page.child(header).child(console);
        }
        page
    }

    fn play_local(&mut self, cx: &mut Context<Self>) {
        if self.client.is_some() || self.starting_client.is_some() || self.preparing {
            return;
        }
        let name = self.local.name.text().to_string();
        if let Err(error) = backend::validate_local_name(&name) {
            self.client_error = Some(error);
            return;
        }
        let Some(instance) = self.selected_instance().cloned() else {
            self.client_error = Some(tr("No version installed yet.").to_string());
            return;
        };
        let tag = match instance.resolve_version(None) {
            Ok(tag) => tag,
            Err(error) => {
                self.client_error = Some(error);
                return;
            }
        };
        self.settings.local_player_name = name.clone();
        self.save_settings();
        self.client_error = None;
        self.crash = None;
        self.prepare_log.clear();
        self.starting_client = Some(tag.clone());
        self.play_pressed = Some(Instant::now());
        self.client_started = Some(std::time::SystemTime::now() - Duration::from_secs(2));
        let tx = self.tx.clone();
        let task = cx.background_executor().spawn(async move {
            let log = |line| {
                let _ = tx.send(Msg::Line(Job::Client, line));
            };
            let mut cmd = backend::local_client_with_log(&tag, &name, &log)?;
            instances::apply(&mut cmd, &instance)?;
            Ok::<_, String>((cmd, instance))
        });
        cx.spawn(async move |this, cx| {
            let result = task.await;
            let _ = this.update(cx, |this, cx| {
                this.starting_client = None;
                match result.and_then(|(cmd, instance)| {
                    this.client_home = Some(instance.home.clone());
                    this.client_vars = instance.vars(cmd.get_program());
                    backend::spawn(Job::Client, cmd, this.tx.clone()).map(|r| (r, instance.id))
                }) {
                    Ok((running, id)) => {
                        this.client = Some(running);
                        this.client_run = Some((id, Instant::now()));
                        this.step_aside(cx);
                    }
                    Err(error) => {
                        this.client_error = Some(error);
                        this.client_started = None;
                        this.play_pressed = None;
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }
}
