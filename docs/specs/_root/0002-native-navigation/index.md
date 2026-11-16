# 0002. Navigasi native dengan GPUI Kit

**Date**: 2026-11-16
**Status**: Proposed
**Konfirmasi desain**: 2026-11-16, isi spec disetujui pengguna setelah pemeriksaan silang dan perbaikan. Implementasi belum dimulai.

## Summary

Tambahkan crate Rust `ui-navigation` untuk sidebar, dockbar, minidock, navbar, dan flyout native. Semua pola disusun dari GPUI Kit yang sudah dipin di workspace. Library Angular hanya menjadi contoh konsep dan interaksi, tanpa menjadi dependensi, runtime, atau target implementasi. Aplikasi memiliki state navigasi dan penyimpanannya.

## Requirements

**User stories**:

- Sebagai pembuat aplikasi Rust, saya ingin memilih lima pola navigasi dari satu kontrak item dan event melalui `ui_kit::navigation`.
- Sebagai pemilik aplikasi, saya ingin menentukan tujuan aktif, cabang terbuka, panel yang tampil, dan hasil aktivasi tanpa router atau service global dari library.
- Sebagai pengguna desktop, saya ingin seluruh tujuan dapat dioperasikan dengan keyboard, fokus tetap terlihat, dan penutupan panel mengembalikan fokus ke tempat yang masuk akal.

**Acceptance criteria**:

- **AC-1**: `ui-navigation` menjadi anggota workspace dengan dependensi lokal berversi, diekspor oleh `ui-kit`, dan dikonsumsi Showcase hanya melalui facade. Graph normal tetap memakai `gpui-kit =0.9.1` dan GPUI 0.3.12. Tidak ada dependensi Angular, JavaScript, DOM, webview, atau renderer baru.
- **AC-2**: Model menolak ID kosong, ID ganda termasuk pada node tersembunyi, label kosong pada node berlabel, dan branch tanpa anak. Error menyebut ID atau path yang bermasalah. Mengganti model atau konfigurasi yang tidak valid mempertahankan state terakhir yang valid.
- **AC-3**: Seleksi, expanded branches, panel terbuka, collapse sidebar, dan buka/tutup flyout berubah melalui setter yang dipanggil aplikasi. Aktivasi hanya mengirim event permintaan. Sinkronisasi nilai dari aplikasi tidak memancarkan kembali event interaksi. Satu aktivasi menghasilkan satu event semantik. Perubahan programatis tampil melalui observer yang disimpan aplikasi; permintaan popup yang ditolak kembali ke snapshot aplikasi pada frame berikutnya tanpa input lain.
- **AC-4**: Sidebar mendukung header, footer, group, branch bersarang, separator, spacer, selected item, disabled item, serta collapse menjadi rail. Anak branch tetap terjangkau tanpa hover melalui panel yang dibuka dengan keyboard ketika rail collapsed.
- **AC-5**: Minidock selalu berupa rail ikon, menerima item datar, separator, dan spacer, serta tidak dapat melebar lewat hover, fokus, atau event collapse. Model dengan group atau branch ditolak untuk pola ini. Header/footer ringkas dan item tanpa ikon tetap memiliki nama aksesibel.
- **AC-6**: Dockbar mendukung sisi kiri/kanan, rail tetap, serta mode panel `Inline` dan `Overlay`. Panel mengikuti ID branch pilihan aplikasi. Memilih branch lain mengganti permintaan panel tanpa memilih tujuan; aktivasi leaf tetap menjadi keputusan aplikasi.
- **AC-7**: Navbar menampilkan root item dan pemicu branch secara horizontal. Leaf memakai kontrol native untuk aktivasi, sedangkan branch memakai `Popover` terkendali. Hover tidak membuka panel dan tidak mengubah seleksi. Overflow bar dapat digulir horizontal, dan kontrol yang mendapat fokus dibawa ke area terlihat.
- **AC-8**: Flyout memiliki satu pemicu bernama yang membuka `Popover` berisi group dan navigasi bersarang. Escape, klik di luar, serta tombol tutup melewati kontrak event yang ditetapkan. Panel tidak harus meniru posisi, grid CSS, atau tab internal Angular.
- **AC-9**: ID elemen diturunkan dari instance, ID node, dan bagian kontrol. Penyisipan atau pengurutan ulang item tidak memindahkan seleksi atau fokus ke item lain. ID state yang sudah tidak terlihat tidak memilih pengganti secara otomatis dan tidak membuka cabang lain.
- **AC-10**: Ikon memakai registrasi `ui-icons`. Ikon hilang atau tidak tersedia mendapat fallback label ringkas tanpa panic. Badge angka menyembunyikan nol, menampilkan 1 sampai 99, dan menampilkan `99+` di atasnya; nama aksesibel tetap menyebut nama tujuan dan angka asli meski ada deskripsi badge kustom. Badge nol tidak diumumkan. Badge teks juga didukung. Total parent disediakan aplikasi.
- **AC-11**: Header dan footer berada di luar scroll vertikal item; spacer mendorong item sesudahnya ke dasar area vertikal ketika ruang tersedia. Navbar boleh menggulir bar horizontal termasuk prefix/suffix. Panel popup memiliki satu area scroll dan tidak melampaui viewport native yang tersedia.
- **AC-12**: Seluruh pola memakai warna semantik, control size upstream, dan skala tema. Rest, hover, pressed/open, selected, disabled, dan focus terlihat berbeda. Tema terang/gelap, kedua density, perubahan rem, dan reduced motion tidak memutus akses ke kontrol.
- **AC-13**: Penutupan branch/panel, penghapusan item terfokus, pergantian pola, penyembunyian region akibat resize, dan pelepasan view tidak meninggalkan fokus pada elemen yang hilang. Fokus slot dan popup termasuk cakupan instance. Fokus pindah ke pemicu yang masih ada, ancestor yang sesuai, atau fallback milik aplikasi. Pemicu popup yang ikut hilang tidak menjadi target pemulihan. Shortcut shell tetap bekerja setelah transisi tersebut.
- **AC-14**: Kontrol tujuan dan disclosure dapat dicapai dengan Tab/Shift+Tab dan diaktifkan dengan Enter/Space melalui Button upstream. Disabled tidak menghasilkan event. Escape menutup lapisan navigasi teratas yang relevan tanpa mengambil Escape dari dialog atau overlay lain yang lebih atas.
- **AC-15**: Contoh consumer yang hanya bergantung pada `ui-kit` dapat membangun model, menyimpan entity/subscription, menangani event, dan menyusun navigasi dengan konten aplikasi. API publik terdokumentasi, termasuk kontrak fokus dan setter di luar render.
- **AC-16**: API `AppShell`, `ShellConfig`, helper layout, `SplitWorkspace`, dan `TabbedWorkspace` yang ada tetap kompatibel. Tujuh demo Layouts yang ada tetap berfungsi. Tidak ada upgrade runtime atau perubahan vendor untuk pekerjaan ini.
- **AC-17**: Showcase memiliki galeri Navigation dengan kelima pola, data nyata yang sama sejauh kontrak polanya mengizinkan, dan konten tujuan yang berubah melalui handler aplikasi. Galeri membuktikan empty, disabled, nested, collapsed, ikon, badge, panel, serta pembaruan data saat berjalan.
- **AC-18**: Seluruh perintah validasi workspace dan pemeriksaan dependensi lulus setelah implementasi. Bukti pure test, interaksi headless GPUI, jendela native, dan pembaca layar dilaporkan terpisah per platform, tanpa memperluas klaim dari satu jenis bukti ke jenis lainnya.

