# 0003. System Settings

**Date**: 2026-11-17
**Status**: Proposed
**Konfirmasi desain**: 2026-11-17, pengguna menerima isi spec beserta keenam perbaikan pemeriksaan silang. Implementasi belum dimulai. Status tetap Proposed sampai tahap implementasi mengubah lifecycle fitur.

## Summary

Anda dapat mengatur aplikasi dalam satu jendela Settings native dengan dua halaman, General dan Theme & Appearance. Semua jendela memakai preferensi yang sama, perubahan tampilan langsung terlihat, dan pilihan tersimpan untuk penggunaan berikutnya. Komposisi Settings tersedia melalui `ui-kit`, sementara aplikasi memiliki penyimpanan dan kebijakan jendelanya.

## Requirements

User stories: Anda dapat membuka pengaturan dengan perintah yang familiar, mempertahankan tampilan pilihan Anda setelah aplikasi dibuka kembali, dan menggunakan komposisi yang sama pada aplikasi lain.

| ID | Acceptance criterion |
| --- | --- |
| AC-1 | Menu Settings…, tombol Settings, dan ⌘, pada macOS atau Ctrl+, pada Windows/Linux membuka satu jendela native nonmodal. Membuka ulang mengaktifkan instance yang sama, termasuk saat input memiliki fokus atau sidebar utama tersembunyi. |
| AC-2 | Sidebar Settings hanya berisi General dan Theme & Appearance dalam urutan itu. General tampil pada pembukaan pertama. Keyboard, fokus terlihat, nama kontrol, perpindahan halaman, overlay, dan penutupan bekerja sesuai kontrak di bawah. |
| AC-3 | General menampilkan pilihan membuka Overview atau halaman terakhir, nama dan versi aplikasi, serta reset seluruh preferensi dengan konfirmasi. Tujuan terakhir tidak valid kembali ke Overview. |
| AC-4 | Light, Dark, dan System memilih varian dari pasangan tema aktif. System mengikuti notifikasi appearance native tanpa memulai ulang aplikasi, dengan ID keluarga tema dan aksen tetap. |
| AC-5 | Tema terdaftar dan satu aksen opsional berlaku pada kedua mode. Warna tidak valid tidak mengubah preferensi aktif. Gunakan aksen tema menghapus override saja dan memulihkan warna pasangan aktif. Teks pada tombol aksen tetap terbaca menurut aturan kontras di bawah. |
| AC-6 | Comfortable/Compact, visibilitas sidebar aplikasi, lebar sidebar 160 sampai 400 logical pixel, dan reset layout bekerja langsung. Perubahan tetap menyisakan akses Settings dan konten yang dapat digunakan pada ukuran jendela minimum. |
| AC-7 | Skala 100%, 110%, 125%, dan 150% mengubah ukuran teks dan kontrol berbasis rem dari sumber tema, tanpa akumulasi pengalian. Semua kontrol Settings tetap terjangkau pada skala maksimum dan ukuran jendela minimum. |
| AC-8 | Reduce Motion menyediakan System dan Reduce. Override Reduce tidak dibatalkan oleh pembaruan sistem atau tema. Beralih kembali ke System memakai preferensi sistem terbaru sesuai batas platform yang didokumentasikan. |
| AC-9 | Reset appearance mempertahankan General, density dan layout sidebar. Reset layout mengembalikan Comfortable beserta visibilitas/lebar sidebar, sambil mempertahankan General dan pilihan appearance lainnya. Reset seluruh preferensi mengembalikan semua pilihan serta katalog tema ke bawaan setelah konfirmasi, tanpa menghapus data aplikasi di luar preferensi. |
| AC-10 | JSON lokal dipulihkan sebelum jendela pertama ditampilkan. Perubahan preferensi, tema khusus dari galeri, dan tujuan terakhir bertahan setelah proses dibuka kembali. File hilang, rusak, terlalu besar, atau versi tidak didukung mengikuti aturan pemulihan tanpa crash. File versi masa depan atau file yang versinya tidak dapat dipastikan dilindungi dari penulisan, termasuk ketika pengguna mengubah atau mereset preferensi. |
| AC-11 | Penulisan terserialisasi dan penggantian file menjaga snapshot tersimpan tetap utuh. Kegagalan menyimpan mempertahankan perubahan aktif, menampilkan status belum tersimpan dan Coba lagi, serta tidak melaporkan tersimpan untuk revisi yang lebih baru dari snapshot disk. Quit hanya berakhir otomatis ketika revisi terbaru sudah tersimpan; revisi baru selama flush harus ikut disimpan dalam tenggat yang sama atau memicu dialog belum tersimpan. |
| AC-12 | Toolbar appearance, toggle sidebar, kontrol density Playground, galeri Themes yang menerapkan tema, dan Settings memakai jalur perubahan preferensi yang sama. Semua jendela sinkron dan perubahan tema tidak menghapus override pengguna lain. Preview kartu tidak memutasi preferensi. |
| AC-13 | Consumer kedua memakai `ui_kit::settings` tanpa dependency Showcase. Entity kontrol, navigasi, subscriptions, dan fokus dipertahankan di luar render. Komponen standar dan satu runtime tetap berasal dari GPUI Kit 0.9.1 serta GPUI 0.3.12. |
| AC-14 | Perintah validasi root, validator dependency, regresi bridge macOS, dan regresi adapter Reduce Motion lulus setelah implementasi. Bukti model murni, UI headless, jendela native, dan pembaca layar dilaporkan terpisah per platform. |

