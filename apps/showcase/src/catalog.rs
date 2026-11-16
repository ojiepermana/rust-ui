//! The explicitly demonstrated catalog, distinct from the full upstream re-export.
/// One native story with a real implementation in the Showcase.
pub struct Story {
    /// Component label.
    pub name: &'static str,
    /// What the component does.
    pub description: &'static str,
    /// States represented by this story, without simulated hover or focus claims.
    pub states: &'static str,
    /// Compilable API form in the workspace context.
    pub usage: &'static str,
    /// Upstream API module.
    pub module: &'static str,
}
macro_rules! stories {
    ($(($name:literal,$description:literal,$states:literal,$usage:literal,$module:literal)),* $(,)?) => {
        /// Native stories implemented by the gallery.
        pub const STORIES: &[Story] = &[$(Story{name:$name,description:$description,states:$states,usage:$usage,module:$module}),*];
    };
}
stories![
    (
        "Button",
        "Commands with upstream keyboard focus and semantic variants.",
        "Default, primary, outline, ghost, danger, disabled, loading. Hover and focus are live interactions.",
        "Button::new(\"save\").primary().label(\"Save\")",
        "button"
    ),
    (
        "Input",
        "Retained text editing state. Type to update the playground label.",
        "Editable, disabled. Focus and selection use native input behavior.",
        "Input::new(&self.label).id(\"label\")",
        "input"
    ),
    (
        "Checkbox",
        "A controlled boolean owned by the application.",
        "Checked, unchecked, disabled.",
        "Checkbox::new(\"remember\").checked(true).label(\"Remember\")",
        "checkbox"
    ),
    (
        "Switch",
        "A controlled on/off preference.",
        "On, off, disabled.",
        "Switch::new(\"enabled\").checked(true).label(\"Enabled\")",
        "switch"
    ),
    (
        "Radio",
        "Mutually exclusive options using the native radio group.",
        "Selected, unselected, disabled.",
        "RadioGroup::horizontal(\"delivery\").child(\"Standard\").child(\"Express\")",
        "radio"
    ),
    (
        "Select",
        "Retained dropdown selection and keyboard navigation.",
        "Selection, placeholder, disabled.",
        "Select::new(&self.fruit).id(\"fruit\")",
        "select"
    ),
    (
        "Slider",
        "A retained numeric value on a constrained scale.",
        "Value, disabled, pointer and keyboard focus.",
        "Slider::new(&self.slider).accessibility_label(\"Progress\")",
        "slider"
    ),
    (
        "Tabs",
        "Controlled native tab selection.",
        "Selected, unselected, disabled tab.",
        "TabBar::new(\"tabs\").child(Tab::new().label(\"Design\"))",
        "tab"
    ),
    (
        "Dialog",
        "A modal mounted through the window's upstream Root.",
        "Open, dismissed. Escape dismissal and focus are upstream behavior.",
        "window.open_dialog(cx, |dialog, _, _| dialog.title(\"Preview\").child(\"Content\"));",
        "dialog"
    ),
    (
        "Popover",
        "Contextual content anchored to its native trigger.",
        "Open, closed. Outside click and Escape use upstream behavior.",
        "Popover::new(\"options\").trigger(Button::new(\"open\").label(\"Options\")).content(|_,_,_| div().child(\"Options\"))",
        "popover"
    ),
    (
        "Tooltip",
        "Hover over the trigger to inspect contextual help.",
        "Hidden, visible after hover.",
        "Button::new(\"help\").label(\"Help\").tooltip(\"Contextual help\")",
        "tooltip"
    ),
    (
        "Menu",
        "A dropdown built by the upstream menu system.",
        "Open, closed, separator. Commands use native action dispatch.",
        "Button::new(\"menu\").label(\"Commands\").dropdown_menu(|menu,_,_| menu.menu(\"Next section\", Box::new(NextSection)))",
        "menu"
    ),
    (
        "Progress",
        "Numeric progress following the Slider state.",
        "Determinate progress, accessible label.",
        "Progress::new(\"progress\").value(65.).accessibility_label(\"Progress\")",
        "progress"
    ),
    (
        "Spinner",
        "Upstream indeterminate activity indicator.",
        "Loading.",
        "Spinner::new()",
        "spinner"
    ),
    (
        "Alert",
        "Feedback with explicit text and semantic status colors.",
        "Info, success, warning, error.",
        "Alert::success(\"saved\", \"Changes saved\")",
        "alert"
    ),
    (
        "Tag",
        "Compact semantic labels.",
        "Secondary, primary, success, warning, danger.",
        "Tag::new().child(\"Stable\")",
        "tag"
    ),
    (
        "Skeleton",
        "A placeholder composed from upstream loading primitives.",
        "Loading.",
        "Skeleton::new().w_full().h_4()",
        "skeleton"
    ),
    (
        "Table",
        "A semantic table composed from upstream cells and rows.",
        "Header, body, caption. Advanced DataTable remains available through the facade.",
        "Table::new().child(TableHeader::new().child(TableRow::new().child(TableHead::new().child(\"Name\"))))",
        "table"
    ),
    (
        "Breadcrumb",
        "Location hierarchy using the upstream navigation primitive.",
        "Ancestors and current location.",
        "Breadcrumb::new().child(BreadcrumbItem::new(\"Library\")).child(BreadcrumbItem::new(\"Components\"))",
        "breadcrumb"
    ),
    (
        "Notification",
        "Feedback mounted by the window's upstream Root.",
        "Visible, dismissed.",
        "window.push_notification(\"Preview updated\", cx);",
        "notification"
    ),
    (
        "Scroll area",
        "A constrained native overflow region.",
        "Scrollable content, wheel input.",
        "div().id(\"scroll\").overflow_y_scroll().children(rows)",
        "scroll"
    ),
    (
        "Resizable panels",
        "Application composition delegates split dragging to GPUI Kit.",
        "Two panels with constrained minimum widths.",
        "SplitWorkspace::master_detail(\"workspace\", master, detail)",
        "resizable"
    ),
];