## Decision

**Chosen option**: crate `ui-navigation` sebagai lapisan komposisi native di atas komponen GPUI Kit yang sudah ada.

Keputusan pengguna yang mengikat: target implementasi adalah repo GPUI; kelima pola termasuk cakupan; Angular hanya contoh; state, aksi, dan penyimpanan milik aplikasi; spec ditulis tanpa implementasi atau commit. Kebijakan versi Angular 22.x tidak menjadi kebijakan rilis crate Rust ini.

Rekomendasi teknis: satu model dan satu entity per instance, dengan event permintaan perubahan. Pemisahan ini menjaga ID, fokus, dan kontrak kontrol sama pada lima pola. Pilihan memperluas `ui-layout` langsung tidak dipilih karena navigasi kini memiliki model, event, dan lifecycle sendiri.

Panduan implementasi: Coding Guides dan Design Guides GPUI Kit pada commit `39ab849a93b5f15507b969164d49891db8d5b470`, serta aturan repo. Tidak ada skill Angular yang mengatur implementasi Rust.

### Kepemilikan dan dependensi

| Pemilik | Tanggung jawab |
| --- | --- |
| `ui-navigation` | Model navigasi tervalidasi, snapshot presentasi, event permintaan, komposisi lima pola, koordinasi fokus instance |
| `ui-components` / GPUI Kit | Button, Collapsible, Popover, Badge, tooltip, rendering, perilaku keyboard kontrol, lifecycle overlay |
| `ui-icons` | ID dan registrasi SVG, lookup serta rendering ikon |
| `ui-theme` | Konfigurasi dan snapshot token, density, proyeksi ke tema upstream |
| `ui-layout` | Shell, pembagian region, adaptasi lebar jendela, split/tabbed workspace |
| `ui-kit` | Ekspor modul `navigation` dan prelude terkurasi |
| Aplikasi / Showcase | Entity yang dipertahankan, subscription, kebijakan seleksi/open, konten tujuan, shortcut aplikasi, persistence |

Dependensi baru yang diizinkan: `ui-navigation -> ui-components, ui-icons, ui-theme, gpui-kit`; `ui-kit -> ui-navigation`. `ui-layout` dan `ui-navigation` tidak saling bergantung. Validator menambahkan crate baru ke himpunan library wajib dan daftar batas yang benar, sehingga metadata, sumber lokal, alternatif versi, dan siklus turut diperiksa untuknya. Showcase tetap memiliki satu dependensi library internal, yaitu `ui-kit`.

## Feature design

### Model dan identitas

Tidak ada database atau model penyimpanan baru. Tipe berikut adalah kontrak Rust yang akan dibangun, bukan API upstream yang dianggap sudah tersedia. Field struct privat, dengan constructor, builder, dan reader; enum publik yang perlu berkembang memakai `#[non_exhaustive]`.