## Decision

**Chosen option**: crate komposisi `ui-settings`, state aplikasi bersama, dan JSON lokal menggunakan dependency `serde`, `serde_json`, dan `thiserror` yang sudah ada. Pengguna memilih jendela native terpisah, desain dari UI saat ini, pasangan tema, satu aksen, penyimpanan JSON, perubahan global yang sinkron, serta patch adapter pada versi yang sama. (basis: scope 002, AGENTS.md, Coding Guides GPUI Kit pada commit `39ab849a93b5f15507b969164d49891db8d5b470`.)

Rekomendasi internal: event meminta perubahan, aplikasi mengesahkan snapshot, lalu UI menyinkronkannya. Alternatif menaruh seluruh fitur pada `ui-layout` tidak dipilih karena lifecycle kontrol Settings berbeda dari komposisi layout yang stateless. Tidak ada database, layanan luar, credential, renderer, atau upgrade runtime baru.

### Kepemilikan

| Pemilik | Tanggung jawab |
| --- | --- |
| `ui-settings` | `SettingsValues`, General dan LayoutPreferences yang mengomposisikan tipe appearance dari `ui-theme`, validasi presentasi, dua halaman, entity kontrol, event permintaan, fokus dan scroll lokal; tidak membuka jendela atau file |
| `ui-theme` | `AppearancePreferences` beserta tipe skala/motion dan keluarga tema, resolusi appearance dan aksen, snapshot token, density, skala dan kebijakan motion |
| `ui-layout` | Validasi `ShellConfig` dan komposisi shell yang menerima preferensi layout |
| `ui-navigation` | Navigasi sidebar Settings dan lifecycle fokus sesuai spec 0002 |
| GPUI Kit / `ui-components` | Kontrol, Root per jendela, overlay, render, keyboard dan perilaku komponen |
| `ui-icons` | Ikon Settings dan Palette yang sudah tersedia |
| `ui-kit` | Ekspor `settings` dan tipe publik yang diperlukan, tanpa implementasi domain |
| Showcase | Controller bersama, tujuan aplikasi, menu, jendela tunggal, informasi aplikasi, file preferensi, retry dan kebijakan penutupan |

Dependency internal baru: `ui-settings -> ui-core, ui-components, ui-theme, ui-icons, ui-layout, ui-navigation`; `ui-kit -> ui-settings`. Semua memakai path dengan alternatif versi. `ui-settings` memakai runtime dependency GPUI Kit yang dipin, tanpa bergantung pada `ui-kit`. Arah dependency crate lain tetap. Validator mengenali library baru beserta batas ini.

`AppearancePreferences`, enum skala/motion dan `ThemeFamilyDefinition` didefinisikan di `ui-theme`, lalu diekspor kembali oleh `ui-settings` bila dibutuhkan consumer. `SettingsValues` mengomposisikannya. `ThemeService` hanya menerima tipe milik lapisannya; tidak ada dependency balik dari `ui-theme` ke `ui-settings`. Density tetap field appearance untuk kebutuhan resolver, tetapi cakupan reset mengikuti tugas pengguna pada bagian Layout.

## Feature design

### Model data

Tipe berikut adalah kontrak yang akan dibangun, bukan klaim bahwa API tersebut sudah tersedia. Field struct privat dengan constructor, reader dan validasi. Enum publik yang perlu berkembang memakai `#[non_exhaustive]`.

| Tipe / field | Tipe nilai, kewajiban dan bawaan |
| --- | --- |
| `SettingsValues` | Nilai wajib berisi `general`, `appearance`, `layout`; satu snapshot per aplikasi dan pengguna OS |
| `GeneralPreferences.startup` | `Overview` atau `LastDestination`, wajib, default `Overview` |
| `AppearancePreferences.mode` | `ThemeMode`, wajib, default `System` |
| `AppearancePreferences.family_id` | ID keluarga tema, wajib, default `default`; referensi ke katalog keluarga |
| `AppearancePreferences.accent` | `Option<String>`, default `None`; override `#RRGGBB`, diserialisasi uppercase |
| `AppearancePreferences.density` | `Density`, wajib, default `Comfortable` |
| `AppearancePreferences.scale` | Enum `Percent100`, `Percent110`, `Percent125`, `Percent150`, default `Percent100` |
| `AppearancePreferences.motion` | Enum `System` atau `Reduce`, default `System` |
| `LayoutPreferences.sidebar_visible` | Boolean wajib, default true |
| `LayoutPreferences.sidebar_width` | `f32` finite, 160 sampai 400, default 224 logical pixel; divalidasi melalui `ShellConfig::with_sidebar_width` |
| `SettingsPage` | `General` atau `ThemeAppearance`, hanya state sesi, default `General` |
| `ThemeFamilyDefinition` | ID dan label tidak kosong, satu `ThemeDefinition` Light dan satu Dark; ID unik, nama definition unik, flag mode sesuai sisi pasangan |
| `PreferencesDocument` | Envelope milik Showcase: `schema_version: u32 = 1`, `values`, `last_destination: Option<String>`, `custom_families: Vec<ThemeFamilyDefinition>` |
| `AppInformation` | Nama dan versi yang disuplai consumer; Showcase memakai nama `Rust UI Kit Showcase` dan versi package pada waktu build |

