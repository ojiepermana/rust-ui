# Verify: navigasi native

Seluruh butir masih belum dijalankan. Dokumen ini menetapkan bukti yang dikumpulkan setelah implementasi, bukan hasil pengujian sesi spec.

## Kontrak pure dan facade

- [ ] Model valid berisi ID leaf, branch, group, separator, dan spacer. ID kosong/ganda, label kosong, serta branch kosong mengembalikan diagnostik yang tepat, termasuk bila node hidden. AC-2.
- [ ] Model tree ditolak saat kind Minidock; model flat diterima. Kegagalan configure/set_model/set_kind tidak mengubah snapshot terakhir. AC-2, AC-5.
- [ ] Selected ID hilang/hidden tidak memilih item lain. Reorder mempertahankan identitas, dan proyeksi ancestor tidak menjadikan group sebagai tujuan. AC-9.
- [ ] Badge 0, 1, 99, dan 120 menghasilkan hidden, 1, 99, serta 99+; label terakhir tetap memiliki angka 120. Badge teks dan ikon tidak ditemukan memiliki fallback yang terdokumentasi. AC-10.
- [ ] Metrics menolak nol, negatif, NaN, Infinity, rail di bawah 3 rem, dan lebar surface di bawah rail. Setter gagal mempertahankan nilai valid sebelumnya. AC-2, AC-12.
- [ ] Consumer hanya mengimpor `ui_kit::gpui`, `ui_kit::navigation`, serta foundation yang diekspor facade. Entity dibuat di constructor, subscription disimpan, dan tidak bergantung pada tipe Showcase. AC-1, AC-15.
- [ ] Model Arc yang sama dan model Arc baru dengan seluruh isi identik dianggap ekuivalen. Configure identik tidak mengirim notify/event, mengganti handle, atau memindahkan fokus. Metrics identik juga no-op. AC-3, AC-9.
- [ ] Setelah filtering hanya tersisa separator/spacer/group kosong: tampil empty state. List dengan item disabled masih terlihat tetap bukan empty. AC-4, AC-5, AC-17.

## Interaksi GPUI dengan view sebenarnya

Gunakan state dan renderer produksi, bukan salinan renderer di fixture. Rekam event serta snapshot aplikasi sebelum/sesudah setiap input.

- [ ] Pointer dan Enter/Space pada leaf masing masing menghasilkan tepat satu Activate. Tanpa setter aplikasi, selected tidak berubah. Setelah setter, selected dan konten aplikasi berubah. AC-3, AC-14.
- [ ] Configure/set_presentation dari aplikasi tidak memancarkan event interaksi. Nilai sama tidak memicu redraw loop. AC-3.
- [ ] Simpan subscription observe pada view penampil. Ubah model, presentasi, metrics, dan registry ikon secara programatis tanpa input atau refresh window lain: hasil baru tampak. Melepas view melepaskan observer dan subscription event. AC-3, AC-15.
- [ ] Sidebar expanded: group, nested branch, selected descendant, disabled branch, header/footer, dan scroll bekerja. Collapsed: leaf dan branch dapat dicapai dengan Tab serta dibuka tanpa hover. AC-4, AC-11, AC-14.
- [ ] Minidock tetap berlebar rail saat hover, fokus, serta permintaan collapsed false; tidak ada toggle yang tidak bekerja. Spacer menggeser bagian bawah hanya bila ada surplus ruang. AC-5, AC-11.
- [ ] Dockbar Inline dan Overlay di kiri/kanan membuka branch yang diminta aplikasi, dapat mengganti panel, dan tidak berubah akibat pointer leave. AC-6.
- [ ] Navbar tidak membuka panel pada hover; branch dapat dibuka lewat keyboard, dipilih leafnya, lalu ditutup oleh keputusan aplikasi. AC-7, AC-14.
- [ ] Flyout membuka satu popup, menampilkan group/list dan expanded branch dari aplikasi, serta tombol tutup meminta nilai false. AC-8.
- [ ] Perubahan open eksternal tidak menghasilkan echo callback. Escape/outside click menghasilkan satu permintaan close. Ulangi rapid open/close dan unmount saat popup terbuka tanpa registrasi atau subscription tersisa. AC-3, AC-8, AC-13.
- [ ] Popup A terbuka lalu klik/konfirmasi pemicu B: tepat satu PanelChange(Some(B)), tidak ada close A publik yang kemudian menutup B. Replay callback A dengan revisi lama tidak mengubah popup B. AC-3, AC-6, AC-7.
- [ ] Aplikasi mengabaikan permintaan open maupun close tanpa memanggil setter: pada frame berikutnya popup dan fokus kembali mengikuti snapshot yang sama, tanpa event echo, input tambahan, atau redraw loop. AC-3, AC-13.
- [ ] Disabled leaf/disclosure tidak menghasilkan event, tidak merespons Enter/Space, dan tidak memperoleh hover/pressed aktif. AC-12, AC-14.
- [ ] Fokus di nested leaf lalu tutup branch: fokus kembali ke disclosure. Hapus node/branch terfokus: fokus berpindah ke ancestor valid atau fallback aplikasi. AC-13.
- [ ] Ganti kind ketika fokus berada di kontrol yang hilang, lalu jalankan shortcut shell. Lepas entity dengan restore_focus dan buktikan focus target masih berada di tree. AC-13.
- [ ] Sidebar collapsed memakai model group → branch → leaf: branch dalam group hadir di rail dan leaf dapat dijangkau lewat keyboard tanpa hover. AC-4, AC-9.
- [ ] Fokus kontrol header/footer atau popup lalu sembunyikan region sidebar melalui resize di bawah collapse_below, dengan entity tetap hidup: fokus pindah ke fallback mounted dan shortcut shell bekerja. AC-13, AC-16.
- [ ] Hapus pemicu saat fokus di popup: fallback/ancestor menerima fokus sebelum subtree hilang dan upstream tidak mengembalikannya ke pemicu lama. AC-13.
- [ ] Buka modal lain di atas popup navigasi: Escape pertama ditangani lapisan teratas, dan update navigasi tidak mengambil fokus dari modal. AC-13, AC-14.
- [ ] Dua instance berbeda dengan ID leaf sama memiliki state/fokus/scroll independen. Reorder saat fokus tidak memindahkannya ke item pada indeks lama. AC-9.
- [ ] Header/footer tidak bergeser ketika list scroll. Popup scroll tetap berada di dalam viewport, termasuk pada ukuran jendela sempit. AC-11.
- [ ] Navbar dengan banyak root dan prefix/suffix pada lebar sempit serta rem 150% dapat digulir horizontal. Tab dan fokus programatis membawa leaf/pemicu paling akhir ke area terlihat, lalu aktivasi dan popup tetap bekerja. AC-7, AC-11, AC-12, AC-14.