| Tipe | Nilai dan aturan |
| --- | --- |
| `NavigationId` | `SharedString` tidak kosong setelah trim untuk pemeriksaan; nilai asli tidak dinormalisasi diam diam. Dipakai untuk identitas instance dan node dalam ruang masing masing |
| `NavigationModel` | Koleksi node terurut dan indeks ID yang dibangun saat validasi. Dibagikan melalui `Arc`; perubahan data membuat model valid yang baru |
| `NavigationNode` | Varian `Item`, `Branch`, `Group`, `Separator`, `Spacer`, semuanya memiliki ID stabil |
| `NavigationItem` | ID, label, optional subtitle/tooltip/`IconId`/badge, `visible` default true, `disabled` default false. Leaf yang bisa diaktifkan; tidak menyimpan URL, action closure, atau tipe domain aplikasi |
| `NavigationBranch` | ID, label, daftar node anak tidak kosong, optional ikon/badge/tooltip, visible dan disabled. Meminta expand atau panel, bukan tujuan yang dipilih |
| `NavigationGroup` | ID, label, node anak, visible. Heading tanpa aktivasi atau badge otomatis; group kosong setelah filtering tidak dirender |
| `NavigationBadge` | Angka `usize` atau teks tidak kosong, dengan optional deskripsi badge yang disuplai aplikasi; deskripsi tidak menggantikan nama tujuan atau angka asli |
| `NavigationKind` | `Sidebar`, `Dockbar(DockbarMode)`, `Minidock`, `Navbar`, `Flyout` |
| `DockbarMode` | `Inline` (panel mengisi ruang di samping rail) atau `Overlay` (Popover nonmodal). Dockbar tidak berarti sistem docking `DockArea` |
| `NavigationPresentation` | `selected: Option<NavigationId>`, `expanded: BTreeSet<NavigationId>`, `open_panel: Option<NavigationId>`, `collapsed: bool`, `flyout_open: bool`. Default tanpa pilihan/panel/expanded dan kedua boolean false |
| `NavigationMetrics` | Lebar sidebar, rail, panel, dan flyout dalam satuan rem. Default berturut turut 16, 3, 18, 24 rem. Semua finite, rail minimum 3 rem, dan lebar lain tidak kurang dari rail |

Node disusun sebagai tree dengan kepemilikan Rust, tanpa pointer parent yang bisa membentuk siklus. ID unik berlaku untuk semua node dalam satu model, termasuk separator, spacer, dan node tersembunyi. ID antar model boleh sama. ID instance harus berbeda untuk dua navigasi hidup dalam satu jendela.

Namespace elemen menggabungkan instance, ID node, dan nama bagian dengan encoding panjang segmen, sehingga ID yang mengandung separator teks tidak bisa bertabrakan. Label dan indeks tidak menjadi bagian identitas. Namespace yang sama dipakai untuk handle fokus, scroll, dan popup. Registry ID global baru tidak diperlukan.

Visibility parent menyembunyikan seluruh subtree. Disabled branch mempertahankan label tetapi menonaktifkan disclosure dan semua descendant. Separator di tepi atau berurutan dipadatkan pada proyeksi visual. Spacer hanya mengisi sisa ruang dalam list vertikal, tanpa menjadi Tab stop. Pengurutan data tidak mengubah ID.

Model minidock harus datar. Crate tidak meratakan group atau branch secara diam diam; aplikasi membentuk model flat dari domainnya dengan tetap mempertahankan ID leaf. Hal ini meniru tujuan pola minidock, tanpa mewajibkan salinan algoritme `toMinidockItems` Angular.

### State, event, dan lifecycle

`Entity<NavigationState>` dibuat aplikasi satu kali di constructor view. Entity ini menyimpan model, presentasi yang disuplai aplikasi, metrics, registry ikon, focus handles, dan scroll handles. Tidak ada registry global navigasi. Entity dan subscription tidak dibuat ulang di `render` atau callback konten popup.

`NavigationState` tidak menafsirkan domain atau menentukan tujuan aktif. Aplikasi boleh menjadikannya state UI kanonis, atau menyinkronkan snapshot dari state domainnya. Dalam kedua cara, handler interaksi crate hanya memancarkan permintaan; setter aplikasi adalah jalur perubahan nilai yang ditampilkan. State mekanis di dalam komponen upstream tetap dimiliki upstream.

Karena `Navigation` adalah `RenderOnce` yang membaca snapshot entity, aplikasi menyimpan dua subscription terpisah: `cx.subscribe` untuk event permintaan dan `cx.observe` untuk menginvalidasi view penampil lewat `cx.notify()`. Observer dibuat di constructor dan disimpan sepanjang lifetime view. Notifikasi state saja tidak dianggap otomatis merender ulang parent. Perubahan model, presentasi, metrics, dan registry ikon harus tampak tanpa input lain. Transisi tema tetap mengikuti refresh tema workspace.

| Event `NavigationEvent` | Isi dan makna |
| --- | --- |
| `Activate` | ID leaf yang diminta. Tidak mengubah selected atau menutup panel otomatis |
| `ExpandedChange` | ID branch dan boolean yang diminta, untuk branch dalam list expanded |
| `PanelChange` | `Option<NavigationId>` yang diminta, untuk branch pada rail atau navbar. `None` meminta tutup |
| `CollapsedChange` | Boolean yang diminta dari toggle sidebar |
| `FlyoutOpenChange` | Boolean yang diminta dari pemicu, Escape, outside click, atau tombol tutup flyout |

Tidak ada event untuk pembacaan, render, perubahan tema, atau sinkronisasi nilai eksternal. Event membawa ID, bukan indeks. Pointer dan keyboard mengarah ke satu jalur aktivasi. Subscription aplikasi dapat menerima atau mengabaikan permintaan. Aplikasi yang menerima `Activate` memperbarui tujuan, pilihan, serta penutupan panel dalam satu pembaruan yang koheren, lalu memberi notifikasi.