Relasi: satu dokumen memiliki tepat satu General, Appearance dan Layout. Banyak snapshot dapat merujuk satu keluarga tema. Satu keluarga memiliki tepat dua definition. Tidak ada akun, profil, primary key database, atau sinkronisasi cloud. ID aplikasi dan pengguna OS menentukan lokasi dokumen. Nama dan versi bukan field yang dapat diedit atau disimpan sebagai preferensi.

Katalog awal menyediakan keluarga `default` (definition `light` dan `dark` saat ini) serta `ocean`. Definition Ocean memakai salinan pasangan default, dengan primary Light `#0F766E`, primary Dark `#2DD4BF`, focus ring mengikuti primary dan foreground dihitung oleh aturan aksen. Nilai warna berada di sumber tema `ui-theme`, bukan literal dalam view. ThemeRegistry tetap satu pemilik definition; metadata keluarga menunjuk definition yang sama.

Galeri membuat keluarga khusus dengan menyalin kedua definition dari keluarga aktif dan menerapkan warna yang dimasukkan ke keduanya. ID `custom-N` memakai angka berikutnya setelah ID khusus tertinggi dalam dokumen (mulai 1 bila kosong), label picker `Custom N`, dan nama definition `<family-id>-light` serta `<family-id>-dark`. Angka diperiksa sebelum penambahan agar overflow ditolak. Maksimum 32 keluarga khusus. Memilih keluarga dari picker saja tetap mempertahankan aksen. Keluarga khusus disimpan lengkap dalam `custom_families`, agar dapat diregistrasikan sebelum preferensi dipulihkan. Tidak ada editor token lengkap atau penghapusan tema khusus satu per satu pada tahap ini.

Register and apply memanggil satu transaksi `SettingsController::create_and_apply_family(accent)`. Transaksi menyiapkan kandidat registry/katalog, ID/label, pasangan definition, dokumen dengan ukuran yang sah dan resolusi tema sebelum memutasi state aktif. Saat semua valid, commit registry, keluarga terpilih, aksen input dan dokumen current dalam satu perubahan koheren, tambah satu revision, notify dan jadwalkan save. Mode serta override lain tetap. Kegagalan validasi/apply tidak meninggalkan registrasi atau seleksi parsial; kegagalan I/O sesudah commit mengikuti SaveFailed dan tidak membatalkan tampilan.

Reset All memakai transaksi penggantian katalog dengan keluarga bawaan Default/Ocean dan definition aslinya, mengeluarkan seluruh keluarga khusus milik Settings beserta definition mereka dari registry aktif dan dokumen. Snapshot tema diselesaikan dari katalog kandidat sebelum commit. Definition standalone yang diregistrasikan consumer di luar katalog keluarga Settings tetap milik consumer dan tidak dihapus. Pemeriksaan tabrakan mencakup seluruh registry. Urutan buat, reset, buat lagi dapat memakai `custom-1` kembali setelah nama lama benar benar dilepas. Semua view menerima katalog baru melalui sinkronisasi snapshot yang sama.

### Antarmuka publik dan event

Semua operasi adalah fungsi Rust lokal, tanpa endpoint jaringan atau auth aplikasi.

| Surface yang dibangun | Input utama | Output / kejadian | Kesalahan penting |
| --- | --- | --- | --- |
| `SettingsView::new` | ID instance, snapshot valid, katalog, `AppInformation`, window dan context | Entity view dengan kontrol dan dua halaman | Snapshot atau katalog tidak valid ditolak sebelum view dibuat |
| `SettingsView::set_snapshot` | Snapshot, katalog dan status penyimpanan dari aplikasi | Sinkronisasi kontrol dan satu notify, tanpa event perubahan pengguna | Snapshot tidak valid tidak menggantikan snapshot sebelumnya |
| `SettingsEvent::Change` | `SettingsChange` bertipe: startup, mode, family, accent, density, scale, motion, sidebar visible/width | Permintaan perubahan, tanpa mutasi preferensi aplikasi tersembunyi | Aplikasi dapat menolak dan mempertahankan snapshot |
| `SettingsEvent::Reset` | `ResetScope::{Appearance, Layout, All}` | Permintaan reset setelah konfirmasi untuk `All` | Cancel tidak menghasilkan reset |
| `SettingsEvent::RetrySave` | Tidak ada payload | Permintaan menyimpan snapshot terbaru pada storage writable, atau klasifikasi ulang saja pada Protected | Status gagal/perlindungan tetap terlihat bila retry gagal |
| `SettingsView::restore_focus` | Window dan context | Fokus ke root Settings yang tetap terpasang sebelum konten dilepas | Tidak bergantung pada kontrol halaman yang akan hilang |
| `ThemeRegistry::register_family` | `ThemeFamilyDefinition` | Pasangan dan metadata tervalidasi terdaftar secara atomik | ID/nama duplikat, pasangan mode salah, token tidak valid |
| `ThemeRegistry::replace_family_catalog` | Katalog keluarga kandidat tervalidasi | Penggantian pasangan dan metadata katalog, melepas definition keluarga yang dihapus dan mempertahankan definition standalone | ID/nama bertabrakan atau pasangan tidak valid, tanpa mutasi parsial |
| `ThemeService::apply_preferences` | `AppearancePreferences`, appearance OS yang diambil melalui adapter GPUI | Snapshot tema efektif serta refresh seluruh jendela | Keluarga/warna tidak valid ditolak sebelum mutasi global |
| `SettingsController::request_change` (Showcase) | Perubahan bertipe atau reset | Validasi, apply, revisi baru, notify dan jadwal save | Nilai tidak valid, storage sedang dimiliki proses lain |
| `SettingsController::create_and_apply_family` (Showcase) | Warna hex valid dari galeri | Label/ID pasangan, registry, seleksi dan aksen committed bersama, satu revision | Batas jumlah/ukuran, ID/nama bertabrakan, resolusi tidak valid; kandidat tidak meninggalkan state parsial |
| `PreferencesStore::load/save` (Showcase) | Lokasi tetap dan dokumen / revision | Dokumen valid atau hasil save untuk revision tertentu | I/O, JSON, future schema, lock, ukuran file |
| `OpenSettings` / `CloseSettings` (Showcase actions) | Perintah menu, shortcut atau tombol | Aktifkan/buka satu jendela, atau tutup melalui kebijakan flush | Window handle sudah tutup dibersihkan; gagal membuka ditampilkan di shell |

