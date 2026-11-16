# Rationale: navigasi native

## Context

Workspace GPUI sudah memiliki shell dan komposisi layout, tetapi navigasi yang didemonstrasikan masih berupa Button milik Showcase dan helper region `ui-layout`. Belum ada satu kontrak item, controlled event, serta koordinasi fokus yang dapat dipakai aplikasi untuk lima pola navigasi.

Pengguna memperjelas target pada 16 November 2026: crate dibangun menggunakan GPUI Kit dalam repo ini. Library Angular adalah contoh saja dan tidak digunakan di repo GPUI. Pengguna memilih sidebar, dockbar, minidock, navbar, dan flyout; state, aksi, serta penyimpanan tetap milik aplikasi.

Kode yang diperiksa adalah GPUI Kit 0.9.1 dan komponen dependensinya pada registry lokal, serta checkout release commit `39ab849a93b5f15507b969164d49891db8d5b470`. Kontrak public API dan state mengikuti sumber yang dipin tersebut. Tidak ada alasan untuk memilih runtime atau renderer lain.

## Options considered

### Opsi 1: memperluas `ui-layout` langsung

Kelebihannya tidak menambah crate atau ekspor baru. Kekurangannya model item, event interaksi, serta retained focus lifecycle akan bercampur dengan komposisi stateless shell/split yang sudah memiliki batas kepemilikan jelas.

### Opsi 2: crate `ui-navigation` dengan komposisi upstream

Kelebihannya ada satu batas untuk model navigasi dan event, dengan consumer melalui facade yang sudah dipakai. Kekurangannya ada crate, metadata, validator, serta dokumentasi ownership yang perlu ditambahkan dan dipelihara. Ini pilihan yang direkomendasikan.

### Opsi 3: meniru API dan renderer Angular secara menyeluruh

Kelebihannya nama dan screenshot awal mudah dicocokkan dengan contoh web. Kekurangannya route service, DOM/CSS, hover policy, dan lifecycle web tidak cocok dengan kontrak native dan tidak diminta pengguna. Menyalin komponen juga bertentangan dengan ownership GPUI Kit.

### Opsi 4: hanya mengekspos Sidebar dan Popover upstream

Kelebihannya paling kecil dan seluruh API upstream langsung tersedia. Kekurangannya kelima pola beserta validasi model dan kebijakan identitas/fokus tetap harus disusun ulang setiap aplikasi. Facade memang sudah mengekspor komponen tersebut, sehingga ini tidak menutup kebutuhan baru.

## Rationale

Navigasi kini merupakan kapabilitas yang memiliki model, lifecycle, dan seam publik sendiri. Crate terpisah memberi batas yang bisa diuji tanpa membawa state domain Showcase ke library. GPUI Kit tetap menjadi pemilik control behavior dan rendering, sedangkan crate baru menyusun pola dan kontrak intent aplikasi.

`NavigationState` per instance mengikuti pola retained entity yang dipakai GPUI. Entity dibuat serta subscription disimpan aplikasi. Perubahan nilai hanya melalui setter aplikasi; callback kontrol tidak memutuskan tujuan aktif. Ini berbeda dari service global yang mendaftarkan dan menyimpan preferensi seluruh navigasi.

Observer invalidation ditulis eksplisit karena elemen RenderOnce tidak boleh diasumsikan merender ulang parent hanya akibat membaca entity. Aplikasi menyimpan observer dan subscription event sebagai dua lifetime yang berbeda. Konfigurasi ekuivalen tidak memberi notifikasi, sementara input yang ditolak dapat menjadwalkan satu refresh untuk menyinkronkan mekanisme popup upstream.

Pemakaian Button dan Collapsible untuk branch dipilih karena `SidebarMenuItem::default_open` upstream bukan input controlled. Menyalin implementasi submenu untuk mengganti ownership akan menciptakan komponen kedua. Popover justru memiliki `open` dan `on_open_change` yang dapat dipakai bersama, dengan focus capture/restoration upstream. State mekanis internal tetap harus disinkronkan dengan snapshot aplikasi dan diuji terhadap event ganda.

Pergantian popup memerlukan rekonsiliasi intent pada batas input: klik pemicu B juga dapat menutup A sebagai outside click. Batch memilih permintaan open tujuan dan mengabaikan close dari panel lama, tanpa membuat dua perubahan domain. Pemulihan fokus mencakup slot serta popup, dan pemilik shell melakukan handoff sebelum region disembunyikan oleh breakpoint. Style selected dipisahkan dari metadata selected/expanded native agar highlight ancestor tidak dibaca sebagai pilihan tujuan kedua.

Model flat minidock dipilih agar rail tidak kehilangan anak tanpa pemberitahuan. Flat model dibuat aplikasi dari domain yang sama; ID leaf menjaga pemetaan selected tetap stabil. Penyaring tree generik dapat dipertimbangkan bila consumer nyata membutuhkan kebijakan perataan yang sama, tanpa memaksakan seluruh aturan Angular sejak awal.

Dockbar memakai istilah `Inline` dan `Overlay` untuk menjelaskan hasil native. Tidak ada route restore saat pointer leave, tidak ada auto open dari selected leaf, dan tidak ada `DockArea` atau resize engine baru. Aplikasi boleh menerapkan kebijakan tersebut di atas event, tetapi bukan default tersembunyi crate.