Permintaan `PanelChange` dari Popover dikumpulkan dalam batch input menggunakan callback deferred milik window. Setiap kandidat mencatat ID panel asal dan revisi proyeksi. Dalam satu dispatch, permintaan membuka B menang atas outside dismiss A, sehingga publik hanya menerima `PanelChange(Some(B))`, bukan urutan tutup A lalu buka B. Callback penutup dari ID/revisi lama tidak boleh menutup panel yang lebih baru. Bila hanya ada dismiss, kirim `PanelChange(None)` sekali. Pemicu popup memakai jalur Popover yang sama pada mouse down atau konfirmasi keyboard, tanpa handler Button kedua yang mengulang permintaan.

Setelah batch permintaan dikirim, state menjadwalkan satu refresh rekonsiliasi melalui observer pada frame berikutnya, termasuk bila aplikasi mengabaikan permintaan dan tidak memanggil setter. Refresh ini tidak mengubah presentasi dan tidak mengirim event baru; tujuannya menyinkronkan kembali state mekanis upstream ke snapshot aplikasi serta memastikan fokus berakhir pada surface yang benar. Ini pekerjaan terbatas akibat input, bukan notify tanpa syarat dari render. Capture weak entity membuat batch tertunda menjadi no-op bila instance telah dilepas.

State yang menunjuk ID hilang/tersembunyi dipertahankan sebagai input aplikasi, tetapi tidak memiliki efek visual. Tidak ada pilihan pertama otomatis. `expanded` hanya berpengaruh pada branch valid. `open_panel` hanya dapat membuka branch root yang terlihat dan enabled pada dockbar/navbar, atau branch collapsed yang tampil di rail sidebar. Pada flyout, branch menggunakan `expanded`; `open_panel` tidak dipakai. Pada minidock, seluruh nilai open/collapse diabaikan untuk presentasi dan tidak ditulis ulang.

Root interaktif adalah Item atau Branch yang dicapai dari root model melewati Group, tetapi belum melewati Branch lain. Dockbar, navbar, dan rail sidebar collapsed memakai definisi yang sama dalam urutan sumber; Group hanya memberi pengelompokan, bukan tombol rail. Panel root menampilkan anaknya tanpa mensyaratkan root tersebut masuk `expanded`; branch descendant tetap mengikuti `expanded`. Klik ulang pemicu panel yang sama meminta `PanelChange(None)`, sedangkan pemicu branch lain meminta ID baru. Setiap permintaan toggle dihitung dari snapshot aplikasi, bukan counter klik internal.

Pergantian kind atau model mempertahankan nilai presentasi milik aplikasi dan menghitung ulang proyeksi yang berlaku. Misalnya, kembali dari minidock ke sidebar dapat memakai expanded yang masih disimpan aplikasi. Dataset atau kombinasi kind/model yang tidak valid ditolak secara atomik, tanpa setter parsial dan tanpa event palsu.

Konfigurasi ekuivalen adalah no-op. Kind dan presentation dibandingkan berdasarkan nilai; model berdasarkan seluruh field node dalam urutan tree, dengan `Arc` identik sebagai jalur cepat. Model baru dengan isi identik juga ekuivalen. Configure/set_model yang ekuivalen tidak mengganti handle, menaikkan revisi, memindahkan fokus, memancarkan event, atau memberi notifikasi. Metrics yang finite dibandingkan menurut nilai empat lebarnya. Ini tidak membatalkan refresh satu kali akibat permintaan popup yang ditolak, karena sumber refresh tersebut adalah input, bukan setter identik.

### API publik yang direncanakan