Consumer mempertahankan subscription event dan observer view/state selama jendela hidup. Control input/select/slider dan `NavigationState` dibuat dalam constructor. Sinkronisasi snapshot tidak memancarkan `Change`, termasuk ketika input aktif. Input aksen yang belum valid adalah draft lokal; apply pada Enter atau akhir edit yang valid. Input kosong, termasuk setelah Enter atau blur, tetap draft lokal dan tidak otomatis menghapus override. Tindakan Gunakan aksen tema mengirim `SettingsChange::Accent(None)` untuk menghapus override saja. Setelah diterima aplikasi, input dan preview kembali mencerminkan warna pasangan aktif, sementara preferensi lain tetap. Pesan validasi tetap dekat input. Mengganti halaman boleh membuang draft yang belum diaplikasikan, tanpa menulis draft ke disk.

### Resolusi tema dan skala

Urutan resolusi adalah keluarga, varian mode, override aksen, override density/skala/motion, lalu proyeksi token dan komponen. Mode System disimpan sebagai System, bukan hasil Light/Dark terakhir. Setiap jendela mempertahankan appearance observer yang meneruskan perubahan ke controller; event OS yang menghasilkan varian efektif sama tidak membuat revision atau save baru.

Seluruh resolusi dihitung dari definition asli, bukan snapshot yang sudah diskalakan. `Theme::update` menjadi jalur proyeksi semantic token, alias komponen dan Base. Skala mengalikan ukuran teks dan line height dari definition asli serta mengatur `theme.font_size` dan `mono_font_size`; Root upstream menentukan `window.rem_size()` dari base font tersebut. Layout rem mengikuti base font, tanpa override rem manual pada setiap render. Lebar sidebar disimpan dalam logical pixel sesuai `ShellConfig`; pada layar sempit collapse responsif tetap bekerja dan tidak menulis ulang preferensi visibilitas.

Density mengatur spacing shell melalui `content_gap` dan ukuran kontrol Settings melalui API ukuran upstream. Hal ini tidak menjanjikan perubahan ukuran semua komponen upstream. Kontrol demo yang sengaja menguji ukuran eksplisit tetap menunjukkan ukuran demo tersebut.

Aksen menerima tepat enam digit hex ASCII setelah `#`, dengan trim pada batas input. Pilih hitam atau putih untuk primary foreground berdasarkan rasio kontras terbesar dari luminansi sRGB relatif. Target rasio minimal 4.5:1 untuk teks pada primary normal, hover dan active. Turunkan hover/active dari primary dengan penyesuaian lightness yang sama arahnya seperti proyeksi saat ini; bila rasio terhadap foreground terpilih kurang, kurangi jarak penyesuaian sampai memenuhi target, dengan primary normal sebagai batas akhir yang pasti valid. Foreground success, warning dan danger dihitung dari background feedback masing masing, sehingga pilihan aksen tidak menurunkan keterbacaannya. Warna aksen tidak menjadi warna teks isi atau satu satunya penanda status. Bukti rasio ini bukan sertifikasi aksesibilitas seluruh aplikasi.

API lama `select`, `select_named` dan `set_density` tetap tersedia bagi consumer lama. Showcase beralih ke jalur controller dan `apply_preferences`, sehingga aksi lama yang melakukan reset field tema tidak membatalkan override pengguna. Light/Dark lama di galeri memetakan mode keluarga aktif. Kartu token dan preview warna membaca definition tanpa menerapkannya secara global. Perintah Register and apply membuat dan menyimpan keluarga khusus seperti aturan model.

### Adapter Reduce Motion

