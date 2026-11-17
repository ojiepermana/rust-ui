# System Settings, rencana verifikasi

**Date**: 2026-11-17

Belum ada implementasi atau hasil verifikasi fitur 0003. Semua baris berikut adalah bukti yang direncanakan. Status lifecycle berada hanya pada index spec.

## Otomatis

| Pemeriksaan | Bukti yang diperlukan | AC |
| --- | --- | --- |
| Model dan resolver | Nilai bawaan, validasi, pasangan atomik, Gunakan aksen tema tanpa mengubah pilihan lain, kontras normal/hover/active, mode System, skala tanpa pengalian berulang, Reset appearance mempertahankan Compact dan Reset layout mengembalikan Comfortable | AC-3, AC-4, AC-5, AC-6, AC-7, AC-9 |
| Katalog tema | Label `Custom N`, transaksi create/apply gagal tanpa registrasi parsial, buat/reset/buat lagi dalam proses yang sama, definition khusus dilepas dari registry dan dokumen tetapi definition standalone consumer tetap | AC-5, AC-9, AC-10, AC-12 |
| Store dengan direktori sementara | Round trip lengkap dengan tema khusus, recovery field schema 1, JSON rusak, versi masa depan sekaligus file besar, byte asli tetap setelah edit/reset sesi Protected, retry klasifikasi tanpa write, backup, kegagalan I/O, lock proses kedua, save revision lama dan retry terbaru | AC-10, AC-11 |
| Quit dengan writer tertahan | Revision 8 disimpan sementara jendela kedua membuat revision 9; aplikasi belum keluar sebelum revision 9 tersimpan atau dialog tenggat ditampilkan; permintaan Quit berulang tidak memperpanjang tenggat | AC-11, AC-12 |
| UI produksi headless | Dua halaman, menu/action dispatch, event controlled, snapshot sync tanpa loop, keyboard/fokus, draft aksen, reset/Cancel, minimum window dan skala/density matrix, dua jendela sinkron | AC-1, AC-2, AC-3, AC-6, AC-7, AC-9, AC-12 |
| Adapter motion produksi | System false/true, Reduce, perubahan sistem selama override, kembali System, tidak ada pembacaan OS pada scheduler deterministik, kompatibilitas setter lama | AC-8 |
| Consumer kedua | `examples/settings-app` mengonsumsi facade tanpa source Showcase, `AppearancePreferences` berasal dari `ui-theme`, tidak ada dependency balik, retained subscriptions dan perubahan snapshot dapat diamati | AC-13 |
| Workspace | Seluruh perintah pada bagian di bawah, source integrity termasuk patch baru dan bridge native macOS | AC-14 |

Perintah root setelah implementasi:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build --workspace --locked
cargo doc --workspace --no-deps --locked
python3 scripts/validate-dependencies.py
cargo test -p accesskit_macos --test native_bridge --locked
```

Regresi adapter yang baru diberi nama `settings_motion_override` dan dijalankan melalui manifest vendor karena package dikecualikan dari workspace:

```sh
cargo test --manifest-path vendor/gpui-base/Cargo.toml --locked --features test-support --lib settings_motion_override
```

Pertahankan regression divider dan selection yang sudah ada. Pengujian patch baru mencakup negative control pada adapter produksi asli, dengan source regresi yang sama, sebelum source integrity dianggap selesai. Jangan memodifikasi checkout vendor aktif untuk negative control; gunakan fixture terpisah.

## Native per platform

Jalankan `cargo run -p showcase --locked` pada desktop yang tersedia. Catat OS, arsitektur, build yang diamati dan kondisi preferensi sebelum tes. Gunakan folder konfigurasi atau pengguna OS khusus untuk pemeriksaan yang perlu recovery, dengan nilai awal pengguna dipulihkan sesudah pengujian.

| Skenario | macOS | Windows | Linux Wayland / X11 |
| --- | --- | --- | --- |
| Menu menunjukkan shortcut, tombol dan shortcut dari input membuka jendela yang sama | Belum diuji | Belum diuji | Belum diuji |
| Perpindahan dua halaman, Tab, overlay Escape, close native/shortcut dan pemulihan fokus | Belum diuji | Belum diuji | Belum diuji |
| Appearance OS berubah ketika mode System, tema dan aksen tetap, dua jendela ikut berubah | Belum diuji | Belum diuji | Belum diuji |
| Density, sidebar dan skala 150% pada minimum window tetap dapat dipakai | Belum diuji | Belum diuji | Belum diuji |
| Motion System/Reduce/System, termasuk pembacaan ulang saat aplikasi aktif kembali | Belum diuji | Belum diuji | Belum diuji |
| Tutup proses dan buka ulang, preferensi serta tema khusus pulih sebelum jendela ditampilkan | Belum diuji | Belum diuji | Belum diuji |
| Gunakan aksen tema, create/reset/create tema khusus, Reset appearance saat Compact dan Reset layout saat Compact | Belum diuji | Belum diuji | Belum diuji |
| Save gagal, Retry dan keputusan close tidak menghilangkan perubahan tanpa pemberitahuan | Belum diuji | Belum diuji | Belum diuji |

## Aksesibilitas

Catat tree/metadata native dan output pembaca layar sebagai bukti berbeda. Periksa nama kontrol, selected pada dua item sidebar, fokus terlihat, urutan keyboard, error dekat input, status save yang dapat dipahami, dan teks/geometry pada skala maksimum. Regresi ABI AccessKit tidak menggantikan penggunaan VoiceOver, Narrator atau pembaca layar Linux. Tidak ada acceptance yang ditandai lulus dari hasil spec ini.
