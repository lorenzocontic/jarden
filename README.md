# Screenshot Archiver

A small Rust utility that organizes old screenshots into monthly folders.

Instead of keeping hundreds of screenshots in one directory, the application groups them by creation month and builds an archive plan.

---

Example output

```
Scanning Screenshots...

Destination

2026-05/
    screenshot_0512.png
    screenshot_0528.png

2026-06/
    screenshot_0602.png

2026-07/
    screenshot_0705.png

Folders to create : 3
Files processed : 5
```

---

Modules

archive.rs
Creates archive operations.

scanner.rs
Reads screenshot list.

organizer.rs
Groups files by month.

report.rs
Displays archive summary.

sample.rs
Demo data.

---

Run

```
cargo run
```
