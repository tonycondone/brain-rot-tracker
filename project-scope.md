Below is a complete, end-to-end “v1.0” plan that takes Tony’s half-joke repo and turns it into a shippable, production-grade product.
You can start from the existing CLI skeleton or treat this as a green-field rebuild—either way, every deliverable is specified so you can hand the document to a junior dev and have them crank out an MVP in ~2 weeks.
────────────────────────────────────────
Product Vision
────────────────────────────────────────
“Rescue your attention span.”
brain-rot-tracker automatically measures how much low-value digital content you consume, translates it into a single “Rot Score”, and nudges you toward healthier habits with real-time feedback, streaks, and rewards.
────────────────────────────────────────
2. Functional Requirements (MoSCoW)
────────────────────────────────────────
MUST
Auto-detect usage of configurable “junk” apps/sites (TikTok, Shorts, Reels, Twitter, Twitch).
Convert usage minutes → Rot Points via a YAML rules file (so non-devs can tweak).
Daily, weekly, monthly dashboards (web).
Local data only; GDPR-friendly (no 3rd-party servers).
Cross-platform: Windows, macOS, Linux, Android, iOS (React-Native shell).
SHOULD
Manual “offline” activity logger (e.g., doom-scrolling on a friend’s phone).
Negative rot activities: reading, Duolingo, exercise.
Gamification: streaks, badges, “Touch Grass” push at 90 min of continuous rot.
Export to CSV, JSON, Apple Health / Google Fit.
COULD
On-device ML that auto-classifies new apps/sites as junk or healthy.
Focus mode: block junk apps once daily rot budget is exceeded.
Social league tables (opt-in, end-to-end encrypted).
WON’T
Content analysis (we don’t read/watch what you consume).
Cloud sync (keeps scope small).
────────────────────────────────────────
3. Architecture
────────────────────────────────────────
┌────────────────────────┐
│   React-Native App     │  (iOS/Android)
│  (Electron wrapper     │  (desktop)
└────┬──────────┬────────┘
│ REST/Web │Socket
┌─────────────┘          └─────────────┐
▼                                      ▼
┌────────────────┐                  ┌────────────────┐
│  Daemon/Agent  │  OS-level usage  │  Local API     │
│  (Rust)        │  + window title  │  (Axum/Rust)   │
└────┬───────────┘  + idle time     └────┬───────────┘
│ SQLite (local)                    │
▼                                   ▼
┌────────────────┐                  ┌────────────────┐
│ Rule Engine    │                  │ Web Dashboard  │
│ (YAML)         │                  │ (Next.js)      │
└────────────────┘                  └────────────────┘
Desktop: Tauri (Rust) or Electron (if you want to reuse web stack).
Mobile: React-Native + JSI modules for the Rust daemon via UniFFI.
Data stays in a single SQLite file (encrypted with SQLCipher).
────────────────────────────────────────
4. Data Model
────────────────────────────────────────
Table: events
id | timestamp | app_or_site | window_title | duration_sec | rot_points
Table: rules
app_or_site_pattern (glob) | multiplier | category (junk|healthy)
Table: badges
badge_id | unlock_condition | unlocked_at
────────────────────────────────────────
5. API Surface (Auto-generated OpenAPI)
────────────────────────────────────────
GET    /rot/daily?tz=...              → {total, breakdown[]}
POST   /rot/manual                    → {activity, duration, category}
GET    /rules                         → list current YAML rules
PUT    /rules                         → overwrite rules
GET    /badges                        → list badges & progress
POST   /focus-mode                    → enable/disable blocking
GET    /export?format=csv|json        → download
────────────────────────────────────────
6. Rule Engine (YAML Snippet)
────────────────────────────────────────
pattern: "TikTok*"
multiplier: 12
category: junk
pattern: "Kindle*"
multiplier: -3
category: healthy
pattern: "YouTube*"
multiplier: 4
category: junk
unless_title_contains:
"MIT OpenCourseWare"
"Yoga with Adriene"
────────────────────────────────────────
7. UI/UX Wireframes (Figma links in /docs)
────────────────────────────────────────
Desktop Tray App
Micro-window: live rot meter (ring) + streak counter.
Click → full dashboard (Next.js served locally).
Mobile Home Screen Widget
Rot ring + “Touch Grass” button (starts 10-min timer that subtracts 30 rot).
Dashboard Pages
Today: timeline heat-map, biggest offenders pie.
Trends: 7-day rolling average.
Rules: table editor with live preview.
Achievements: grid of locked/unlocked badges.
────────────────────────────────────────
8. Implementation Sprints (2-week MVP)
────────────────────────────────────────
Week 1
Day 1–2: Rust daemon (macOS & Windows usage stats via accessibility API / Win32).
Day 3: SQLite schema + rule engine.
Day 4: Local Axum API + integration tests (RSpec-style).
Day 5: React-Native shell + WebSocket to daemon.
Week 2
Day 6–7: Dashboard (Next.js) served from Tauri.
Day 8: Gamification layer (badges & streaks).
Day 9: Manual activity logger + CSV export.
Day 10: Polish & cross-platform builds (GitHub Actions).
Day 11–12: Internal dog-food & bug bash.
Day 13: Release v1.0 (GitHub Release + Homebrew cask + .deb + .apk).
────────────────────────────────────────
9. DevOps & Release
────────────────────────────────────────
GitHub Actions matrix build → artifacts attached to release.
auto-update via TauriUpdater & Expo OTA for mobile.
Code-signing:
Apple Developer cert (macOS/iOS)
Microsoft EV cert (Windows)
Play Store key (Android)
────────────────────────────────────────
10. Privacy & Security Checklist
────────────────────────────────────────
☐ No network calls except update check (user opt-in).
☐ SQLCipher passphrase derived from OS keychain.
☐ Sandboxed desktop app (no root/admin).
☐ App Store privacy nutrition labels auto-generated from manifest.
────────────────────────────────────────
11. Stretch Roadmap (post-MVP)
────────────────────────────────────────
Q2 2026
Cloud-less “family plan”: local p2p sync via Wi-Fi Direct.
VisionOS & watchOS complications.
Machine-learning auto-rule suggestion (CoreML / TensorFlow Lite on-device).
────────────────────────────────────────
12. One-Pager Marketing Site
────────────────────────────────────────
Landing page (Next.js static):
Hero GIF: rot meter turning red → user gets “Touch Grass” push → goes outside → meter turns green.
Testimonials from beta users.
Download buttons auto-detecting OS.
────────────────────────────────────────
13. Repo Structure (monorepo)
────────────────────────────────────────
/
├─ daemon/           # Rust (workspace)
├─ app/              # React-Native + Tauri
├─ dashboard/        # Next.js
├─ docs/             # User & dev docs (Docusaurus)
├─ scripts/          # CI, release
└─ assets/           # icons, screenshots
────────────────────────────────────────
14. Final Hand-off Package
────────────────────────────────────────
README.md with 30-second setup gif.
CONTRIBUTING.md with just dev one-command boot.
MIT license.
.env.example (empty—everything local).
Figma link + exported PNGs in /docs/design.