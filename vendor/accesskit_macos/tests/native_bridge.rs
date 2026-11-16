//! Main-thread native ABI regressions against the production adapter modules.
//! No window or external application's accessibility tree is opened by this test.
#![allow(dead_code)]

#[path = "../src/context.rs"]
mod context;
#[path = "../src/filters.rs"]
mod filters;
#[path = "../src/node.rs"]
mod node;
#[path = "../src/util.rs"]
mod util;
mod event {
    include!("../src/event.rs");

    pub(super) fn apply(
        context: &std::rc::Rc<crate::context::Context>,
        update: accesskit::TreeUpdate,
    ) -> Vec<(
        accesskit_consumer::NodeId,
        &'static objc2_app_kit::NSAccessibilityNotificationName,
    )> {
        let mut generator = EventGenerator::new(context.clone());
        context
            .tree
            .borrow_mut()
            .update_and_process_changes(update, &mut generator);
        generator
            .into_result()
            .events
            .into_iter()
            .filter_map(|event| match event {
                QueuedEvent::Generic {
                    node_id,
                    notification,
                } => Some((node_id, notification)),
                _ => None,
            })
            .collect()
    }
}

use accesskit::{Action, ActionRequest, Node, NodeId, Role, Tree as TreeData, TreeId, TreeUpdate};
use accesskit_consumer::Tree;
use context::{ActionHandlerNoMut, Context};
use node::PlatformNode;
use objc2::{
    msg_send, msg_send_id,
    rc::{Id, WeakId},
    runtime::Sel,
    sel,
};
use objc2_app_kit::{NSAccessibilityButtonRole, NSAccessibilityValueChangedNotification, NSView};
use objc2_foundation::{MainThreadMarker, NSObject, NSString};
use std::{cell::RefCell, rc::Rc};

const ROOT: NodeId = NodeId(1);
const BUTTON: NodeId = NodeId(2);
const BRANCH: NodeId = NodeId(3);
const PLAIN: NodeId = NodeId(4);
const ITEM: NodeId = NodeId(5);
const TAB: NodeId = NodeId(6);

#[derive(Default)]
struct Actions(RefCell<Vec<ActionRequest>>);
impl ActionHandlerNoMut for Actions {
    fn do_action(&self, request: ActionRequest) {
        self.0.borrow_mut().push(request);
    }
}
struct Fixture {
    context: Rc<Context>,
    actions: Rc<Actions>,
    _view: Id<NSView>,
}
fn button(selected: Option<bool>, expanded: Option<bool>) -> Node {
    let mut node = Node::new(Role::Button);
    node.set_label("Destination");
    node.add_action(Action::Click);
    if let Some(value) = selected {
        node.set_selected(value);
    }
    if let Some(value) = expanded {
        node.set_expanded(value);
    }
    node
}
impl Fixture {
    fn new() -> Self {
        let mtm =
            MainThreadMarker::new().expect("native bridge test must run on macOS main thread");
        let view = unsafe { NSView::new(mtm) };
        let mut root = Node::new(Role::Window);
        root.set_children(vec![BUTTON, BRANCH, PLAIN, ITEM, TAB]);
        let mut item = Node::new(Role::ListBoxOption);
        item.set_selected(true);
        item.add_action(Action::Click);
        let mut tab = Node::new(Role::Tab);
        tab.set_selected(true);
        tab.add_action(Action::Click);
        let tree = Tree::new(
            TreeUpdate {
                nodes: vec![
                    (ROOT, root),
                    (BUTTON, button(Some(true), None)),
                    (BRANCH, button(Some(false), Some(false))),
                    (PLAIN, button(None, None)),
                    (ITEM, item),
                    (TAB, tab),
                ],
                tree: Some(TreeData::new(ROOT)),
                tree_id: TreeId::ROOT,
                focus: BUTTON,
            },
            true,
        );
        let actions = Rc::new(Actions::default());
        let context = Context::new(WeakId::new(&view), tree, actions.clone(), mtm);
        Self {
            context,
            actions,
            _view: view,
        }
    }
    fn platform(&self, id: NodeId) -> Id<PlatformNode> {
        let id = self
            .context
            .tree
            .borrow()
            .state()
            .node_by_tree_local_id(id, TreeId::ROOT)
            .unwrap()
            .id();
        self.context.get_or_create_platform_node(id)
    }
    fn update(
        &self,
        id: NodeId,
        data: Node,
    ) -> Vec<(
        accesskit_consumer::NodeId,
        &'static objc2_app_kit::NSAccessibilityNotificationName,
    )> {
        event::apply(
            &self.context,
            TreeUpdate {
                nodes: vec![(id, data)],
                tree: None,
                tree_id: TreeId::ROOT,
                focus: BUTTON,
            },
        )
    }
}
fn allowed(node: &PlatformNode, selector: Sel) -> bool {
    unsafe { msg_send![node, isAccessibilitySelectorAllowed: selector] }
}
fn selected(node: &PlatformNode) -> bool {
    unsafe { msg_send![node, isAccessibilitySelected] }
}
fn expanded(node: &PlatformNode) -> bool {
    unsafe { msg_send![node, isAccessibilityExpanded] }
}