Pengguna memilih perbaikan pada vendor `gpui-base` 0.9.1 yang sudah ada. Tambahkan API eksplisit `set_reduce_motion_override(Option<bool>, cx)` pada modul adapter, ekspor melalui Base. Adapter menyimpan pembacaan sistem terakhir terpisah dari override. Callback portal selalu memperbarui pembacaan itu, lalu effective motion adalah override bila ada atau pembacaan sistem terbaru. Override `Some(true)` tidak dapat dibatalkan callback sistem. `None` membaca ulang platform yang sinkron dan menyerahkan kendali pada sistem. Perilaku caller lama yang langsung memakai `App::set_reduce_motion` tetap dilindungi oleh deteksi kepemilikan yang ada saat API override belum dipakai; caller API baru secara eksplisit menyerahkan ownership kepada adapter untuk masa override tersebut.

Di macOS/Windows, baca ulang saat startup, saat memilih System, dan saat jendela aplikasi aktif kembali. Tidak mengklaim notifikasi perubahan motion native saat aplikasi tetap aktif. Linux memakai pembacaan serta event portal yang sudah ada; jika sistem tidak memberi nilai, default false dipakai untuk System. Tema tidak boleh membawa nilai `reduced_motion` lama yang mengalahkan preferensi pengguna pada jalur Settings. Hindari nol durasi yang tersimpan permanen dalam definition; policy motion efektif diproyeksikan ulang dari sumber asli.

Patch dibatasi ke adapter, ekspor API yang diperlukan dan regresinya. Pertahankan versi, provenance, lisensi, patch divider dan regresi selection yang sudah ada. Rekam diff baru, hash `source-integrity.json`, penjelasan `PATCHES.md`, serta validator dengan daftar file yang benar. Tambahkan pengujian modul adapter produksi di vendor dan jalankan terpisah karena vendor bukan anggota workspace. Tidak mengubah bridge AccessKit untuk fitur ini.

### Komposisi dan lifecycle jendela

Sumber desain adalah UI aplikasi yang ada dan semantic token saat ini. Jendela berjudul Settings, default 820 × 680 logical pixel, minimum 640 × 520. Sidebar Settings memakai lebar 12 rem dan tetap memiliki dua item; pengaturan visibilitas/lebar sidebar hanya memengaruhi shell aplikasi utama. Konten kanan dapat menggulung dan label membungkus, tanpa overflow horizontal pada skala 150%. Gunakan kelompok baris label, penjelasan singkat bila perlu dan kontrol, dengan batas halus, bukan kumpulan kartu dekoratif. Tidak memerlukan aset raster baru.

General berurutan: On launch, Application (nama/versi), lalu Reset all settings…. Theme & Appearance berurutan: Appearance, Theme dan Accent (termasuk Gunakan aksen tema), Layout (Density, Show sidebar, Sidebar width, Reset layout), Text size, Reduce Motion, lalu Reset appearance. Tombol reset All membuka Dialog native komponen dengan Cancel sebagai fokus/default aman dan Reset sebagai tindakan yang menjelaskan hasilnya. Escape membatalkan dialog sebelum mencapai handler jendela. Reset layout dan appearance dapat langsung dilakukan karena lingkupnya jelas dan nilainya dapat dipilih ulang.

Menu/key binding dipasang sekali pada bootstrap, binding lebih dahulu daripada `set_menus`. Pada macOS letakkan Settings… pada menu aplikasi. Pada Windows/Linux gunakan menu aplikasi yang disajikan GPUI Kit, termasuk fallback window menu bila platform tidak memiliki menubar native. Tombol Settings pada header utama tetap tersedia tanpa sidebar. Semua entry point mengirim `OpenSettings` yang sama. Scope Settings tidak masuk ke tujuh tujuan konten utama atau tujuan terakhir.

Controller memegang satu handle jendela Settings. Pembukaan pertama memilih General; pembukaan ulang instance hidup mempertahankan halaman dan mengaktifkan jendelanya. Sesudah jendela benar benar ditutup, pembukaan berikutnya kembali ke General. Simpan handle fokus pemicu dan window pemiliknya. Saat ditutup, aktifkan pemilik yang masih hidup dan fokuskan pemicu jika masih terpasang, selain itu shell fallback. Saat halaman berubah, pindahkan fokus ke tombol navigasi terpilih atau root sebelum kontrol lama dilepas, lalu biarkan Tab menuju konten.

Tab/Shift+Tab mengikuti kontrol native. Sidebar memakai `ui-navigation` tanpa mengarang kontrak keyboard baru. ⌘W pada macOS dan Ctrl+W pada Windows/Linux hanya menutup Settings ketika jendela itu aktif; tombol tutup native memakai kebijakan yang sama. Escape hanya menutup overlay paling atas, tanpa menutup jendela Settings biasa. Menutup jendela utama menutup Settings terkait melalui kebijakan flush yang sama, lalu mempertahankan aturan aplikasi keluar ketika tidak ada jendela. Tidak membuat jendela utama baru saat pemilik telah ditutup.

### Penyimpanan dan state transitions

Controller adalah satu `Entity` yang dimiliki bootstrap aplikasi dan dibagikan sebagai handle ke view. Ia memegang current values, dokumen tersimpan terakhir, monotonically increasing revision, revisi tersimpan, status save, katalog dan handle jendela. Tema efektif hanya proyeksi dari current values. Otoritas preferensi tidak diduplikasi di view.

