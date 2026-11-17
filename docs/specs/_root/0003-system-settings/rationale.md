# System Settings, alasan keputusan

## Context

Scope 002 meminta pusat preferensi native dengan dua halaman dan perubahan tampilan yang bertahan setelah aplikasi dibuka kembali. Showcase sudah memiliki pilihan appearance, tema khusus, density dan komposisi layout. State tersebut masih berada pada beberapa jalur aplikasi, tanpa penyimpanan preferensi bersama.

Repo adalah workspace Rust native dengan tujuh library lokal sebelum penambahan ini. Aturan ownership membedakan konfigurasi tema, komposisi layout, navigasi, facade dan state aplikasi. Fitur ini perlu menyatukan pengalaman pengaturan tanpa menghilangkan batas tersebut.

## Options considered

| Pilihan | Manfaat | Biaya |
| --- | --- | --- |
| Seluruh Settings dalam Showcase | Jumlah boundary baru paling sedikit dan iterasi awal cepat | Consumer lain harus memisahkan ulang UI dan state aplikasi. Tidak memenuhi pilihan penggunaan ulang pengguna. (basis: AGENTS.md, scope 002.) |
| `ui-settings` sebagai komposisi terkendali, aplikasi menyimpan JSON | Boundary publik jelas, lifecycle dapat diuji dan penyimpanan tetap bebas untuk consumer | Memerlukan kontrak event, sinkronisasi snapshot dan controller aplikasi. Ini pilihan yang direkomendasikan. (basis: Coding Guides pinned, pola `ui-navigation` pada spec 0002.) |
| Settings sekaligus storage dalam `ui-layout` | Lebih sedikit crate dan satu tempat untuk shell serta preferensinya | Memasukkan state dan I/O ke komposisi stateless, memperluas biaya setiap consumer layout. (basis: docs/architecture/README.md.) |
| Preferensi native tiap OS | Mengikuti layanan konfigurasi masing masing desktop | Perilaku schema, recovery dan pengujian berbeda pada tiga platform. Pengguna memilih JSON dengan dependency yang sudah tersedia. (basis: Cargo.toml, keputusan percakapan.) |

Jendela terpisah dipilih pengguna agar aplikasi utama tetap dapat digunakan. Alternatif halaman pada shell memerlukan navigasi kembali; alternatif modal menahan tugas utama saat memilih tampilan. Pasangan tema dipilih agar System tidak menghilangkan warna khas tema. Alternatif tema dengan mode tetap lebih mudah bagi registry saat ini tetapi tidak memberi perilaku System yang diinginkan.

## Rationale

`ui-settings` memiliki capability dan lifecycle sendiri, sementara aplikasi tetap memiliki efek ke file serta perintah jendela. Bentuk ini mengikuti pola event navigasi yang sudah ada dan arahan upstream untuk state retained (state yang hidup antar frame), sehingga UI tidak menyimpan salinan preferensi tersembunyi. Resolver `ui-theme` menerima override lengkap agar hasilnya selalu berasal dari definition asli. (basis: AGENTS.md, Coding Guides pinned, spec 0002.)

JSON dipilih karena data kecil, milik satu aplikasi dan tidak membutuhkan query atau layanan luar. Satu writer dan revision mencegah hasil save lama menghapus perubahan baru. Penulisan snapshot melalui file sementara serta penggantian pada direktori sama memisahkan tampilan aktif dari hasil penyimpanan. Lock proses membatasi kebijakan lintas proses tanpa menambahkan IPC atau database. (basis: Cargo.toml, snapshot persistence dan serialisasi writer.)

Ukuran font dasar menjadi kontrol skala karena Root upstream menulis rem dari `theme.font_size` pada prepare. Mengatur rem sendiri pada view akan ditimpa Root dan dapat menghasilkan ukuran berbeda antar jendela. (basis: Coding Guides pinned, `gpui-component` 0.9.1 `src/root.rs`, `WindowState::prepare`.)

Patch motion berada pada adapter pemilik pembacaan preferensi sistem. Menambahkan adapter platform kedua pada `ui-theme` akan menggandakan pembacaan dan ownership flag. Memperbaiki state adapter pada versi sama menjaga satu jalur sistem dan renderer yang dipin. Pengguna memilih pendekatan ini secara eksplisit. (basis: `vendor/gpui-base/src/reduce_motion.rs`, `vendor/gpui-base/PATCHES.md`, docs/architecture/compatibility.md.)

## Bukti source yang diperiksa

Pemeriksaan pada 17 November 2026 tidak menjalankan ulang build atau UI. Checkout utama berada pada `main`; ref lokal `origin/main` tidak memiliki commit tambahan terhadap HEAD ketika preflight dilakukan. Tidak ada fetch karena kebijakan repo `integration: off`; hasil ini bukan klaim keadaan remote terkini. Perubahan tes navigasi dari pekerjaan lain ditemukan dan tidak menjadi bagian desain ini.