fn selected_button_preserves_push_button_role_without_toggle_value() {
    let fixture = Fixture::new();
    let node = fixture.platform(BUTTON);
    assert!(
        allowed(&node, sel!(isAccessibilitySelected)),
        "AXSelected must be readable on an explicitly selected Button"
    );
    assert!(selected(&node));
    let role: Id<NSString> = unsafe { msg_send_id![&*node, accessibilityRole] };
    assert_eq!(&*role, unsafe { NSAccessibilityButtonRole });
    let value: *mut NSObject = unsafe { msg_send![&*node, accessibilityValue] };
    assert!(
        value.is_null(),
        "selection must not create a toggle/check value"
    );
    fixture.update(BUTTON, button(Some(false), None));
    assert!(!selected(&node));
    let mut disabled = button(Some(true), None);
    disabled.set_disabled();
    fixture.update(BUTTON, disabled);
    assert!(
        selected(&node),
        "read-only selection remains meaningful when disabled"
    );
    let enabled: bool = unsafe { msg_send![&*node, isAccessibilityEnabled] };
    assert!(!enabled);
}
fn disclosure_exposes_real_expanded_selector_and_live_state() {
    let fixture = Fixture::new();
    let node = fixture.platform(BRANCH);
    let responds: bool =
        unsafe { msg_send![&*node, respondsToSelector: sel!(isAccessibilityExpanded)] };
    assert!(
        responds,
        "native bridge must implement the AXExpanded getter"
    );
    assert!(allowed(&node, sel!(isAccessibilityExpanded)));
    assert!(
        !allowed(&node, sel!(setAccessibilityExpanded:)),
        "expanded remains application controlled"
    );
    assert!(!expanded(&node));
    fixture.update(BRANCH, button(Some(false), Some(true)));
    assert!(expanded(&node));
    assert!(
        !selected(&node),
        "expanded branch is not the selected destination"
    );
    fixture.update(BRANCH, button(Some(false), Some(false)));
    assert!(!expanded(&node));
}
fn ordinary_buttons_do_not_gain_unsupplied_attributes_or_selection_actions() {
    let fixture = Fixture::new();
    let plain = fixture.platform(PLAIN);
    let chosen = fixture.platform(BUTTON);
    assert!(!allowed(&plain, sel!(isAccessibilitySelected)));
    assert!(!allowed(&plain, sel!(isAccessibilityExpanded)));
    assert!(!allowed(&chosen, sel!(setAccessibilitySelected:)));
    assert!(!allowed(&chosen, sel!(accessibilityPerformPick)));
    let _: () = unsafe { msg_send![&*chosen, setAccessibilitySelected: false] };
    assert!(
        fixture.actions.0.borrow().is_empty(),
        "read-only Button selection cannot synthesize activation"
    );
    assert!(allowed(&chosen, sel!(accessibilityPerformPress)));
    let pressed: bool = unsafe { msg_send![&*chosen, accessibilityPerformPress] };
    assert!(pressed);
    assert_eq!(fixture.actions.0.borrow().len(), 1);
    assert_eq!(fixture.actions.0.borrow()[0].action, Action::Click);
}
fn existing_item_selection_and_tab_value_are_preserved() {
    let fixture = Fixture::new();
    let item = fixture.platform(ITEM);
    let tab = fixture.platform(TAB);
    assert!(allowed(&item, sel!(isAccessibilitySelected)));
    assert!(selected(&item));
    assert!(allowed(&item, sel!(setAccessibilitySelected:)));
    assert!(
        !allowed(&tab, sel!(isAccessibilitySelected)),
        "Tabs retain their existing radio/value mapping"
    );
    let value: *mut NSObject = unsafe { msg_send![&*tab, accessibilityValue] };
    assert!(!value.is_null());
    let checked: bool = unsafe { msg_send![value, boolValue] };
    assert!(checked);
    let _: () = unsafe { msg_send![&*item, setAccessibilitySelected: false] };
    assert_eq!(
        fixture.actions.0.borrow().len(),
        1,
        "existing selectable-item actions remain available"
    );
    let mut disabled = Node::new(Role::ListBoxOption);
    disabled.set_selected(true);
    disabled.set_disabled();
    fixture.update(ITEM, disabled);
    assert!(
        !selected(&item),
        "existing disabled item selection behavior is unchanged"
    );
}
fn state_changes_notify_once_and_equivalent_updates_are_silent() {
    let fixture = Fixture::new();
    let notifications = fixture.update(BUTTON, button(Some(false), None));
    assert_eq!(
        notifications.len(),
        1,
        "Button selection needs one live native state-change notification"
    );
    assert_eq!(notifications[0].1, unsafe {
        NSAccessibilityValueChangedNotification
    });
    assert!(fixture.update(BUTTON, button(Some(false), None)).is_empty());
    let notifications = fixture.update(BRANCH, button(Some(false), Some(true)));
    assert_eq!(
        notifications.len(),
        1,
        "expanded transition needs one native state-change notification"
    );
    assert_eq!(notifications[0].1, unsafe {
        NSAccessibilityValueChangedNotification
    });
    assert!(
        fixture
            .update(BRANCH, button(Some(false), Some(true)))
            .is_empty()
    );
    let notifications = fixture.update(BRANCH, button(Some(true), Some(false)));
    assert_eq!(
        notifications.len(),
        1,
        "combined selected/expanded change is coalesced per node"
    );
}
fn removing_attributes_or_changing_role_reconciles_exposure() {
    let fixture = Fixture::new();
    let node = fixture.platform(BRANCH);
    fixture.update(BRANCH, button(None, None));
    assert!(!allowed(&node, sel!(isAccessibilitySelected)));
    assert!(!allowed(&node, sel!(isAccessibilityExpanded)));
    let mut heading = Node::new(Role::Heading);
    heading.set_selected(true);
    fixture.update(BRANCH, heading);
    assert!(
        !allowed(&node, sel!(isAccessibilitySelected)),
        "unrelated roles must not acquire Button selection behavior"
    );
}
fn main() {
    let cases: &[(&str, fn())] = &[
        (
            "selected Button retains role and no toggle",
            selected_button_preserves_push_button_role_without_toggle_value,
        ),
        (
            "expanded selector and live state",
            disclosure_exposes_real_expanded_selector_and_live_state,
        ),
        (
            "absent properties and read-only actions",
            ordinary_buttons_do_not_gain_unsupplied_attributes_or_selection_actions,
        ),
        (
            "legacy selection and tabs",
            existing_item_selection_and_tab_value_are_preserved,
        ),
        (
            "state-change notifications and no-op",
            state_changes_notify_once_and_equivalent_updates_are_silent,
        ),
        (
            "attribute removal and role changes",
            removing_attributes_or_changing_role_reconciles_exposure,
        ),
    ];
    let mut failed = 0;
    for (name, test) in cases {
        if std::panic::catch_unwind(*test).is_ok() {
            println!("PASS {name}");
        } else {
            eprintln!("FAIL {name}");
            failed += 1;
        }
    }
    println!(
        "Native bridge cases: {} passed; {failed} failed",
        cases.len() - failed
    );
    if failed != 0 {
        std::process::exit(1);
    }
}
