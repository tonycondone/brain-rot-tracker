# Brain Rot Tracker

## Overview

**"Rescue your attention span."**

Brain Rot Tracker automatically measures how much low-value digital content you consume, translates it into a single "Rot Score", and nudges you toward healthier habits with real-time feedback, streaks, and rewards.

## Features

### Core Features (MUST)
- ✅ Auto-detect usage of configurable "junk" apps/sites (TikTok, Shorts, Reels, Twitter, Twitch)
- ✅ Convert usage minutes → Rot Points via YAML rules file
- ✅ Daily, weekly, monthly dashboards (web)
- ✅ Local data only; GDPR-friendly (no 3rd-party servers)
- ✅ Cross-platform: Windows, macOS, Linux desktop support

### Enhanced Features (SHOULD)
- 🔄 Manual "offline" activity logger
- 🔄 Negative rot activities: reading, Duolingo, exercise
- 🔄 Gamification: streaks, badges, "Touch Grass" notifications
- 🔄 Export to CSV, JSON

## Architecture

```
┌────────────────────────┐
│   Desktop App (Tauri)  │  Cross-platform
└────┬──────────┬────────┘
     │ REST/WS  │
┌─────────────┘  └─────────────┐
▼                              ▼
┌────────────────┐          ┌────────────────┐
│  Usage Daemon  │          │  Local API     │
│  (Rust)        │          │  (Axum/Rust)   │
└────┬───────────┘          └────┬───────────┘
     │ SQLite (local)            │
     ▼                           ▼
┌────────────────┐          ┌────────────────┐
│ Rule Engine    │          │ Web Dashboard  │
│ (YAML)         │          │ (React/Vite)   │
└────────────────┘          └────────────────┘
```

## Tech Stack

- **Desktop**: Tauri (Rust + React)
- **Backend**: Axum (Rust) with local SQLite
- **Frontend**: React + TypeScript + Tailwind CSS
- **Database**: SQLite with SQLCipher encryption
- **Configuration**: YAML-based rule engine

## Quick Start

### Prerequisites

- Rust 1.70+
- Node.js 18+
- Platform-specific dependencies (see installation guide)

### Installation

1. Clone the repository
2. Install dependencies:
   ```bash
   npm run setup
   ```
3. Start development:
   ```bash
   npm run dev
   ```

### Building for Production

```bash
npm run build
```

## Project Structure

```
/
├─ daemon/           # Rust usage tracking daemon
├─ app/              # Tauri desktop application
├─ dashboard/        # React web dashboard
├─ docs/             # Documentation
├─ scripts/          # Build and deployment scripts
├─ assets/           # Icons, images, resources
└─ rules/            # Default YAML rule configurations
```

## Privacy & Security

- 🔒 **Local-first**: All data stays on your device
- 🔒 **Encrypted storage**: SQLCipher for database encryption
- 🔒 **No tracking**: Zero telemetry or analytics
- 🔒 **Sandboxed**: Runs without admin/root privileges
- 🔒 **GDPR compliant**: No data leaves your machine

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for development setup and guidelines.

## License

MIT License - see [LICENSE](./LICENSE) for details.