| Surface | Kontrak |
| --- | --- |
| `NavigationId::new(value)` | `Result<NavigationId, NavigationError>`; reader `as_str()` |
| Constructor node | `NavigationItem::new(id, label)`, `NavigationBranch::new(id, label, children)`, `NavigationGroup::new(id, label, children)`; builder untuk optional field dan reader tanpa membuka field |
| Node struktural dan badge | `NavigationNode::separator(id)`, `NavigationNode::spacer(id)`, `NavigationBadge::count(value)`, dan `NavigationBadge::text(value)`; optional deskripsi badge lewat `with_description` |
| `NavigationModel::new(nodes)` | `Result<NavigationModel, NavigationError>`; validasi menyeluruh, termasuk node yang hidden; reader `nodes()` dan lookup `item(id)` |
| `NavigationMetrics::default()` dan `with_*_width(rem)` | Nilai default valid; builder fallible untuk tiap lebar, reader empat lebar, dan `validate()` |
| `NavigationPresentation::default()` | Builder `with_selected`, `with_expanded`, `with_open_panel`, `collapsed`, `flyout_open`; reader `selected`, `expanded`, `open_panel`, `is_collapsed`, `is_flyout_open` |
| `NavigationState::new(instance_id, fallback_focus, window, cx)` | Entity awal valid dengan model kosong, sidebar, presentasi default, metrics default, registry ikon belum diisi, serta fallback aplikasi yang wajib disuplai. Dibuat di `cx.new` oleh aplikasi |
| `NavigationState::configure(kind, model, presentation, window, cx)` | `Result<(), NavigationError>`; validasi kind/model sebelum commit, rekonsiliasi fokus sebelum elemen lama hilang, satu notifikasi bila nilai berubah; konfigurasi ekuivalen no-op |
| `set_model(model, window, cx)`, `set_kind(kind, window, cx)` | Setter fallible dengan aturan atomik yang sama |
| `set_presentation(presentation, window, cx)` | Sinkronisasi dari aplikasi tanpa event permintaan; rekonsiliasi fokus dan notify hanya jika nilai berubah |
| `set_metrics(metrics, cx)` | Setter fallible; tidak ada dimensi invalid yang mencapai layout |
| `set_icons(Arc<IconRegistry>, cx)` | Registry hanya dibaca; penggantian registry memicu render baru tanpa mengubah selected/open |
| `set_fallback_focus(FocusHandle)` | Mengganti fallback dengan target milik aplikasi yang masih mounted |
| `restore_focus(window, cx)` | Sebelum aplikasi melepas atau menyembunyikan region visual, termasuk bila entity tetap hidup, kembalikan fokus ke fallback hanya bila fokus masih di dalam instance |
| Reader state | `kind()`, `model()`, `presentation()`, `metrics()`, `focus_handle()`; tidak membuka mutable field |
| `Navigation::new(&Entity<NavigationState>)` | Elemen `RenderOnce`/`IntoElement` dan `Styled`; builder `header`, `footer`, `empty`, `accessibility_label` (default `Navigation`), `trigger_label` (default `Menu`), dan `side(ui_components::Side)` (default Left) |
| `ui_navigation::init(cx)` | Registrasi key binding instance satu kali setelah `gpui_kit::init` dan `ThemeService::init`; diekspor juga sebagai `ui_kit::navigation::init` |
| Event subscription | `NavigationState: EventEmitter<NavigationEvent>`; subscription dikembalikan GPUI dan disimpan aplikasi selama view hidup |
| Invalidation subscription | Aplikasi menyimpan `cx.observe` atas navigation entity dan memberi `cx.notify()` pada view penampil; terpisah dari subscription event |
| Ekspor facade | `ui_kit::navigation` mengekspor modul crate; prelude menambahkan tipe utama secara eksplisit, tanpa mengubah path komponen upstream |

`window` dan `cx` pada API state masing masing adalah `&mut Window` dan `&mut Context<NavigationState>`; model masuk sebagai `Arc<NavigationModel>`, presentation dan metrics sebagai nilai. `NavigationError` membedakan `InvalidId`, `DuplicateId`, `EmptyLabel`, `EmptyBranch`, `UnsupportedNode`, dan `InvalidMetrics`, dengan reader diagnostik. Pemanggil menampilkan error melalui UI aplikasi yang sudah ada; crate tidak membuka dialog, melakukan I/O, atau panic untuk data invalid.

`Navigation` membaca snapshot singkat sebelum membangun elemen. Listener menangkap weak entity agar pelepasan view tidak membentuk siklus ownership. Callback popup tidak mengakses entity yang sedang diborrow ulang secara tidak aman. Tidak ada entity domain atau subscription baru dari renderer.

### Komposisi upstream dan perilaku lima pola

| Pola | Komposisi dan kontrak |
| --- | --- |
| Sidebar expanded | Region vertikal, Button untuk leaf/disclosure, `Collapsible::open` untuk isi branch, group heading, scroll items dengan header/footer terpisah. Toggle collapse memakai `SidebarToggleButton` upstream dengan input dan callback terkendali |
| Sidebar collapsed | Rail selebar metrics, nama ringkas/ikon dan tooltip, leaf tetap Button. Branch membuka `Popover` terkendali dari `open_panel`; tidak memerlukan hover preview |
| Dockbar Inline | Rail dan panel root branch berdampingan; panel berisi heading branch, tombol tutup, serta list yang sama. Tidak ada pointer leave yang mengubah panel atau route |
| Dockbar Overlay | Rail yang sama dengan panel `Popover` nonmodal. Kiri membuka ke kanan melalui anchor `LeftCenter`, kanan melalui `RightCenter`; penempatan/clamping tetap milik upstream |
| Minidock | Rail leaf tanpa toggle, disclosure, atau popup; spacer memberi pemisahan bagian atas/bawah; header/footer harus berupa konten yang muat pada rail |
| Navbar | Bar horizontal Button; root branch memicu `Popover` dengan anchor `TopLeft`, leaf memancarkan Activate. Bukan `menubar` tiruan yang hanya mengimplementasikan sebagian arrow keys |
| Flyout | Button bernama, default label `Menu`, dengan `Popover` dan tombol tutup. Isi berupa group/list yang membungkus sesuai lebar, dengan branch terkendali. Grid dan tab root Angular bukan kontrak wajib |

`Button::selected` dan status `open` diperlakukan terpisah. Selected leaf berasal dari ID aplikasi; branch dapat memperoleh penekanan ancestor ketika mengandung selected leaf, tanpa dianggap tujuan yang aktif. Heading group tidak menjadi tombol. Semua leaf dan disclosure memakai Button native, bukan div yang diberi handler klik.

`SidebarMenuItem::default_open` upstream hanya menginisialisasi keyed state submenu. Karena tidak memiliki setter controlled untuk state submenu saat ini, implementasi tidak memakainya untuk branch yang wajib dikendalikan aplikasi. Button dan `Collapsible::open` adalah komposisi yang dipilih; tidak menyalin implementasi SidebarMenuItem.

