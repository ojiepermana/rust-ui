# Scope 002: System Settings

Anda dapat mengatur preferensi aplikasi melalui Settings dengan tepat dua tujuan di sidebar, General dan Theme & Appearance. Pengaturan ini ditujukan untuk aplikasi, dengan Showcase sebagai permukaan penggunaan awal.

**Build approach:** Tracer Bullet, mengikuti pendekatan proyek dan pilihan Anda. Selesaikan satu alur membuka Settings, mengubah pilihan, lalu memulihkannya setelah aplikasi dibuka kembali sebelum memperluas kontrol.
**Workflow:** Beta, mengikuti tingkat proyek pada [scope 001](001-rust-ui-kit.md). Rekomendasi setelah implementasi adalah verifikasi perilaku aplikasi dan pengujian regresi.

## At a glance

| # | Feature | Phase | Status |
| --- | --- | --- | --- |
| 2 | System Settings dengan General dan Theme & Appearance | Settings awal | in-progress |

## Settings awal

### 2. System Settings dengan General dan Theme & Appearance · in-progress

Anda dapat membuka satu pusat preferensi dari menu aplikasi, tombol Settings, atau shortcut. Semua pilihan tampilan berada di Theme & Appearance dan berlaku pada aplikasi yang sedang digunakan.

**Done when:** Settings dapat dibuka melalui menu dan shortcut, sidebar hanya berisi General serta Theme & Appearance, preferensi bekerja dan terlihat langsung, pilihan pulih setelah aplikasi dibuka kembali, reset serta pemulihan konfigurasi bermasalah bekerja, dan fokus serta navigasi keyboard tetap dapat digunakan.

#### Akses Settings

Menu aplikasi menyediakan Settings… dengan shortcut ⌘, di macOS. Target shortcut aplikasi untuk Windows dan Linux adalah Ctrl+,. Tombol Settings menyediakan akses yang sama melalui pointer. Membuka Settings berulang kali membawa pengguna ke permukaan yang sudah terbuka tanpa menggandakan instance.

Sidebar Settings hanya memiliki General dan Theme & Appearance, dalam urutan tersebut. General menjadi tujuan saat pertama dibuka. Navigasi dan penutupan memakai perilaku native yang sesuai dengan bentuk permukaannya, dengan fokus dipulihkan ke aplikasi saat Settings ditutup. Shortcut tetap dapat membuka Settings saat fokus berada pada input atau sidebar utama disembunyikan.

#### General

Preferensi awal mencakup tujuan saat aplikasi dibuka, yaitu Overview atau melanjutkan halaman terakhir, informasi nama dan versi aplikasi, serta reset seluruh preferensi dengan konfirmasi yang menjelaskan akibatnya. Nilai bawaan membuka Overview. Reset preferensi tidak menghapus data pengguna di luar pengaturan aplikasi.

#### Theme & Appearance

Appearance menyediakan Light, Dark dan System. System mengikuti perubahan tampilan sistem operasi selama aplikasi berjalan. Nilai bawaan mengikuti System.

Theme menyediakan pilihan tema yang terdaftar dan warna aksen yang tervalidasi. Pilihan tema, mode appearance dan aksen memiliki perilaku yang jelas saat dipakai bersama, termasuk ketika sistem berganti antara Light dan Dark. Nilai tidak valid menampilkan pesan tanpa merusak tampilan aktif.

Layout menyediakan kepadatan Comfortable atau Compact, tampil atau sembunyikan sidebar aplikasi, lebar sidebar dalam batas yang didukung, serta reset layout. Perubahan berlaku pada shell aplikasi, dengan konten tetap dapat digunakan pada ukuran jendela minimum. Pilihan komposisi demo Layouts tetap merupakan demonstrasi, bukan pengaturan layout aplikasi.

Ukuran teks menyediakan skala tampilan yang terbatas dan dapat dikembalikan ke bawaan. Kontrol, label dan konten tetap terbaca ketika skala berubah. Reduce Motion mengikuti preferensi sistem dan menyediakan pilihan untuk mengurangi animasi secara eksplisit. Reset tampilan mengembalikan preferensi visual tanpa mengubah pilihan General.

#### Penyimpanan dan konsistensi

Preferensi tersimpan secara lokal dan diterapkan saat aplikasi berikutnya dibuka. Perubahan visual terlihat langsung tanpa memulai ulang aplikasi. Nilai yang hilang atau rusak dipulihkan ke bawaan yang aman, sementara kegagalan menyimpan dijelaskan kepada pengguna. Berpindah tema tidak diam diam menghapus pilihan layout, ukuran teks atau Reduce Motion.

Kontrol tampilan lain yang sudah ada tetap mencerminkan preferensi aktif yang sama. Galeri Themes dan Playground tetap dapat dipakai untuk demonstrasi, dengan aturan yang jelas tentang perubahan yang memengaruhi preferensi tersimpan. Preferensi tambahan hanya ditambahkan ke salah satu dari dua halaman jika dibutuhkan, tanpa menambah tujuan sidebar pada tahap ini.

#### Keputusan desain

Desain dikonfirmasi pada 17 November 2026, termasuk keenam perbaikan dari pemeriksaan silang. Spec tertaut di bawah merinci permukaan Settings, penggunaan ulang, kepemilikan state, penyimpanan, pasangan tema dan skala. Implementasi belum dimulai.

Verifikasi native pada platform yang tersedia dan pemeriksaan aksesibilitas dicatat terpisah dari tes headless. Bukti mencakup menu dan shortcut, perpindahan fokus, perubahan appearance sistem, penerapan visual, serta pemulihan preferensi setelah aplikasi dibuka kembali. Scope ini belum menyatakan pemeriksaan tersebut lulus.

- [x] Design it (spec): `/architect System Settings`
- [ ] Build it: `/develop System Settings`
  - [ ] Alur awal jendela, shortcut, General dan penyimpanan preferensi (AC-1, AC-2, AC-3, AC-10, AC-11, AC-13)
  - [ ] Pasangan tema, aksen dan sinkronisasi kontrol global (AC-4, AC-5, AC-10, AC-12)
  - [ ] Layout, skala, reset dan adapter Reduce Motion (AC-2, AC-6, AC-7, AC-8, AC-9, AC-12)
  - [ ] Recovery, retry, flush, lifecycle jendela dan consumer kedua (AC-1, AC-10, AC-11, AC-12, AC-13)
  - [ ] Validasi workspace dan bukti native sesuai ledger (AC-1, AC-2, AC-3, AC-4, AC-5, AC-6, AC-7, AC-8, AC-9, AC-10, AC-11, AC-12, AC-13, AC-14)
- [ ] Verify it: `/check verify System Settings`
- [ ] Test it: `/test System Settings`

Spec [0003](../../specs/_root/0003-system-settings/index.md).

## Di luar tahap ini

Akun, sinkronisasi cloud, notifikasi, shortcut yang dapat diubah pengguna, marketplace tema, editor seluruh token tema, docking lengkap dan halaman sidebar tambahan dapat dipertimbangkan pada scope berikutnya.

## Legend

`in-progress` menandai pekerjaan fitur yang sedang berjalan, termasuk desain yang sudah siap dibangun. Kotak pertama yang belum dicentang menunjukkan langkah berikutnya. Penerimaan fitur tetap memerlukan implementasi dan bukti verifikasi. Scope ini tidak menutup verifikasi navigasi pada scope 001.