| Source | Temuan yang memengaruhi spec |
| --- | --- |
| `apps/showcase/src/main.rs` | Bootstrap hanya membuka jendela utama dan keluar ketika seluruh jendela ditutup. Perlu policy pemilik Settings. |
| `apps/showcase/src/lib.rs` | Tujuh tujuan utama, appearance toolbar, tema khusus dan density; subscriptions serta fokus sudah retained. Semua aksi global perlu dihubungkan ke controller bersama. |
| `crates/ui-theme/src/lib.rs` | `select` memilih default light/dark; `select_named` mengunci mode; apply mereset density dan mengatur motion dari definition. Jalur baru harus menerapkan override sesudah definition tanpa mengubah API legacy. |
| `crates/ui-layout/src/lib.rs` | `ShellConfig` mendukung sidebar 160 sampai 400 logical pixel, default 224, dan validasi lebar konten. |
| `crates/ui-navigation/src/state.rs` | Entity retained dan event permintaan terpisah dari keputusan aplikasi. Reuse untuk sidebar Settings. |
| GPUI 0.3.12 `App::on_action`, `App::set_menus`, MenuItem dan observer window | Action global, menu, aktivasi window, appearance observer dan activation observer tersedia pada source terpasang. |
| GPUI Component 0.9.1 `WindowState::prepare` | Root mengatur rem dari ukuran font dasar tema. |
| `vendor/gpui-base/src/reduce_motion.rs` | Adapter melacak nilai yang terakhir diterapkan dan ownership melalui kesamaan flag; tidak ada API eksplisit untuk override reversible. macOS/Windows tidak memiliki watcher live dalam adapter ini. |
| `/tmp/rust-ui-upstream-091` | HEAD cocok dengan release commit `39ab849a93b5f15507b969164d49891db8d5b470`; coding/design guide dan source dipakai sebagai bukti lokal, bukan asumsi API versi terbaru. Path ini fixture pemeriksaan lokal, bukan dependency build. |

## Pemeriksaan silang

Pada 17 November 2026, model `gpt-6-astra` membaca spec, rationale, ledger dan scope tanpa membaca implementasi atau mengambil referensi web. Pemeriksaan menemukan enam gap keputusan. Pengguna memilih menerapkan seluruh rekomendasi sebelum konfirmasi akhir spec. Penulis spec menerapkan perubahan dan memeriksa konsistensi dokumen; ini bukan hasil verifikasi implementasi.

| Gap | Perbaikan yang disetujui |
| --- | --- |
| Tipe appearance berpotensi membutuhkan dependency balik | `AppearancePreferences` dan tipe pendukung dimiliki `ui-theme`; `SettingsValues` mengomposisikannya, dengan ekspor kembali untuk consumer |
| File kebesaran dapat memuat schema masa depan yang belum diketahui | StorageProtected tidak menulis file asal, perubahan hanya untuk sesi, dan retry hanya mengklasifikasikan ulang sebelum rekonsiliasi eksplisit |
| Label, transaksi dan penghapusan registrasi keluarga khusus belum lengkap | Label `Custom N`, transaksi create/apply, serta reset katalog aktif dan tersimpan yang melepas definition keluarga khusus |
| Quit dapat menyelesaikan flush revision lama saat revision baru masuk | Perbandingan current/persisted revision di controller pada keputusan keluar, memakai tenggat absolut yang sama |
| Override aksen tidak mempunyai tindakan penghapusan khusus | Gunakan aksen tema mengirim `Accent(None)`; draft input kosong tidak mengubah state tersimpan |
| Reset layout dan density tidak sesuai pengelompokan scope | Reset layout mengembalikan Comfortable dan sidebar; Reset appearance mempertahankan density |

Rencana verifikasi menambahkan kasus untuk setiap gap tersebut. Pada 17 November 2026, pengguna menerima isi spec yang telah diperbaiki dan meminta pembaruan scope serta milestone tanpa memulai implementasi. Persetujuan ini mengesahkan desain; status spec tetap Proposed karena fitur belum dibangun atau diverifikasi.

## References

### Project sources

`AGENTS.md`; `docs/scope/_root/002-system-settings.md`; `Cargo.toml`; `docs/architecture/README.md`; `docs/architecture/compatibility.md`; spec 0002; source Showcase, theme, layout, navigation dan vendor yang disebut pada tabel.

### Practices and standards

Coding Guides dan Design Guides GPUI Kit pada release commit yang dipin; komposisi terkendali dengan satu owner state; snapshot persistence dan serialisasi writer; perhitungan rasio kontras dari luminansi sRGB relatif untuk teks aksen. Tidak ada riset web atau klaim verifikasi link dalam keputusan ini.