Semua popup memakai `Popover::open` bersama `on_open_change`, bukan `default_open`. Mekanisme focus/dismiss internal upstream boleh menjalankan transisi sebelum snapshot aplikasi tersinkron, tetapi nilai aplikasi kembali menjadi acuan pada render berikutnya. Perilaku ini tidak boleh menghasilkan dua event atau menjadikan state internal popup sumber seleksi domain. Sinkronisasi `open` eksternal tidak mengirim permintaan balik.

Native `Root` tetap satu per jendela. Focus restoration dan Escape popup memakai upstream. Crate tidak menambah focus trap untuk panel nonmodal atau menulis ulang collision geometry. Anchor GPUI Kit 0.9.1 melakukan clamping; spec tidak menjanjikan flipping otomatis yang tidak disediakan seam ini.

Pada sidebar/minidock, header dan footer berada di atas/bawah list. Pada dockbar keduanya milik rail, sedangkan judul panel dan tombol tutup dibentuk dari branch yang dibuka. Pada navbar keduanya menjadi prefix/suffix bar. Pada flyout keduanya berada di dalam popup di luar scroll list; pemicu Menu tetap berada pada alur layout. Konten slot dibuat aplikasi pada setiap komposisi, dengan state kontrol slot tetap dipertahankan aplikasi.

Metrics adalah lebar preferred; style root eksplisit lewat `Styled` dapat mengikuti parent, misalnya `w_full()` di slot AppShell. Ukuran popup dibatasi ruang viewport setelah inset `spacing.sm` pada kedua sisi, dan isi memakai satu scroll owner dengan `min_h_0`. Root sidebar/rail serta setiap panel memiliki scroll handle terpisah yang dipertahankan state berdasarkan ID region; pergantian panel tidak membuat list milik panel lain mewarisi posisi scrollnya.

Navbar memiliki scroll horizontal untuk keseluruhan bar, termasuk prefix/suffix, sehingga slot aplikasi tidak dapat membuat tujuan terakhir hilang permanen. Tingginya tidak dipotong untuk meniru bar web. State mempertahankan handle scroll bar dan membawa bounds kontrol yang baru mendapat fokus ke viewport horizontalnya, baik lewat Tab maupun fokus programatis. Penghitungan memakai urutan dan bounds saat ini, bukan indeks yang disimpan lintas reorder. Tidak ada menu More atau penyembunyian tujuan otomatis.

### Fokus dan keyboard

State mempertahankan focus handles keyed berdasarkan ID instance dan node. Button menggunakan `track_focus`, sehingga handle yang sama menggerakkan fokus keyboard dan ring. Penyisipan/reorder node mempertahankan handle node yang sama. Handle milik node yang hilang dibuang setelah fokus dipindahkan.

Cakupan fokus instance meliputi region root dengan seluruh kontrol header/footer yang diproyeksikan, node dalam list, close button, serta setiap region popup yang terkait instance walaupun dipasang pada overlay Root. Scope root dan panel memakai handle yang dipertahankan, sehingga pemeriksaan kepemilikan tidak hanya mencari ID node leaf. Modal independen atau navigasi instance lain bukan bagian cakupan ini.

Setter yang akan menghilangkan kontrol memeriksa fokus sebelum commit. Prioritas target: pemicu disclosure/panel yang masih terlihat dan enabled, ancestor terdekat yang masih bisa menerima fokus, lalu fallback aplikasi. Jangan memindahkan fokus bila perubahan tidak menghilangkan atau menonaktifkan kontrol terfokus. Upstream mempertahankan dismissal popup; aplikasi memanggil `restore_focus(window, cx)` sebelum membuang seluruh entity jika fokus masih berada di dalam instance.

`restore_focus` hanya memindahkan fokus dari instance sendiri ke fallback yang sudah dikonfigurasi; tidak mengambil fokus dari view atau modal lain. Bila setter menutup kontrol yang berada di bawah modal lain, jangan merebut fokus modal. Fallback yang masih mounted adalah prasyarat consumer, dan contoh consumer harus memperlihatkannya.

Pemilik region memanggil helper ini sebelum region visual dilepas, meski navigation entity tetap hidup. Pada AppShell, aplikasi mengamati perubahan bounds window dengan subscription yang dipertahankan, menghitung transisi dari sidebar terlihat ke tersembunyi memakai ShellConfig yang sama, dan memulihkan fokus sebelum frame yang menghilangkan sidebar. Hal yang sama berlaku saat aplikasi mengganti konten slot yang sedang terfokus. Tidak ada perintah focus tanpa syarat dari render.

Bila pemicu popup ikut hilang, aplikasi/setter memindahkan fokus terlebih dahulu ke ancestor atau fallback yang masih mounted, baru melepas subtree. Pemulihan upstream tidak boleh mengembalikan fokus ke pemicu lama setelah itu. Integrasi wajib membuktikan urutan ini pada penghapusan item, pergantian kind, dan resize shell, termasuk fokus yang berasal dari header/footer atau popup.

Patokan aksesibilitas berikut diuji terpisah dari style Button. `Button::selected` upstream adalah style, bukan bukti metadata selected atau toggle.

| Kontrol | Hasil native yang diharapkan |
| --- | --- |
| Leaf | Role Button, nama tujuan lengkap dengan badge yang terlihat, selected true hanya untuk leaf pilihan aplikasi, disabled sesuai model. Tidak diberi state toggle hanya karena selected |
| Disclosure branch / pemicu panel | Role Button, nama branch, expanded sesuai panel/disclosure efektif, selected false. Penekanan ancestor adalah visual, bukan pilihan tujuan |
| Pemicu flyout | Role Button, nama trigger, expanded sesuai flyout_open, selected false |
| Toggle collapse | Role Button dengan nama Expand navigation atau Collapse navigation sesuai state, bukan nama kosong berupa ikon |
| Group / separator / spacer | Tidak dapat diaktifkan atau menjadi Tab stop; group berlabel sebagai heading, tanpa selected/expanded palsu |

