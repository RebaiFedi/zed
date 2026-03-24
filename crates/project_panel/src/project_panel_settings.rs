use editor::EditorSettings;
use gpui::Pixels;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{
    DockSide, ProjectPanelEntrySpacing, ProjectPanelSortMode, RegisterSetting, Settings,
    ShowDiagnostics, ShowIndentGuides,
};
use ui::{
    px,
    scrollbars::{ScrollbarVisibility, ShowScrollbar},
};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, RegisterSetting)]
pub struct ProjectPanelSettings {
    pub button: bool,
    pub hide_gitignore: bool,
    pub default_width: Pixels,
    pub dock: DockSide,
    pub entry_spacing: ProjectPanelEntrySpacing,
    pub file_icons: bool,
    pub folder_icons: bool,
    pub git_status: bool,
    pub indent_size: f32,
    pub indent_guides: IndentGuidesSettings,
    pub sticky_scroll: bool,
    pub auto_reveal_entries: bool,
    pub auto_fold_dirs: bool,
    pub bold_folder_labels: bool,
    pub starts_open: bool,
    pub scrollbar: ScrollbarSettings,
    pub show_diagnostics: ShowDiagnostics,
    pub hide_root: bool,
    pub hide_hidden: bool,
    pub drag_and_drop: bool,
    pub auto_open: AutoOpenSettings,
    pub sort_mode: ProjectPanelSortMode,
    pub diagnostic_badges: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct IndentGuidesSettings {
    pub show: ShowIndentGuides,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct ScrollbarSettings {
    /// When to show the scrollbar in the project panel.
    ///
    /// Default: inherits editor scrollbar settings
    pub show: Option<ShowScrollbar>,
    /// Whether to allow horizontal scrolling in the project panel.
    /// When false, the view is locked to the leftmost position and long file names are clipped.
    ///
    /// Default: true
    pub horizontal_scroll: bool,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
pub struct AutoOpenSettings {
    pub on_create: bool,
    pub on_paste: bool,
    pub on_drop: bool,
}

impl AutoOpenSettings {
    #[inline]
    pub fn should_open_on_create(self) -> bool {
        self.on_create
    }

    #[inline]
    pub fn should_open_on_paste(self) -> bool {
        self.on_paste
    }

    #[inline]
    pub fn should_open_on_drop(self) -> bool {
        self.on_drop
    }
}

impl ScrollbarVisibility for ProjectPanelSettings {
    fn visibility(&self, cx: &ui::App) -> ShowScrollbar {
        self.scrollbar
            .show
            .unwrap_or_else(|| EditorSettings::get_global(cx).scrollbar.show)
    }
}

impl Settings for ProjectPanelSettings {
    fn from_settings(content: &settings::SettingsContent) -> Self {
        let project_panel = content.project_panel.clone().expect("project_panel settings must exist");
        Self {
            button: project_panel.button.expect("project_panel.button must exist"),
            hide_gitignore: project_panel.hide_gitignore.expect("project_panel.hide_gitignore must exist"),
            default_width: px(project_panel.default_width.expect("project_panel.default_width must exist")),
            dock: project_panel.dock.expect("project_panel.dock must exist"),
            entry_spacing: project_panel.entry_spacing.expect("project_panel.entry_spacing must exist"),
            file_icons: project_panel.file_icons.expect("project_panel.file_icons must exist"),
            folder_icons: project_panel.folder_icons.expect("project_panel.folder_icons must exist"),
            git_status: project_panel.git_status.expect("project_panel.git_status must exist")
                && content
                    .git
                    .as_ref()
                    .expect("git settings must exist")
                    .enabled
                    .expect("git.enabled must exist")
                    .is_git_status_enabled(),
            indent_size: project_panel.indent_size.expect("project_panel.indent_size must exist"),
            indent_guides: IndentGuidesSettings {
                show: project_panel.indent_guides.expect("project_panel.indent_guides must exist").show.expect("indent_guides.show must exist"),
            },
            sticky_scroll: project_panel.sticky_scroll.expect("project_panel.sticky_scroll must exist"),
            auto_reveal_entries: project_panel.auto_reveal_entries.expect("project_panel.auto_reveal_entries must exist"),
            auto_fold_dirs: project_panel.auto_fold_dirs.expect("project_panel.auto_fold_dirs must exist"),
            bold_folder_labels: project_panel.bold_folder_labels.expect("project_panel.bold_folder_labels must exist"),
            starts_open: project_panel.starts_open.expect("project_panel.starts_open must exist"),
            scrollbar: {
                let scrollbar = project_panel.scrollbar.expect("project_panel.scrollbar must exist");
                ScrollbarSettings {
                    show: scrollbar.show.map(Into::into),
                    horizontal_scroll: scrollbar.horizontal_scroll.expect("scrollbar.horizontal_scroll must exist"),
                }
            },
            show_diagnostics: project_panel.show_diagnostics.expect("project_panel.show_diagnostics must exist"),
            hide_root: project_panel.hide_root.expect("project_panel.hide_root must exist"),
            hide_hidden: project_panel.hide_hidden.expect("project_panel.hide_hidden must exist"),
            drag_and_drop: project_panel.drag_and_drop.expect("project_panel.drag_and_drop must exist"),
            auto_open: {
                let auto_open = project_panel.auto_open.expect("project_panel.auto_open must exist");
                AutoOpenSettings {
                    on_create: auto_open.on_create.expect("auto_open.on_create must exist"),
                    on_paste: auto_open.on_paste.expect("auto_open.on_paste must exist"),
                    on_drop: auto_open.on_drop.expect("auto_open.on_drop must exist"),
                }
            },
            sort_mode: project_panel.sort_mode.expect("project_panel.sort_mode must exist"),
            diagnostic_badges: project_panel.diagnostic_badges.expect("project_panel.diagnostic_badges must exist"),
        }
    }
}