Alur: Load → Ready; perubahan valid → Dirty → Saving → Saved atau SaveFailed. `StorageProtected` adalah status storage terpisah yang mencegah masuk ke Saving, sementara current values dapat berubah untuk sesi saja. Perubahan saat Saving tetap menjadi revision baru dan hanya snapshot terbaru yang menunggu ditulis. Hasil save revision lama tidak mengubah current values atau membersihkan Dirty revision baru. Retry biasa memakai dokumen terbaru; retry Protected mengikuti aturan klasifikasi ulang di bawah. Mutasi identik tidak menambah revision. Controller memberi notify sekali per perubahan koheren. I/O berjalan pada executor background dengan satu antrean writer, bukan dari render atau tiap keystroke.

Debounce save 250 ms setelah perubahan valid, termasuk perubahan tujuan utama. Saat close atau Quit, flush snapshot terbaru dengan satu tenggat absolut 2 detik sejak permintaan pertama. Penyelesaian flush dan keputusan benar benar keluar berlangsung dalam pembaruan controller yang sama: wajib `persisted_revision == current_revision` dan tidak ada hasil durability yang belum terkonfirmasi. Jika ada revision baru dari jendela lain ketika writer bekerja, lanjutkan flush terbaru memakai sisa tenggat, tanpa memperpanjangnya. Permintaan Quit berulang digabung; setelah keputusan keluar final, controller menolak mutasi baru dan menonaktifkan dispatch sebelum melepas jendela. Keberhasilan snapshot lama saja tidak boleh menyelesaikan Quit.

Jika save gagal atau melewati tenggat, tampilkan dialog Keep using app dan Quit without saving atau Close without saving sesuai tindakan. Dialog mengacu pada kondisi dirty terbaru. Pilihan tanpa menyimpan tidak mengembalikan tampilan aktif ke snapshot lama. Untuk menutup Settings saja, controller dan antrean retry tetap hidup di aplikasi; jika kemudian Quit, peringatan memakai kondisi dirty terbaru. Close native, menu dan shortcut tidak melewati kebijakan ini.

Path tetap Showcase menggunakan ID `dev.rust-ui-kit.showcase`: macOS `~/Library/Application Support/dev.rust-ui-kit.showcase/preferences.json`; Windows `%APPDATA%/dev.rust-ui-kit.showcase/preferences.json`; Linux `$XDG_CONFIG_HOME/dev.rust-ui-kit.showcase/preferences.json` bila root absolut, selain itu `~/.config/dev.rust-ui-kit.showcase/preferences.json`. Tanda `~` pada spec berarti home pengguna OS, bukan string literal. Home/APPDATA yang tidak tersedia menghasilkan sesi tanpa penyimpanan dengan status jelas, tanpa fallback ke direktori kerja. Consumer kedua menyediakan path dan ID sendiri.

Baca maksimum 256 KiB dengan probe satu byte tambahan untuk mendeteksi ukuran berlebih. Snapshot yang akan disimpan juga dibatasi 256 KiB setelah serialisasi; registrasi keluarga yang melampaui batas ditolak sebelum mengubah state. File tidak ada menghasilkan default tanpa error. Sebelum decoding nilai, parse envelope JSON utuh yang harus berupa object dan tentukan schema. Dokumen lama tanpa `schema_version` diperlakukan sebagai schema 1 hanya bila envelope object utuh dapat diparse. Schema 1 dikenali sebelum pemulihan field: field hilang memakai default per field; field tidak dikenal diabaikan agar perluasan aman. Field nilai rusak kembali ke default field itu dan dicatat sebagai recovery; pasangan tema yang rusak ditolak utuh, dan ID keluarga yang hilang kembali ke default tanpa menghapus override valid lain.

File kebesaran, gagal dibaca, JSON tidak dapat diparse, atau field schema tidak dapat dipastikan menghasilkan default dengan status `StorageProtected`. Schema yang dikenali tetapi bukan 1 juga menghasilkan status itu dengan alasan versi tidak didukung. File asli pada path preferences tidak ditimpa, dipindah atau dipotong, termasuk bila ukuran berlebih ternyata memuat versi masa depan. Backup bukan izin untuk menimpa versi yang belum diketahui. Perubahan dan reset dapat diterapkan sementara pada sesi dengan pesan tidak akan tersimpan; tidak ada writer yang diantrekan pada status Protected. Ini berbeda dari proses lain memegang lock, yang menolak mutasi sebagaimana aturan lock.

`RetrySave` pada status Protected hanya mencoba load/klasifikasi ulang di background, tanpa menulis dokumen current. Bila file masih tidak diketahui/tidak didukung, pertahankan perlindungan. Bila file telah diganti eksternal menjadi schema 1 yang dapat dipulihkan atau sudah tidak ada, pertahankan perubahan sesi dan minta pilihan eksplisit Gunakan file tersimpan atau Simpan pilihan sesi sebelum melakukan rekonsiliasi. Hanya pilihan kedua yang mengizinkan save ke sumber yang sudah terbukti didukung itu. Dialog close/Quit tetap memperingatkan perubahan sesi yang belum tersimpan. Tidak ada tombol Reset All yang melewati perlindungan file.