Metadata selected/expanded disetel secara eksplisit melalui seam aksesibilitas native yang didukung komponen, bukan disimpulkan dari warna atau `.toggled`. Bila bridge platform tidak mengekspos suatu status, catat properti yang diharapkan, properti yang benar benar tersedia, platform, dan dampaknya. Jangan mengklaim AC aksesibilitas itu selesai, mengganti role menjadi tree/menu yang tidak diimplementasikan, atau meng-upgrade/patch upstream diam diam. Pembacaan teknologi bantu tetap mempunyai bukti tersendiri.

List tujuan memakai urutan Tab visual, dengan Enter/Space untuk aktivasi upstream. Tidak ada janji treeview/menubar atau roving arrow navigation untuk list Button. Escape pada panel Inline ditangani Action `CloseNavigationPanel` dalam key context instance; handler meminta `PanelChange(None)`. Binding didaftarkan satu kali lewat `ui_navigation::init(cx)` setelah init GPUI Kit. Tidak ada binding global Ctrl+Tab dari crate; aplikasi menjaga shortcut shellnya sendiri.

### Tampilan, ikon, badge, dan nilai

| Nilai yang ditampilkan | Sumber |
| --- | --- |
| Urutan, label, subtitle, tooltip, visible, disabled | Model valid dari aplikasi |
| Selected leaf dan branch ancestor | `presentation.selected` serta ancestry pada model yang terlihat |
| Isi expanded branch | `presentation.expanded`; tidak auto open karena selected |
| Panel dockbar/navbar/rail | `presentation.open_panel`; ID yang tidak berlaku tidak menampilkan panel |
| Flyout terbuka | `presentation.flyout_open` |
| Warna kontrol, focus ring, popup, selected | Semantic theme upstream yang diproyeksikan `ui-theme` |
| Padding, gap, radius, density lokal | Snapshot lengkap `ThemeService::global(cx).tokens()` dan density; control size upstream small untuk Compact, medium untuk Comfortable |
| Lebar navigasi | `NavigationMetrics` dalam rem; sidebar 16, rail 3, panel 18, flyout 24 secara default |
| Viewport popup dan tinggi scroll | Bounds jendela native pada frame tersebut, dikurangi inset spacing tema; tidak ada cache dimensi lintas rem/resize |
| Ikon node | Lookup `IconId` pada registry aplikasi; fallback dua `char` Unicode pertama dari label bila lookup tidak tersedia |
| Badge | Angka/teks yang diberikan aplikasi. Tidak ada fetch, Observable, polling, atau akumulasi parent internal |
| Label aksesibel | Nama node, angka asli atau teks badge yang terlihat, lalu optional deskripsi badge aplikasi; deskripsi tidak menggantikan nama atau angka |
| Konten tujuan | View aplikasi setelah menangani Activate; crate tidak menerima route path |

Badge menggunakan `Badge::count().max(99)` untuk count dan `Tag` netral upstream untuk teks. Nama lengkap ditempatkan pada Button pemiliknya sehingga informasi tidak bergantung pada pembacaan glyph badge. Style badge upstream tetap dimiliki upstream; jangan menyalin pill Angular atau mengklaim jaminan kontras untuk seluruh custom theme tanpa pengukuran.

Komposisi count adalah `nama, angka deskripsi`, dengan deskripsi opsional dan tanpa angka yang diulang di dalamnya. Misalnya, label Inbox, count 120, dan deskripsi unread messages menghasilkan `Inbox, 120 unread messages`. Tanpa deskripsi hasilnya `Inbox, 120`. Count nol menghilangkan seluruh bagian badge dari tampilan serta nama aksesibel. Badge teks memakai `nama, teks`, lalu deskripsi bila diberikan. Aplikasi memasok teks yang sudah dilokalkan dan bentuk jamak yang sesuai; library tidak menebak domain atau pluralisasi. Tidak ada input label lengkap badge yang dapat membuang nama tujuan atau angka asli.

Tidak ada warna Angular, CSS token, atau aturan sumbu `theme-space`/`theme-radius` yang dibawa. Ikon registry saat ini memiliki ukuran logical pixel tetap; proyeksi ukuran ikon navigasi mengikuti ukuran kontrol/rem, memakai aset SVG yang sama dan mendokumentasikan batas default `IconSize` yang tetap. Skala tema yang tidak diproyeksikan oleh storage legacy tetap dibaca dari snapshot `ui-theme`.

### Integrasi dengan layout yang sudah ada

`AppShell::sidebar` tetap region dengan lebar `ShellConfig`; collapse shell menyembunyikan region, berbeda dari collapse Navigation menjadi rail. Keduanya tidak digabung menjadi satu boolean. Aplikasi yang menyembunyikan seluruh sidebar wajib menyediakan jalur navigasi ringkas atau shortcut alternatif, seperti Ctrl+Tab pada Showcase.

