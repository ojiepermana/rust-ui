# Components

Import the unchanged upstream modules through `ui_kit::components` or depend on ui-components directly. The prelude includes Button and common component traits; it does not flatten every upstream type into an ambiguous namespace.

```rust
use ui_kit::components::{button::Button, input::{Input, InputState}};
use ui_kit::prelude::*;
```

The primary integration is a re-export. No standard control is copied, restyled into a parallel implementation or wrapped merely to rename it. The full catalog and real API docs are at [GPUI Kit](https://gpui-kit.com/component) and [gpui-component 0.9.1](https://docs.rs/gpui-component/0.9.1/gpui_component/).

## Demonstrated coverage

The Showcase catalog is defined in apps/showcase/src/catalog.rs. Each entry has description, actual preview, supported state notes, copyable Rust usage and a versioned API module link.

Button, Input, Checkbox, Switch, Radio, Select, Slider, Tabs, Dialog, Popover, Tooltip, Menu, Progress, Spinner, Alert, Tag, Skeleton, Table, Breadcrumb, Notification, Scroll area and Resizable panels have working stories. The property controls affect the selected story only where its API supports those properties. Loading is a Button state; error and success are Alert variants. Hover, pressed and focused states use real input rather than fabricated screenshots.

The upstream re-export also includes advanced DataTable, Tree, dock, editor, chart and other systems. Bespoke stories and interaction tests for those systems are not part of the initial 22 story set. Speech and syntax grammars need upstream opt in features not exposed by this facade in this release. A native webview, JavaScript extension host and business logic are outside scope.

Stateful controls must retain their Entity<State> outside render. Subscribe to InputEvent or SelectEvent using the owning Context and retain the returned Subscription. See the compiled Showcase and minimal consumer rather than inventing a second state framework.