Sebelum pertama menimpa file yang sudah dipastikan schema 1 dan memerlukan recovery field, pertahankan byte aslinya sebagai `preferences.recovery-N.json`, memakai nomor bebas berikutnya dan tanpa overwrite. Jika backup gagal, pertahankan sesi aktif tetapi save gagal. Hanya snapshot schema 1 tervalidasi yang ditulis. Recovery tidak otomatis menulis pada saat load; perubahan pengguna atau retry eksplisit menjadi pemicu save pada storage yang dapat ditulis. Proses yang menjadi writer memperoleh lock sebelum load/klasifikasi dan mempertahankannya selama hidup proses.

Writer memegang exclusive lock file `preferences.lock` menggunakan API file lock standard library selama proses hidup. Lock dibuat tanpa truncate. Jika proses lain memegang lock, aplikasi tetap dapat dibuka dengan status preferensi dimiliki proses lain, tetapi mutasi preferensi ditolak dan kontrol dinonaktifkan dengan alasan. Tidak ada sinkronisasi lintas proses atau penguncian manual berbasis keberadaan file. Kegagalan memperoleh lock karena I/O berbeda dari lock sibuk dan mengikuti SaveFailed.

Tulis file sementara unik dengan `create_new` pada direktori yang sama, serialize snapshot penuh, `flush` dan `sync_all`, lalu rename mengganti file tujuan. Bila gagal sebelum replace, snapshot tersimpan lama tetap utuh. Sync direktori pada platform yang mendukungnya; kegagalan setelah replace dilaporkan sebagai durability belum terkonfirmasi dan retry aman. Bersihkan file sementara milik operasi sendiri saat gagal, jangan memindai/menghapus file pengguna lain. Simpan file konfigurasi dengan akses pengguna OS; tidak berisi secret atau data domain. Log hanya operasi, revision dan jenis error, tanpa dump seluruh dokumen.

Reset appearance mencakup mode, keluarga, aksen, skala dan motion, dengan density serta layout sidebar dipertahankan. Reset layout mengembalikan density Comfortable, sidebar terlihat dan lebar 224, tanpa mengubah General, mode, keluarga, aksen, skala atau motion. Cakupan reset ini independen dari letak field density pada `AppearancePreferences`. Reset All mencakup seluruh values, `last_destination = None` serta penggantian katalog aktif dan tersimpan ke bawaan melalui transaksi katalog yang dijelaskan di atas, setelah konfirmasi. Navigasi utama berikutnya dapat kembali mencatat `last_destination`; reset tidak memindahkan halaman utama secara tersembunyi. Dalam sesi Protected, reset hanya mengubah state sesi.

### Value sourcing dan invariants

| Action / tampilan | Nilai | Sumber |
| --- | --- | --- |
| Membuka Settings | Halaman, nama/versi, dua label sidebar | `SettingsPage` sesi, `AppInformation`, konstanta halaman |
| Startup | Tujuan konten | `general.startup`, `last_destination`, lookup ID `Section` yang masih tersedia, fallback Overview |
| Pilih keluarga/mode | Definition aktif | `family_id`, katalog tervalidasi, mode tersimpan, `WindowAppearance` / appearance App GPUI |
| Ubah aksen | Primary, ring, foreground dan state warna | Input hex tervalidasi, rumus luminansi dan batas kontras pada spec |
| Gunakan aksen tema | Override `None`, warna Light/Dark bawaan keluarga aktif | Tindakan eksplisit `SettingsChange::Accent(None)` dan pasangan definition terpilih |
| Ubah density/skala | Ukuran kontrol dan rem | Density/skala tersimpan dan definition asli, Root upstream |
| Ubah layout | Visibilitas, lebar, collapse responsif | LayoutPreferences, ShellConfig dan lebar jendela saat ini |
| Ubah motion | Flag efektif | Override pengguna, pembacaan native terakhir atau event portal, fallback false |
| Terapkan tema galeri | ID, label dan definition pasangan | Keluarga aktif, input warna, ID khusus tertinggi pada dokumen, format `Custom N`, transaksi create/apply |
| Status tersimpan | Saved / belum tersimpan / error | Current revision, persisted revision, hasil writer untuk revision tertentu |
| StorageProtected | Alasan tidak dapat menulis dan status sesi sementara | Hasil batas baca, parse envelope dan klasifikasi schema sebelum decoding nilai |
| Penyelesaian Quit | Boleh keluar atau lanjut flush/dialog | Perbandingan revision controller saat callback selesai dan tenggat absolut dari waktu permintaan awal |
| Reset | Snapshot bawaan yang tepat | Default model dan `ResetScope` yang dikonfirmasi |
| Tutup Settings | Fokus yang dikembalikan | Handle pemicu dan window pemilik, fallback shell yang masih mounted |

Invariants: dua halaman Settings tetap; semua mutasi global melewati controller; snapshot valid sebelum apply; callback sync tidak menghasilkan event pengguna; tema hanya satu proyeksi; hasil I/O lama tidak menggantikan state baru; file hanya milik aplikasi/pengguna ini. Tidak ada autentikasi, peran organisasi, telemetry, environment variable baru atau credential untuk fitur lokal ini. Akses file mengikuti izin OS. Masukan tidak dipercaya sebelum validasi.