## Peta acuan Angular ke kontrak Rust

| Contoh Angular | Keputusan native |
| --- | --- |
| Container dan instance id | Entity `NavigationState` milik aplikasi dan ID instance eksplisit |
| Sidebar dan collapsible | Sidebar Button/Collapsible, selected serta expanded supplied oleh aplikasi |
| Dockbar rail plus aside | Rail dan panel Inline/Overlay, tanpa routing dan hover restore |
| Minidock permanen | Model flat, rail tidak dapat expand, tanpa menyalin helper filtering web |
| Navbar | Bar Button dan popup branch native, bukan menubar web dengan kontrak keyboard setengah jadi |
| Flyout | Satu trigger dengan Popover native dan list/group; grid CSS dan posisi menutup trigger tidak diwajibkan |
| Header, content, footer | Slot navigasi dengan scroll item terpisah; berbeda dari header/footer shell aplikasi |
| `activeIds`, `openedIds`, `itemSelected` | Snapshot presentasi dan event ID yang diputuskan aplikasi |
| `badge-context` dan total parent | Nilai badge siap tampil dari aplikasi, tanpa registry reaktif atau perhitungan bisnis |
| Route context dan Angular Router | Tidak dibawa; aplikasi memetakan ID Activate ke tujuan native |
| Persistence dan NavigationService | Tidak dibawa; state instance di entity aplikasi dan persistence aplikasi |
| CSS, sumbu tema, ikon web | Semantic theme GPUI, snapshot `ui-theme`, aset `ui-icons`, serta control size upstream |

## Batas layout yang sudah terukur dari source

`AppShell` sudah menerima header, sidebar, toolbar, inspector, dan footer. Sidebar disembunyikan di bawah ambang 800 logical pixel atau saat collapsed eksplisit. Inspector muncul bila lebar mencapai ambang ditambah inspector width. `ShellConfig` menolak sidebar width di luar 160 sampai 400 logical pixel.

Kontrak tersebut tidak otomatis memuat rail 3 rem atau rail plus panel dockbar. Contoh consumer menyusun navigasi jenis ini bersama konten dalam region content shell. Mengubah rentang ShellConfig atau menambah docking preset akan menjadi keputusan lain, sehingga spec ini tidak menyamarkannya sebagai perubahan navigasi kecil.

`sidebar_navigation`, `navigation_group`, `header`, `content_container`, `dashboard`, SplitWorkspace, dan TabbedWorkspace tetap dimiliki `ui-layout`. Galeri Layouts yang sebenarnya berada di `apps/showcase/src/lib.rs`; `catalog.rs` mendata cerita komponen. Galeri Navigation ditambahkan sebagai cerita komponen tanpa menghapus tujuh komposisi layout tersebut.

## Verifikasi dan klaim

Isi desain dikonfirmasi pengguna pada 16 November 2026. Pemeriksaan silang GPT-6 Astra menemukan enam celah keputusan dan dua klarifikasi; seluruh rekomendasi disetujui pengguna, diterapkan pada spec, dan diperiksa kembali tanpa kontradiksi material tersisa dari perbaikannya. Pemeriksaan ini hanya atas dokumen, bukan implementasi.

Spec ini berasal dari pembacaan source dan keputusan pengguna, belum dari implementasi baru. Tes pure membuktikan validasi/proyeksi, tes GPUI membuktikan interaksi dan fokus sintetis, dan jendela native membuktikan perilaku pada platform yang diamati. Pembaca layar memerlukan bukti sendiri. Keberadaan aksesibilitas dalam tree tidak membuktikan urutan pembacaan VoiceOver atau teknologi bantu lain.

Tidak ada seksi References tersendiri sesuai pilihan pengguna. Penalaran dan batas sumber yang dipin tetap dicatat di atas.

## Amendemen bridge macOS, 16 November 2026

Bukti awal menunjukkan selected hilang pada Button karena adapter membatasi getter pada role item. Expanded belum dipetakan dari properti opsional AccessKit. Regresi yang memanggil selector Objective-C pada node platform asli gagal untuk selected, expanded, dan notifikasi state, sementara perilaku item pilihan dan Tab lama tetap lulus.

Pengguna kemudian secara eksplisit mengizinkan perbaikan bridge tanpa upgrade. Opsi memperbaiki adapter pada versi yang sama dipilih karena pemilik masalah jelas dan perubahan produksinya terbatas pada dua file. Upgrade bertentangan dengan permintaan pengguna. Mengganti renderer atau memalsukan role menjadi menu/toggle juga bertentangan dengan kontrak komponen. Patch lokal versi 0.26.3 mempertahankan API/runtime, menyediakan bukti sebelum/sesudah serta provenance, dan dapat dilepas setelah upstream setara diverifikasi. Tidak ada perubahan model domain atau storage aplikasi.

Getter selected baru hanya memperluas Button/DefaultButton yang benar benar memiliki nilai selected, tanpa menambah setter pemilihan atau AXPick. Getter expanded membaca nilai opsional dan tidak mengarang atribut bagi Button biasa. Jalur event yang sama menggabungkan notifikasi state agar pembaruan koheren tidak membuat duplikasi. Pemeriksaan native dan batas alat dicatat terpisah dari pembacaan teknologi bantu. Pembaruan ini dikerjakan inline melalui architect sesuai permintaan eksplisit dan kebijakan keputusan auto; bukan cross model review.
