//! Native interactive workbench consuming the local UI Kit facade.
use ui_kit::gpui as gpui_kit;
mod catalog;
mod navigation_demo;
pub use catalog::STORIES;
use ui_kit::components::{
    Colorize, Size,
    alert::Alert,
    breadcrumb::{Breadcrumb, BreadcrumbItem},
    checkbox::Checkbox,
    input::{Input, InputEvent, InputState},
    menu::DropdownMenu,
    popover::Popover,
    progress::Progress,
    radio::RadioGroup,
    select::{Select, SelectEvent, SelectState},
    skeleton::Skeleton,
    slider::{Slider, SliderState},
    spinner::Spinner,
    switch::Switch,
    tab::{Tab, TabBar},
    table::{Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow},
    tag::Tag,
    text::TextView,
};
use ui_kit::{
    gpui::{self, AnyElement, ClipboardItem, FocusHandle, KeyBinding, TestSupportExt},
    prelude::*,
};

gpui::actions!(
    showcase,
    [
        /// Move to the next workbench section.
        NextSection,
        /// Move to the previous workbench section.
        PreviousSection,
        /// Collapse or expand primary navigation.
        ToggleNavigation,
        /// Quit the native application.
        Quit
    ]
);

/// Primary destinations in the workbench.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Section {
    /// Capabilities and integration status.
    #[default]
    Overview,
    /// Upstream live component stories.
    Components,
    /// Token exploration and theme registration.
    Themes,
    /// Reusable layout compositions.
    Layouts,
    /// Searchable SVG registry.
    Icons,
    /// Immediate property changes.
    Playground,
    /// Bundled integration documentation.
    Documentation,
}
impl Section {
    /// Stable navigation order.
    pub const ALL: [Self; 7] = [
        Self::Overview,
        Self::Components,
        Self::Themes,
        Self::Layouts,
        Self::Icons,
        Self::Playground,
        Self::Documentation,
    ];
    /// Visible destination label.
    pub fn title(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Components => "Components",
            Self::Themes => "Themes",
            Self::Layouts => "Layouts",
            Self::Icons => "Icons",
            Self::Playground => "Playground",
            Self::Documentation => "Documentation",
        }
    }
    fn key(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Components => "components",
            Self::Themes => "themes",
            Self::Layouts => "layouts",
            Self::Icons => "icons",
            Self::Playground => "playground",
            Self::Documentation => "documentation",
        }
    }
    fn icon(self) -> IconName {
        match self {
            Self::Overview => IconName::Home,
            Self::Components => IconName::Components,
            Self::Themes => IconName::Palette,
            Self::Layouts => IconName::Layout,
            Self::Icons => IconName::Icons,
            Self::Playground => IconName::Playground,
            Self::Documentation => IconName::Book,
        }
    }
}
/// Register app shortcuts once during application bootstrap.
pub fn init(cx: &mut App) {
    ui_kit::navigation::init(cx);
    cx.bind_keys([
        KeyBinding::new("ctrl-tab", NextSection, None),
        KeyBinding::new("ctrl-shift-tab", PreviousSection, None),
        KeyBinding::new(
            if cfg!(target_os = "macos") {
                "cmd-b"
            } else {
                "ctrl-b"
            },
            ToggleNavigation,
            None,
        ),
        KeyBinding::new(
            if cfg!(target_os = "macos") {
                "cmd-q"
            } else {
                "ctrl-q"
            },
            Quit,
            None,
        ),
    ]);
    cx.on_action(|_: &Quit, cx| cx.quit());
}