## Showcase dan jendela native

- [ ] Story Navigation tersedia di Components dan memiliki lima pilihan pola. Activate memperbarui konten tujuan melalui handler Showcase. Empty, disabled, nested, badge, dan pembaruan data dapat diamati. AC-17.
- [ ] Ketujuh destinasi utama dan ketujuh komposisi Layouts tetap dapat digunakan. Split drag, tab selection, tema, dan shortcut shell tidak mengalami regresi. AC-16.
- [ ] Susun rail/Inline dockbar dengan konten di area content AppShell. Sidebar expanded dapat memakai slot sidebar; root shell collapsed tetap memiliki Ctrl+Tab atau alternatif aplikasi. Tidak ada perubahan diam diam pada validasi ShellConfig. AC-11, AC-13, AC-16.
- [ ] Pada platform native yang tersedia, operasikan kelima pola dengan pointer dan keyboard, termasuk Escape, outside click, pemulihan fokus, serta scroll dengan konten panjang. Catat OS, arsitektur, ukuran jendela, dan hasilnya. AC-4 sampai AC-14, AC-17, AC-18.
- [ ] Periksa light/dark x Comfortable/Compact, rem default serta 125%/150% dari nilai tersebut, dan reduced motion aktif/nonaktif. Periksa label panjang, ring fokus, badge, popup dekat tepi, dan selected/open yang tetap berbeda. AC-12.
- [ ] Inspeksi nama, role, disabled, selected/open, dan fokus melalui tree aksesibilitas native. Catat pengujian pembaca layar tersendiri; bila belum dijalankan, tulis belum diverifikasi. AC-10, AC-12, AC-13, AC-18.
- [ ] Count 120 dengan deskripsi unread messages memiliki nama `Inbox, 120 unread messages`; count nol dengan deskripsi yang sama memiliki nama `Inbox` saja. Badge teks juga mempertahankan nama tujuan. AC-10.
- [ ] Leaf pilihan: Button dengan selected true tanpa toggle palsu. Branch ancestor: selected false dengan expanded sesuai disclosure/panel, meski di-highlight. Pemicu flyout, collapse, serta heading mengikuti tabel semantik spec. Catat gap properti per platform dan jangan menandai AC terkait selesai jika bridge belum memenuhinya. AC-12, AC-14, AC-18.

## Gate implementasi

Jalankan dari root repo GPUI setelah kode dibuat. AC-1, AC-15, AC-16, AC-18.

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --locked
cargo doc --workspace --no-deps --locked
python3 scripts/validate-dependencies.py
cargo run -p showcase --locked
```

Catat hasil setiap perintah, cakupan tes yang dijalankan, dan gap platform. Tidak ada commit, push, publikasi, atau rilis yang tersirat dari kelulusan gate ini.