Jangan memasukkan rail 3 rem atau dockbar Inline yang memerlukan rail plus panel ke slot sidebar berlebar tetap lalu menganggap shell otomatis menyesuaikan. Consumer contoh menyusun rail/panel dan konten dalam satu `h_flex().items_stretch()` di area content AppShell. Sidebar expanded dapat memakai slot sidebar yang ada. Pola Navbar/Flyout dapat ditempatkan di header/toolbar. Tidak ada pelonggaran batas ShellConfig untuk menyelundupkan lebar rail.

Galeri Navigation ditambahkan pada bagian Components sebagai satu story bernama Navigation dengan selector lima pola; tujuh destinasi utama Showcase dan tujuh komposisi Layouts tetap ada. Perubahan kind demo adalah update state di luar render. Demo memakai model flat terpisah untuk minidock, dengan ID leaf yang sama dengan model tree.

### Batas keamanan dan kegagalan

Crate hanya merender data dan memancarkan intent. Tidak membuka URL, memeriksa izin, menjalankan action domain, menyimpan credential, atau membaca storage. `visible` dan `disabled` adalah presentasi; aplikasi tetap menegakkan izin operasinya sendiri.

Model kosong menampilkan slot `empty`, atau teks netral `No navigation items` bila slot tidak disuplai. Semua item hidden, atau hasil filtering yang hanya berisi separator/spacer/group kosong, diperlakukan sama. Item/branch disabled yang masih terlihat tetap merupakan konten dan tidak membuat surface dianggap kosong. Label panjang dipotong secara visual dengan nama aksesibel lengkap. Data invalid dikembalikan sebagai Result sebelum dipasang. Pergantian model/jenis dilakukan atomik, dan state presentation yang stale diproyeksikan tanpa crash atau event sintetis. Subscription dan task yang memang dimiliki upstream mengikuti lifetime entity/windownya, tanpa timer polling aplikasi.

## Build plan

Pendekatan rekomendasi adalah irisan end to end, selaras dengan foundation yang sudah ada: satu pola berjalan dari crate sampai consumer dan interaksi nyata sebelum menambah pola berikutnya. Ini rencana implementasi berikutnya, bukan pekerjaan yang dijalankan dalam sesi spec ini.

1. Bangun irisan sidebar leaf: crate, satu model valid, entity milik aplikasi, Activate, re-export facade, satu preview nyata, dan tes consumer. Tambahkan batas validator serta dependensi lokal berversi. Memenuhi **AC-1**, **AC-2**, **AC-3**, **AC-15**.
2. Lengkapi group/branch, expanded, collapsed rail, ikon, badge, empty, header/footer, scroll, dan focus reconciliation pada sidebar. Tambahkan regresi kontrol yang keluar dari tree serta event keyboard tunggal. Memenuhi **AC-4**, **AC-9**, **AC-10**, **AC-11**, **AC-13**, **AC-14**.
3. Tambahkan minidock dengan model flat aplikasi, validasi kind/model atomik, spacer, serta bukti bahwa hover/fokus tidak memperlebar rail. Memenuhi **AC-2**, **AC-5**, **AC-9**, **AC-11**, **AC-14**.
4. Tambahkan dockbar Inline dan Overlay dari model, event, serta list yang sama. Verifikasi kiri/kanan, pergantian panel, Escape, outside click, dan fokus setelah panel dihapus. Memenuhi **AC-3**, **AC-6**, **AC-9**, **AC-11**, **AC-13**, **AC-14**.
5. Tambahkan navbar dan flyout menggunakan Popover controlled upstream. Verifikasi disabled, nested branch, pointer/keyboard, external open sync, serta close/open cepat tanpa subscription bocor. Memenuhi **AC-3**, **AC-7**, **AC-8**, **AC-13**, **AC-14**.
6. Lengkapi story Navigation, contoh aplikasi facade, dokumentasi publik, ownership architecture, dan catatan kompatibilitas. Uji integrasi AppShell tanpa mengubah kontrak layout serta jalankan matriks tema/density/rem dan semua gate. Memenuhi **AC-1**, **AC-10** sampai **AC-18**.

## Consequences

- Positif: kelima pola berbagi kontrak native yang sama, aplikasi bebas menghubungkan tujuan dan persistence, dan perilaku kontrol/overlay tetap mengikuti upstream.
- Biaya: aplikasi harus menyimpan entity/subscription serta menangani permintaan perubahan. Crate tidak menyediakan router atau otomatis memilih tujuan pertama.
- Batas: bentuk panel, hover preview, grid, route context, registry badge, dan persistence Angular tidak disalin. Dockbar Overlay adalah panel nonmodal, bukan replika drawer web.
- Kompatibilitas: penambahan crate dan ekspor bersifat additive. Nama upstream serta API layout yang ada dipertahankan. Patch divider yang sudah ada tidak disentuh.
- Bukti: spec ini berstatus Proposed dan belum membuktikan implementasi, build, perilaku native, atau output pembaca layar.

## Follow-up

- [ ] Setelah spec dikonfirmasi, daftarkan scope navigasi native pada repo GPUI melalui `/scope`. Scope Angular fitur 26 yang terlanjur ditulis tidak menjadi backlog implementasi repo ini; koreksi lintas repo perlu sesi scope tersendiri.
- [ ] Laporkan keterbatasan platform dan pembaca layar sesuai mesin yang tersedia pada sesi implementasi. Tidak ada klaim Windows/Linux atau screen reader dari bukti macOS/headless saja.

## Rationale

Penalaran, perbandingan opsi, dan peta acuan ada di `rationale.md`. Langkah verifikasi ada di `verify.md` pada direktori spec ini.