/// Retained state for all native stories. Entities and subscriptions survive page changes.
pub struct Showcase {
    navigation_demo: Entity<navigation_demo::NavigationDemo>,
    section: Section,
    collapsed: bool,
    focus: FocusHandle,
    icons: IconRegistry,
    label: Entity<InputState>,
    query: Entity<InputState>,
    primary: Entity<InputState>,
    fruit: Entity<SelectState<Vec<&'static str>>>,
    picker: Entity<SelectState<Vec<&'static str>>>,
    slider: Entity<SliderState>,
    checked: bool,
    enabled: bool,
    radio: usize,
    tab: usize,
    story: usize,
    variant: usize,
    button_size: Size,
    disabled: bool,
    loading: bool,
    selected_icon: IconName,
    icon_size: IconSize,
    icon_state: IconState,
    layout: usize,
    layout_item: usize,
    clicks: usize,
    status: SharedString,
    custom_number: usize,
    _subscriptions: Vec<Subscription>,
}
impl Showcase {
    /// Create the production workbench after runtime and ThemeService initialization.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let label = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("Save changes")
                .placeholder("Button label")
        });
        let query = cx.new(|cx| InputState::new(window, cx).placeholder("Search icons…"));
        let primary = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("#2563EB")
                .placeholder("#RRGGBB")
        });
        let fruit = cx.new(|cx| {
            SelectState::new(
                vec!["Apple", "Orange", "Pear"],
                Some(ui_kit::components::IndexPath::default()),
                window,
                cx,
            )
        });
        let picker = cx.new(|cx| {
            SelectState::new(
                STORIES.iter().map(|s| s.name).collect(),
                Some(ui_kit::components::IndexPath::default()),
                window,
                cx,
            )
        });
        let slider = cx.new(|_| {
            SliderState::new()
                .min(0.)
                .max(100.)
                .step(5.)
                .default_value(65.)
        });
        let mut subscriptions = Vec::new();
        for input in [&label, &query, &primary] {
            subscriptions.push(cx.subscribe_in(input, window, |_, _, event, _, cx| {
                if matches!(event, InputEvent::Change) {
                    cx.notify();
                }
            }));
        }
        subscriptions.push(
            cx.subscribe_in(&picker, window, |this, _, event, window, cx| {
                if let SelectEvent::Confirm(Some(name)) = event
                    && let Some(index) = STORIES.iter().position(|s| s.name == *name)
                {
                    this.navigation_demo
                        .update(cx, |demo, cx| demo.restore_focus(window, cx));
                    this.story = index;
                    cx.notify();
                }
            }),
        );
        subscriptions.push(cx.observe(&slider, |_, _, cx| cx.notify()));
        subscriptions.push(cx.observe_window_appearance(window, |this, window, cx| {
            if let Err(error) = ThemeService::sync_system(window, cx) {
                this.status = error.to_string().into();
            }
            cx.notify();
        }));
        let (icons, status) = match IconRegistry::built_in() {
            Ok(icons) => (icons, "Ready · local workspace source".into()),
            Err(error) => (IconRegistry::default(), error.to_string().into()),
        };
        let focus = cx.focus_handle();
        window.focus(&focus, cx);
        let navigation_demo =
            cx.new(|cx| navigation_demo::NavigationDemo::new(focus.clone(), window, cx));
        Self {
            navigation_demo,
            section: Section::Overview,
            collapsed: false,
            focus,
            icons,
            label,
            query,
            primary,
            fruit,
            picker,
            slider,
            checked: true,
            enabled: true,
            radio: 0,
            tab: 0,
            story: 0,
            variant: 0,
            button_size: Size::Medium,
            disabled: false,
            loading: false,
            selected_icon: IconName::Check,
            icon_size: IconSize::Medium,
            icon_state: IconState::Default,
            layout: 0,
            layout_item: 0,
            clicks: 0,
            status,
            custom_number: 0,
            _subscriptions: subscriptions,
        }
    }
    /// Active navigation destination, useful for application integration tests.
    pub fn section(&self) -> Section {
        self.section
    }
    /// Number of accepted preview activations.
    pub fn clicks(&self) -> usize {
        self.clicks
    }
    /// Number of current icon search matches.
    pub fn icon_matches(&self, cx: &App) -> usize {
        self.icons
            .search(self.query.read(cx).value().as_ref())
            .count()
    }
    fn navigate(&mut self, section: Section, window: &mut Window, cx: &mut Context<Self>) {
        self.navigation_demo
            .update(cx, |demo, cx| demo.restore_focus(window, cx));
        self.section = section;
        // Upstream buttons preserve pointer focus. Reanchor actions before an
        // input on the previous page leaves the dispatch tree.
        window.focus(&self.focus, cx);
        cx.notify();
    }
    fn next(&mut self, _: &NextSection, window: &mut Window, cx: &mut Context<Self>) {
        let i = Section::ALL
            .iter()
            .position(|s| *s == self.section)
            .unwrap_or(0);
        self.navigate(Section::ALL[(i + 1) % 7], window, cx);
    }
    fn previous(&mut self, _: &PreviousSection, window: &mut Window, cx: &mut Context<Self>) {
        let i = Section::ALL
            .iter()
            .position(|s| *s == self.section)
            .unwrap_or(0);
        self.navigate(Section::ALL[(i + 6) % 7], window, cx);
    }
    fn toggle_navigation(
        &mut self,
        _: &ToggleNavigation,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.collapsed = !self.collapsed;
        window.focus(&self.focus, cx);
        cx.notify();
    }
    fn set_theme(&mut self, mode: ThemeMode, window: &mut Window, cx: &mut Context<Self>) {
        self.status = match ThemeService::select(mode, Some(window), cx) {
            Ok(()) => format!("Theme: {}", ThemeService::global(cx).selected()).into(),
            Err(e) => e.to_string().into(),
        };
        cx.notify();
    }
    fn icon(&self, name: IconName, cx: &App) -> ui_kit::components::Icon {
        self.icons
            .render(&name.id(), IconSize::Medium, IconState::Default, cx)
            .unwrap_or_else(|_| ui_kit::components::Icon::empty())
    }
    fn preview_button(&self, cx: &mut Context<Self>) -> Button {
        let icon_color = match self.variant {
            0 => cx.theme().primary_foreground,
            3 => cx.theme().danger_foreground,
            _ => cx.theme().foreground,
        };
        let button = Button::new("preview-button")
            .label(self.label.read(cx).value())
            .icon(self.icon(self.selected_icon, cx).text_color(icon_color))
            .with_size(self.button_size)
            .disabled(self.disabled)
            .loading(self.loading)
            .on_click(cx.listener(|this, _, _, cx| {
                this.clicks += 1;
                this.status = format!("Preview activations: {}", this.clicks).into();
                cx.notify();
            }));
        match self.variant {
            1 => button.outline(),
            2 => button.ghost(),
            3 => button.danger(),
            _ => button.primary(),
        }
    }
    fn theme_controls(&self, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex().gap_1().children(
            [
                (ThemeMode::Light, "Light", "theme-light"),
                (ThemeMode::Dark, "Dark", "theme-dark"),
                (ThemeMode::System, "System", "theme-system"),
            ]
            .into_iter()
            .map(|(mode, label, id)| {
                Button::new(id)
                    .small()
                    .ghost()
                    .label(label)
                    .selected(ThemeService::global(cx).mode() == mode)
                    .on_click(
                        cx.listener(move |this, _, window, cx| this.set_theme(mode, window, cx)),
                    )
            }),
        )
    }
    fn navigation(&self, cx: &mut Context<Self>) -> impl IntoElement {
        sidebar_navigation()
            .child(
                div()
                    .px_2()
                    .py_4()
                    .text_lg()
                    .font_semibold()
                    .child("Rust UI Kit"),
            )
            .child(
                div()
                    .px_2()
                    .pb_4()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("Native component workbench"),
            )
            .child(
                navigation_group("LIBRARY").children(Section::ALL.into_iter().map(|section| {
                    Button::new(SharedString::from(format!("nav-{}", section.key())))
                        .ghost()
                        .w_full()
                        .justify_start()
                        .selected(self.section == section)
                        .icon(self.icon(section.icon(), cx))
                        .label(section.title())
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.navigate(section, window, cx)
                        }))
                })),
            )
            .child(div().flex_1())
            .child(
                div()
                    .text_xs()
                    .px_2()
                    .py_3()
                    .text_color(cx.theme().muted_foreground)
                    .child("v0.1.0 · GPUI Kit 0.9.1\nCtrl+Tab changes section"),
            )
    }
    fn overview(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex().gap_6().child(title("Build native. Reuse everything.","A Rust desktop foundation with shared components, tokens, layouts and icons.",cx))
            .child(dashboard().children([
                (STORIES.len().to_string(),"Live component stories"),(ThemeService::global(cx).registry().themes().count().to_string(),"Registered themes"),
                ("7".into(),"Layout compositions"),(self.icons.search("").count().to_string(),"Semantic SVG icons")
            ].into_iter().map(|(value,label)|card(cx).w_48().child(div().text_3xl().font_semibold().child(value)).child(muted(label,cx)))))
            .child(card(cx).gap_4().child(div().text_lg().font_semibold().child("One workspace, one design system"))
                .child("Standard controls come from GPUI Kit. Themes, icon registration and application compositions belong to reusable local libraries.")
                .child(code("ui-components → ui-kit → showcase\nui-theme → semantic tokens → native Button\nui-icons → SVG data → Button icon\nui-layout → AppShell → this window",cx))
                .child(h_flex().gap_2().child(Button::new("explore-components").primary().label("Explore components").on_click(cx.listener(|this,_,window,cx|this.navigate(Section::Components,window,cx))))
                    .child(Button::new("open-playground").outline().label("Open playground").on_click(cx.listener(|this,_,window,cx|this.navigate(Section::Playground,window,cx))))))
            .child(muted(format!("Library {}  ·  {} / {}  ·  native GPUI runtime",env!("CARGO_PKG_VERSION"),std::env::consts::OS,std::env::consts::ARCH),cx))
            .child(muted("macOS is the local verification target. Windows and Linux checks are configured in CI; runtime behavior on those platforms is unverified here.",cx)).into_any_element()
    }
    fn components(&self, cx: &mut Context<Self>) -> AnyElement {
        let story = &STORIES[self.story];
        v_flex().gap_5().child(title("Components","Live upstream controls through the local ui-components and ui-kit crates.",cx))
            .child(h_flex().gap_4().child(div().w_64().child(Select::new(&self.picker).id("story-picker").title_prefix("Component: ")))
                .child(Button::new("component-api").ghost().label(if story.name == "Navigation" { "Copy import" } else { "API reference ↗" }).on_click(move|_,_,cx| { if story.name == "Navigation" { cx.write_to_clipboard(ClipboardItem::new_string("use ui_kit::navigation::*;".into())); } else { cx.open_url(&format!("https://docs.rs/gpui-component/0.9.1/gpui_component/{}/",story.module)); } })))
            .child(card(cx).gap_4().child(div().text_2xl().font_semibold().child(story.name)).child(muted(story.description,cx))
                .child(div().id("live-preview").test_support().min_h_32().p_5().rounded_lg().border_1().border_color(cx.theme().border).child(self.story_preview(cx)))
                .child(muted(story.states,cx)))
            .child(usage_card(story.usage,cx))
            .child(self.property_controls(cx))
            .child(muted("The complete upstream catalog is re-exported. These stories demonstrate the supported integration; advanced systems remain available through their upstream modules.",cx)).into_any_element()
    }
    fn story_preview(&self, cx: &mut Context<Self>) -> AnyElement {
        match STORIES[self.story].name {
            "Navigation" => self.navigation_demo.clone().into_any_element(),
            "Button" => h_flex()
                .flex_wrap()
                .gap_3()
                .child(self.preview_button(cx))
                .child(Button::new("default-button").label("Default"))
                .child(
                    Button::new("disabled-button")
                        .label("Disabled")
                        .disabled(true),
                )
                .into_any_element(),
            "Input" => v_flex()
                .gap_2()
                .child("Button label")
                .child(
                    Input::new(&self.label)
                        .id("preview-label")
                        .disabled(self.disabled),
                )
                .child(muted(
                    format!("Current value: {}", self.label.read(cx).value()),
                    cx,
                ))
                .into_any_element(),
            "Checkbox" => Checkbox::new("demo-checkbox")
                .label("Remember preference")
                .checked(self.checked)
                .disabled(self.disabled)
                .on_click(cx.listener(|this, value, _, cx| {
                    this.checked = *value;
                    cx.notify();
                }))
                .into_any_element(),
            "Switch" => Switch::new("demo-switch")
                .label("Enable notifications")
                .checked(self.enabled)
                .disabled(self.disabled)
                .on_click(cx.listener(|this, value, _, cx| {
                    this.enabled = *value;
                    cx.notify();
                }))
                .into_any_element(),
            "Radio" => RadioGroup::horizontal("demo-radio")
                .child("Standard")
                .child("Express")
                .selected_index(Some(self.radio))
                .disabled(self.disabled)
                .on_click(cx.listener(|this, index, _, cx| {
                    this.radio = *index;
                    cx.notify();
                }))
                .into_any_element(),
            "Select" => Select::new(&self.fruit)
                .id("demo-select")
                .disabled(self.disabled)
                .into_any_element(),
            "Slider" => v_flex()
                .gap_4()
                .child(
                    Slider::new(&self.slider)
                        .accessibility_label("Preview progress")
                        .disabled(self.disabled),
                )
                .child(muted(
                    format!("Value: {:.0}", self.slider.read(cx).value().start()),
                    cx,
                ))
                .into_any_element(),
            "Tabs" => self.tabs(cx).into_any_element(),
            "Dialog" => Button::new("open-dialog")
                .outline()
                .label("Open preview…")
                .on_click(|_, window, cx| {
                    window.open_dialog(cx, |dialog, _, _| {
                        dialog.title("Component preview").child(
                            "This dialog uses the upstream Root, focus trap and Escape handling.",
                        )
                    });
                })
                .into_any_element(),
            "Popover" => Popover::new("demo-popover")
                .trigger(Button::new("popover-trigger").label("Options").outline())
                .content(|_, _, cx| {
                    card(cx)
                        .child("Contextual options")
                        .child("Click outside or press Escape to dismiss.")
                })
                .into_any_element(),
            "Tooltip" => Button::new("tooltip-trigger")
                .label("Hover for help")
                .outline()
                .tooltip("A short explanation from the upstream tooltip.")
                .into_any_element(),
            "Menu" => Button::new("menu-trigger")
                .label("Commands")
                .outline()
                .dropdown_menu(|menu, _, _| {
                    menu.menu("Next section", Box::new(NextSection))
                        .separator()
                        .menu("Toggle navigation", Box::new(ToggleNavigation))
                })
                .into_any_element(),
            "Progress" => Progress::new("demo-progress")
                .value(self.slider.read(cx).value().start())
                .accessibility_label("Completion")
                .into_any_element(),
            "Spinner" => h_flex()
                .gap_3()
                .child(Spinner::new())
                .child("Loading preview…")
                .into_any_element(),
            "Alert" => v_flex()
                .gap_3()
                .child(Alert::info("alert-info", "A new version is available."))
                .child(Alert::success("alert-success", "Changes saved."))
                .child(Alert::warning("alert-warning", "Storage is almost full."))
                .child(Alert::error(
                    "alert-error",
                    "The preview could not be saved.",
                ))
                .into_any_element(),
            "Tag" => h_flex()
                .gap_3()
                .child(Tag::new().child("Stable"))
                .child(Tag::primary().child("Primary"))
                .child(Tag::success().child("Success"))
                .child(Tag::warning().child("Warning"))
                .child(Tag::danger().child("Danger"))
                .into_any_element(),
            "Skeleton" => v_flex()
                .gap_3()
                .child(Skeleton::new().w_48().h_4())
                .child(Skeleton::new().w_64().h_4())
                .child(Skeleton::new().w_32().h_4())
                .into_any_element(),
            "Table" => sample_table().into_any_element(),
            "Breadcrumb" => Breadcrumb::new()
                .child(BreadcrumbItem::new("Library").on_click(
                    cx.listener(|this, _, window, cx| this.navigate(Section::Overview, window, cx)),
                ))
                .child(BreadcrumbItem::new("Components"))
                .into_any_element(),
            "Notification" => Button::new("notify-trigger")
                .outline()
                .label("Show notification")
                .on_click(|_, window, cx| window.push_notification("Preview updated", cx))
                .into_any_element(),
            "Scroll area" => v_flex()
                .id("demo-scroll")
                .h_48()
                .overflow_y_scroll()
                .gap_2()
                .children((1..=30).map(|n| div().p_2().child(format!("Scrollable item {n}"))))
                .into_any_element(),
            "Resizable panels" => div()
                .h_48()
                .child(SplitWorkspace::master_detail(
                    "story-split",
                    panel("Master", "Drag the divider to resize.", cx),
                    panel("Detail", "The panel primitive is owned by GPUI Kit.", cx),
                ))
                .into_any_element(),
            _ => div().child("Story unavailable").into_any_element(),
        }
    }
    fn property_controls(&self, cx: &mut Context<Self>) -> AnyElement {
        card(cx).gap_4().child(div().font_semibold().child("Preview properties"))
            .child(h_flex().gap_5().flex_wrap()
                .child(Checkbox::new("property-disabled").label("Disabled").checked(self.disabled).on_click(cx.listener(|this,value,_,cx|{this.disabled = *value;cx.notify();})))
                .child(Checkbox::new("property-loading").label("Button loading").checked(self.loading).on_click(cx.listener(|this,value,_,cx|{this.loading = *value;cx.notify();}))))
            .child(muted("Properties affect stories that support them. Hover, active and focus states are exercised with native input.",cx)).into_any_element()
    }
    fn tabs(&self, cx: &mut Context<Self>) -> TabBar {
        TabBar::new("workspace-tabs")
            .selected_index(self.tab)
            .child(Tab::new().label("Design"))
            .child(Tab::new().label("Code"))
            .child(Tab::new().label("Disabled").disabled(true))
            .on_click(cx.listener(|this, index, _, cx| {
                this.tab = *index;
                cx.notify();
            }))
    }
    fn themes(&self, cx: &mut Context<Self>) -> AnyElement {
        let service = ThemeService::global(cx);
        let tokens = service.tokens().clone();
        let definitions: Vec<_> = service.registry().themes().cloned().collect();
        let colors = serde_json::to_value(tokens.colors).unwrap_or_default();
        v_flex().gap_5().child(title("Themes","One authoritative configuration, projected onto upstream semantic tokens.",cx))
            .child(h_flex().gap_3().flex_wrap().children(definitions.into_iter().map(|definition|{
                let name=definition.name().to_owned();let label=format!("{} · {}",name,if definition.is_dark(){"Dark"}else{"Light"});
                let colors = &definition.tokens().colors;
                let surface = colors.surface.as_ref().and_then(|color|ui_kit::theme::parse_color(color).ok()).unwrap_or(cx.theme().popover);
                let foreground = colors.foreground.as_ref().and_then(|color|ui_kit::theme::parse_color(color).ok()).unwrap_or(cx.theme().foreground);
                let primary = colors.primary.as_ref().and_then(|color|ui_kit::theme::parse_color(color).ok()).unwrap_or(cx.theme().primary);
                card(cx).w_64().gap_3().child(label)
                    .child(h_flex().p_4().gap_3().rounded_md().bg(surface).text_color(foreground).child(div().text_2xl().child("Aa")).child(div().w_6().h_6().rounded_md().bg(primary)))
                    .child(Button::new(SharedString::from(format!("apply-theme-{name}"))).outline().label("Apply theme").on_click(cx.listener(move|this,_,_,cx|{this.status=match ThemeService::select_named(&name,cx){Ok(())=>format!("Applied {name}").into(),Err(e)=>e.to_string().into()};cx.notify();})))
            })))
            .child(card(cx).gap_3().child("Custom primary color").child(muted("Register a named copy of the current theme. Invalid values leave the active theme unchanged.",cx))
                .child(Input::new(&self.primary).id("custom-primary"))
                .child(Button::new("register-theme").outline().label("Register and apply").on_click(cx.listener(|this,_,_,cx|{
                    let color=this.primary.read(cx).value();let name=format!("custom-{}",this.custom_number+1);
                    let result=ThemeService::global(cx).registry().get(ThemeService::global(cx).selected()).cloned()
                        .and_then(|base|base.with_primary(&name,&color)).and_then(|theme|ThemeService::register(theme,cx)).and_then(|_|ThemeService::select_named(&name,cx));
                    match result {Ok(())=>{this.custom_number+=1;this.status=format!("Registered {name}").into();},Err(e)=>this.status=e.to_string().into()};cx.notify();
                }))))
            .child(card(cx).gap_3().child(div().font_semibold().child("Color tokens"))
                .child(h_flex().flex_wrap().gap_3().children(color_swatches(&tokens))))
            .child(card(cx).gap_3().child(div().font_semibold().child("Typography"))
                .children([("Extra small",tokens.typography.xs), ("Small",tokens.typography.sm),("Body",tokens.typography.md),("Heading",tokens.typography.lg),("Display",tokens.typography.xl)].into_iter().map(|(label,style)|div().text_size(style.size).font_weight(style.weight).child(format!("{label} · The quick brown fox")))))
            .child(card(cx).gap_3().child(div().font_semibold().child("Spacing scale"))
                .children([("xxs",tokens.spacing.xxs),("xs",tokens.spacing.xs),("sm",tokens.spacing.sm),("md",tokens.spacing.md),("lg",tokens.spacing.lg),("xl",tokens.spacing.xl),("xxl",tokens.spacing.xxl)].into_iter().map(|(label,spacing)|h_flex().gap_4().child(div().w_16().child(label)).child(div().h_3().w(spacing).bg(tokens.colors.primary)).child(format!("{:.0}px",f32::from(spacing))))))
            .child(card(cx).gap_3().child(div().font_semibold().child("Radius scale"))
                .child(h_flex().gap_4().children([tokens.radius.sm,tokens.radius.md,tokens.radius.lg,tokens.radius.xl].into_iter().map(|radius|div().w_16().h_12().rounded(radius).bg(tokens.colors.accent).border_1().border_color(tokens.colors.border).flex().items_center().justify_center().child(format!("{:.0}",f32::from(radius)))))))
            .child(usage_card("ThemeService::select(ThemeMode::System, Some(window), cx)?;",cx))
            .child(muted(format!("{} semantic color entries · system appearance uses native window notifications",colors.as_object().map_or(0,|v|v.len())),cx)).into_any_element()
    }
    fn layouts(&self, cx: &mut Context<Self>) -> AnyElement {
        const NAMES: [&str; 7] = [
            "Application shell",
            "Sidebar navigation",
            "Dashboard",
            "Master detail",
            "Resizable split",
            "Tabbed workspace",
            "Inspector",
        ];
        let body = match self.layout {
            0 => AppShell::new(panel(
                "Content",
                "Header, sidebar, toolbar, content and status slots.",
                cx,
            ))
            .header(header().child("Application header"))
            .sidebar(panel("Navigation", "Reusable sidebar region", cx))
            .toolbar(div().p_2().child("Toolbar"))
            .footer(div().p_2().text_xs().child("Status: ready"))
            .into_any_element(),
            1 => SplitWorkspace::master_detail(
                "nav-layout",
                sidebar_navigation().child(
                    navigation_group("WORKSPACE").children(
                        ["Projects", "Members", "Settings"]
                            .into_iter()
                            .enumerate()
                            .map(|(index, name)| {
                                Button::new(SharedString::from(format!("layout-nav-{name}")))
                                    .ghost()
                                    .selected(self.layout_item == index)
                                    .label(name)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.layout_item = index;
                                        cx.notify();
                                    }))
                            }),
                    ),
                ),
                panel(
                    ["Projects", "Members", "Settings"][self.layout_item],
                    "Controlled selection with grouped navigation.",
                    cx,
                ),
            )
            .into_any_element(),
            2 => div()
                .p_4()
                .child(
                    dashboard().children(
                        [("Projects", "12"), ("Members", "48"), ("Tasks", "216")]
                            .into_iter()
                            .map(|(label, value)| {
                                card(cx)
                                    .w_40()
                                    .child(muted(label, cx))
                                    .child(div().text_2xl().child(value))
                            }),
                    ),
                )
                .into_any_element(),
            3 => SplitWorkspace::master_detail(
                "master-layout",
                v_flex().p_3().gap_2().children(
                    ["Design system", "Desktop shell", "Native runtime"]
                        .into_iter()
                        .enumerate()
                        .map(|(index, name)| {
                            Button::new(SharedString::from(format!("master-{index}")))
                                .ghost()
                                .selected(self.layout_item == index)
                                .label(name)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.layout_item = index;
                                    cx.notify();
                                }))
                        }),
                ),
                panel(
                    ["Design system", "Desktop shell", "Native runtime"][self.layout_item],
                    "Selection updates the detail pane.",
                    cx,
                ),
            )
            .into_any_element(),
            4 => SplitWorkspace::master_detail(
                "resizable-layout",
                panel(
                    "Left panel",
                    "Drag the divider. Widths are constrained by upstream panel primitives.",
                    cx,
                ),
                panel(
                    "Right panel",
                    "Resize the window to exercise native layout.",
                    cx,
                ),
            )
            .into_any_element(),
            5 => TabbedWorkspace::new(
                self.tabs(cx),
                panel(
                    if self.tab == 0 { "Design" } else { "Code" },
                    "Tabs control application owned content.",
                    cx,
                ),
            )
            .into_any_element(),
            _ => SplitWorkspace::master_detail(
                "inspector-layout",
                panel("Navigator", "Choose an item", cx),
                panel("Workspace", "Three reusable resizable regions", cx),
            )
            .with_inspector(panel("Inspector", "Properties and context", cx))
            .into_any_element(),
        };
        v_flex().gap_5().child(title("Layouts","Application compositions over native GPUI and GPUI Kit panel primitives.",cx))
            .child(h_flex().flex_wrap().gap_2().children(NAMES.into_iter().enumerate().map(|(index,name)|Button::new(SharedString::from(format!("layout-{index}"))).small().outline().selected(self.layout==index).label(name).on_click(cx.listener(move|this,_,window,cx|{this.layout=index;window.focus(&this.focus,cx);cx.notify();})))))
            .child(div().id("layout-preview").test_support().w_full().h_80().border_1().border_color(cx.theme().border).rounded_lg().overflow_hidden().child(body))
            .child(muted("The main shell automatically hides its sidebar below 800 logical pixels. Ctrl+Tab changes sections even when navigation is collapsed. Split sizes remain in upstream keyed element state while mounted.",cx))
            .child(usage_card("AppShell::new(content)\n    .header(header)\n    .sidebar(navigation)\n    .footer(status)",cx)).into_any_element()
    }
    fn icon_gallery(&self, cx: &mut Context<Self>) -> AnyElement {
        let query = self.query.read(cx).value();
        let matches: Vec<_> = self.icons.search(&query).cloned().collect();
        v_flex().gap_5().child(title("Icons","A small semantic catalog, with original SVG assets and explicit namespaced registration.",cx))
            .child(div().child("Search by icon name").child(Input::new(&self.query).id("icon-search")))
            .child(h_flex().gap_2().children([(IconSize::Small,"14 px"),(IconSize::Medium,"18 px"),(IconSize::Large,"28 px")].into_iter().enumerate().map(|(index,(size,label))|Button::new(SharedString::from(format!("icon-size-{index}"))).outline().small().label(label).selected(self.icon_size==size).on_click(cx.listener(move|this,_,_,cx|{this.icon_size=size;cx.notify();}))))
                .children([(IconState::Default,"Foreground"),(IconState::Active,"Accent"),(IconState::Disabled,"Disabled")].into_iter().enumerate().map(|(index,(state,label))|Button::new(SharedString::from(format!("icon-color-{index}"))).ghost().small().label(label).selected(self.icon_state==state).on_click(cx.listener(move|this,_,_,cx|{this.icon_state=state;cx.notify();})))))
            .child(muted(format!("{} matching icons",matches.len()),cx))
            .child(h_flex().flex_wrap().gap_3().children(matches.iter().map(|id| {
                let icon=self.icons.render(id,self.icon_size,self.icon_state,cx).unwrap_or_else(|_|ui_kit::components::Icon::empty());
                let usage=self.icons.usage(id).unwrap_or_default();let key=id.as_str().to_owned();
                card(cx).w_48().gap_3().items_center().child(icon).child(div().text_sm().child(key.clone()))
                    .child(Button::new(SharedString::from(format!("copy-{key}"))).ghost().small().label("Copy Rust usage").on_click(cx.listener(move|this,_,_,cx|{cx.write_to_clipboard(ClipboardItem::new_string(usage.to_string()));this.status="Rust usage copied".into();cx.notify();})))
            })))
            .when(matches.is_empty(),|v|v.child(card(cx).child("No icons match this search. Clear the search or try a semantic name such as settings.")))
            .child(usage_card("let mut icons = IconRegistry::built_in()?;\nicons.register(IconId::new(\"product\", \"symbol\")?, svg_bytes)?;",cx)).into_any_element()
    }
    fn playground(&self, cx: &mut Context<Self>) -> AnyElement {
        let dense = ThemeService::global(cx).density() == Density::Compact;
        let code_text = format!(
            "Button::new(\"save\")\n    .{}()\n    .label({:?})\n    .disabled({})\n    .loading({})",
            ["primary", "outline", "ghost", "danger"][self.variant],
            self.label.read(cx).value().as_ref(),
            self.disabled,
            self.loading
        );
        v_flex()
            .gap_5()
            .child(title(
                "Playground",
                "Change properties and inspect the result immediately, without a rebuild.",
                cx,
            ))
            .child(
                card(cx)
                    .gap_5()
                    .child(
                        div()
                            .min_h_32()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(self.preview_button(cx)),
                    )
                    .child(
                        div()
                            .id("activation-count")
                            .role(gpui::Role::Status)
                            .test_support()
                            .aria_label(format!("Activations: {}", self.clicks))
                            .text_sm()
                            .child(format!("Activations: {}", self.clicks)),
                    ),
            )
            .child(
                card(cx)
                    .gap(content_gap(cx))
                    .child("Button label")
                    .child(Input::new(&self.label).id("playground-label"))
                    .child(muted("Variant", cx))
                    .child(
                        h_flex().gap_2().children(
                            ["Primary", "Outline", "Ghost", "Danger"]
                                .into_iter()
                                .enumerate()
                                .map(|(index, name)| {
                                    Button::new(SharedString::from(format!("variant-{index}")))
                                        .small()
                                        .outline()
                                        .label(name)
                                        .selected(self.variant == index)
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.variant = index;
                                            cx.notify();
                                        }))
                                }),
                        ),
                    )
                    .child(muted("Size", cx))
                    .child(
                        h_flex().gap_2().children(
                            [
                                (Size::Small, "Small"),
                                (Size::Medium, "Medium"),
                                (Size::Large, "Large"),
                            ]
                            .into_iter()
                            .enumerate()
                            .map(|(index, (size, label))| {
                                Button::new(SharedString::from(format!("size-{index}")))
                                    .small()
                                    .outline()
                                    .label(label)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.button_size = size;
                                        cx.notify();
                                    }))
                            }),
                        ),
                    )
                    .child(muted("Semantic icon", cx))
                    .child(
                        h_flex().gap_2().children(
                            [IconName::Check, IconName::Search, IconName::Settings]
                                .into_iter()
                                .map(|name| {
                                    Button::new(SharedString::from(format!(
                                        "choose-icon-{}",
                                        name.name()
                                    )))
                                    .small()
                                    .ghost()
                                    .label(name.name())
                                    .icon(self.icon(name, cx))
                                    .selected(self.selected_icon == name)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.selected_icon = name;
                                        cx.notify();
                                    }))
                                }),
                        ),
                    )
                    .child(
                        Checkbox::new("compact-density")
                            .label("Compact content density")
                            .checked(dense)
                            .on_click(cx.listener(|_, value, _, cx| {
                                ThemeService::set_density(
                                    if *value {
                                        Density::Compact
                                    } else {
                                        Density::Comfortable
                                    },
                                    cx,
                                );
                                cx.notify();
                            })),
                    ),
            )
            .child(self.property_controls(cx))
            .child(usage_card(&code_text, cx))
            .into_any_element()
    }
    fn documentation(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_5()
            .child(title(
                "Documentation",
                "Bundled repository documentation and public API references.",
                cx,
            ))
            .child(
                card(cx).child(
                    TextView::markdown(
                        "bundled-docs",
                        include_str!("../../../docs/getting-started/showcase.md"),
                    )
                    .selectable(true),
                ),
            )
            .child(
                Button::new("open-upstream-docs")
                    .outline()
                    .label("GPUI Kit documentation ↗")
                    .on_click(|_, _, cx| cx.open_url("https://gpui-kit.com")),
            )
            .child(
                Button::new("copy-install")
                    .ghost()
                    .label("Copy local dependency")
                    .on_click(|_, _, cx| {
                        cx.write_to_clipboard(ClipboardItem::new_string(
                            concat!(
                                "[dependencies]\nui-kit = { path = \"../gpui/crates/ui-kit\" }\n\n",
                                "[patch.crates-io]\ngpui-base = { path = \"../gpui/vendor/gpui-base\" }\n",
                                "accesskit_macos = { path = \"../gpui/vendor/accesskit_macos\" }\n"
                            ).into(),
                        ))
                    }),
            )
            .into_any_element()
    }
}
impl Render for Showcase {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let content = match self.section {
            Section::Overview => self.overview(cx),
            Section::Components => self.components(cx),
            Section::Themes => self.themes(cx),
            Section::Layouts => self.layouts(cx),
            Section::Icons => self.icon_gallery(cx),
            Section::Playground => self.playground(cx),
            Section::Documentation => self.documentation(cx),
        };
        div()
            .size_full()
            .track_focus(&self.focus)
            .on_action(cx.listener(Self::next))
            .on_action(cx.listener(Self::previous))
            .on_action(cx.listener(Self::toggle_navigation))
            .child(
                AppShell::new(content_container("page-scroll").child(content))
                    .collapsed(self.collapsed)
                    .sidebar(self.navigation(cx))
                    .header(
                        header()
                            .child(
                                h_flex()
                                    .gap_3()
                                    .child(
                                        Button::new("toggle-navigation")
                                            .ghost()
                                            .icon(self.icon(IconName::Menu, cx))
                                            .accessibility_label("Toggle navigation")
                                            .tooltip(if cfg!(target_os = "macos") {
                                                "Toggle navigation · ⌘B"
                                            } else {
                                                "Toggle navigation · Ctrl+B"
                                            })
                                            .on_click(cx.listener(|this, _, window, cx| {
                                                this.toggle_navigation(
                                                    &ToggleNavigation,
                                                    window,
                                                    cx,
                                                );
                                            })),
                                    )
                                    .child(div().font_semibold().child(self.section.title())),
                            )
                            .child(self.theme_controls(cx)),
                    )
                    .footer(
                        h_flex()
                            .px_4()
                            .py_2()
                            .justify_between()
                            .text_xs()
                            .text_color(cx.theme().muted_foreground)
                            .child(
                                div()
                                    .id("app-status")
                                    .role(gpui::Role::Status)
                                    .test_support()
                                    .aria_label(self.status.clone())
                                    .child(self.status.clone()),
                            )
                            .child(format!(
                                "{} · {}",
                                ThemeService::global(cx).selected(),
                                if matches!(ThemeService::global(cx).density(), Density::Compact) {
                                    "Compact"
                                } else {
                                    "Comfortable"
                                }
                            )),
                    ),
            )
    }
}
fn title(heading: &str, description: &str, cx: &App) -> gpui::Div {
    v_flex()
        .gap_2()
        .child(div().text_2xl().font_semibold().child(heading.to_owned()))
        .child(muted(description, cx))
}
fn muted(text: impl Into<SharedString>, cx: &App) -> gpui::Div {
    div()
        .text_sm()
        .text_color(cx.theme().muted_foreground)
        .child(text.into())
}
fn card(cx: &App) -> gpui::Div {
    v_flex()
        .p_5()
        .gap_2()
        .bg(cx.theme().popover)
        .rounded(cx.theme().radius_lg)
        .border_1()
        .border_color(cx.theme().border)
}
fn code(text: &str, cx: &App) -> gpui::Div {
    div()
        .p_4()
        .rounded(cx.theme().radius)
        .bg(cx.theme().muted)
        .font_family(cx.theme().mono_font_family.clone())
        .text_sm()
        .child(text.to_owned())
}
fn usage_card(text: &str, cx: &App) -> gpui::Div {
    let copied = text.to_owned();
    card(cx)
        .gap_3()
        .child(
            h_flex()
                .justify_between()
                .child(div().font_semibold().child("Rust usage"))
                .child(
                    Button::new("copy-usage")
                        .ghost()
                        .small()
                        .label("Copy")
                        .on_click(move |_, _, cx| {
                            cx.write_to_clipboard(ClipboardItem::new_string(copied.clone()))
                        }),
                ),
        )
        .child(code(text, cx))
}
fn panel(heading: &str, description: &str, cx: &App) -> AnyElement {
    v_flex()
        .id(SharedString::from(format!("panel-{heading}")))
        .test_support()
        .size_full()
        .p_4()
        .gap_3()
        .bg(cx.theme().popover)
        .child(div().font_semibold().child(heading.to_owned()))
        .child(muted(description, cx))
        .into_any_element()
}
fn sample_table() -> Table {
    Table::new()
        .accessibility_label("Crate responsibilities")
        .child(
            TableHeader::new().child(
                TableRow::new()
                    .child(TableHead::new().child("Crate"))
                    .child(TableHead::new().child("Responsibility")),
            ),
        )
        .child(
            TableBody::new().children(
                [
                    ("ui-theme", "Tokens and theme integration"),
                    ("ui-icons", "Validated semantic SVGs"),
                    ("ui-layout", "Application compositions"),
                ]
                .into_iter()
                .map(|(name, role)| {
                    TableRow::new()
                        .child(TableCell::new().child(name))
                        .child(TableCell::new().child(role))
                }),
            ),
        )
        .child(TableCaption::new().child("Reusable workspace foundations"))
}
fn color_swatches(tokens: &ui_kit::components::theme::SemanticThemeTokens) -> Vec<gpui::Div> {
    let colors = tokens.colors;
    [
        ("Background", colors.background),
        ("Surface", colors.surface),
        ("Surface text", colors.surface_foreground),
        ("Primary", colors.primary),
        ("Primary text", colors.primary_foreground),
        ("Secondary", colors.secondary),
        ("Secondary text", colors.secondary_foreground),
        ("Accent", colors.accent),
        ("Accent text", colors.accent_foreground),
        ("Foreground", colors.foreground),
        ("Muted", colors.muted),
        ("Muted text", colors.muted_foreground),
        ("Border", colors.border),
        ("Danger", colors.destructive),
        ("Danger text", colors.destructive_foreground),
        ("Input", colors.input),
        ("Focus ring", colors.ring),
        ("Selection", colors.selection),
    ]
    .into_iter()
    .map(|(label, color)| {
        v_flex()
            .gap_2()
            .w_24()
            .child(
                div()
                    .h_12()
                    .rounded_md()
                    .border_1()
                    .border_color(colors.border)
                    .bg(color),
            )
            .child(div().text_xs().child(label))
            .child(div().text_xs().child(color.to_hex()))
    })
    .collect()
}

#[cfg(test)]
mod tests;