### Critical test scenarios

| Skenario | Kontrak |
| --- | --- |
| Menu, tombol, shortcut dari input, pembukaan berulang, close lalu reopen dan fokus pemicu hilang | AC-1, AC-2 |
| Mode startup, ID halaman hilang, metadata versi, Reset appearance saat Compact, Reset layout saat Compact, serta Reset All/Cancel | AC-3, AC-9, AC-10 |
| Dua keluarga, ketiga mode, aksen kosong/valid/salah, Gunakan aksen tema pada kedua mode tanpa mengubah preferensi lain, event appearance berulang dan kontras seluruh state tombol | AC-4, AC-5, AC-12 |
| Buat keluarga, Reset All, buat lagi tanpa restart; label deterministik, tidak ada duplikat tertinggal, kegagalan kandidat tidak meninggalkan registry parsial | AC-5, AC-9, AC-10, AC-12 |
| Settings dan shell utama pada 640 × 520, skala 150%, kedua density dan sidebar tersembunyi/lebar maksimum | AC-2, AC-6, AC-7 |
| Override motion, pembaruan sistem saat override, kembali System, dan pemanggil lama adapter | AC-8 |
| Load/save restart, keluarga khusus, field hilang/rusak schema 1, backup gagal, JSON tidak dapat diparse, schema masa depan sekaligus file terlalu besar, izin baca ditolak; byte file asli tetap setelah perubahan/reset sesi | AC-10, AC-11 |
| Save tertunda, perubahan baru saat writer bekerja, retry/reklasifikasi Protected, lock proses kedua, serta flush/close gagal dan perubahan jendela kedua saat Quit menunggu revision lama | AC-11, AC-12 |
| Consumer kedua dan dependency/source integrity, lifecycle subscription setelah penutupan | AC-13, AC-14 |

## Build plan

Pendekatan Tracer Bullet memulai satu alur lengkap sebelum memperluas pilihan. Dokumen schema 1 disiapkan dari awal; kontrol lanjutan dapat ditambahkan pada slice berikutnya tanpa mengganti format.

1. Tambahkan batas `ui-settings`, model dasar, controller Showcase, store schema 1 dan satu jendela dengan dua tujuan. Hubungkan menu, tombol, shortcut, satu perubahan startup, save dan load ulang sampai alur pertama bekerja. AC-1, AC-2, AC-3, AC-10, AC-11, AC-13.
2. Tambahkan keluarga Default/Ocean, resolver override, theme picker dan aksen; arahkan toolbar serta galeri ke controller, dengan pasangan khusus yang ikut disimpan. Buktikan pergantian System tanpa menghapus pilihan lain. AC-4, AC-5, AC-10, AC-12.
3. Hubungkan density, layout, skala, tiga lingkup reset dan fokus sebelum pelepasan konten. Lengkapi patch adapter motion pada versi sama beserta source integrity dan tes produksinya. AC-2, AC-6, AC-7, AC-8, AC-9, AC-12.
4. Lengkapi recovery, antrean revision, retry, lock proses, flush/close, observer semua jendela dan consumer `examples/settings-app` melalui facade saja. Dokumentasikan API publik serta resep kedua override vendor yang telah ada dan patch motion baru. AC-1, AC-10, AC-11, AC-12, AC-13.
5. Jalankan semua pemeriksaan root, regresi adapter vendor dan bridge macOS, lalu buktikan penggunaan native menurut ledger. Catat batas Windows/Linux dan pembaca layar sesuai bukti nyata. AC-1, AC-2, AC-3, AC-4, AC-5, AC-6, AC-7, AC-8, AC-9, AC-10, AC-11, AC-12, AC-13, AC-14.

## Consequences

Komposisi dapat dipakai ulang tanpa membawa file preferences atau tujuan Showcase. Biayanya adalah controller aplikasi yang harus mempertahankan subscriptions dan memasok snapshot yang benar. JSON mudah diperiksa tetapi memerlukan antrean writer, recovery dan batas versi yang eksplisit. Patch motion menambah pekerjaan menjaga integritas vendor dan resep consumer sampai upstream memiliki API yang setara.

Verifikasi pada macOS tidak membuktikan runtime Windows/Linux atau output pembaca layar. Mode System untuk motion mengikuti batas adapter yang dinyatakan, sedangkan appearance memakai notifikasi live. Pengaturan skala mengikuti model rem upstream sehingga sebagian geometry pixel yang disengaja perlu bukti rendered, bukan asumsi.

## Follow-up

Desain telah dikonfirmasi dan milestone build tersedia pada scope 002. Langkah berikutnya yang dapat Anda jalankan adalah `/develop System Settings`; implementasi belum dimulai. Integrasi Git tetap off. Tidak ada commit, push, publikasi atau release dalam pekerjaan arsitektur ini. Scope 001 dan penerimaan navigasi yang belum selesai tidak ditutup oleh fitur ini.

## Rationale

Alasan keputusan dan bukti source: [rationale.md](rationale.md). Rencana pembuktian: [verify.md](verify.md).
