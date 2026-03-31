# 🦀 ClapCrab

> Control your tools with the sound of your hands. Clap twice, launch anything.

ClapCrab is a lightweight, real-time audio detection tool built in Rust. It listens through your microphone and triggers actions based on sound patterns — starting with **double claps**.

## 🎯 What it does

- 🎤 Captures audio from your default microphone in real-time
- 👏 Detects double clap patterns using volume spike + duration analysis
- 🚀 Triggers custom commands on detection (e.g., open Claude Code)
- 🔇 Filters out voices and ambient noise — only reacts to sharp, short sounds

## ⚠️ Platform Support

| Platform | Status |
|----------|--------|
| 🍎 macOS | ✅ Supported |
| 🐧 Linux | 🔜 Coming soon |
| 🪟 Windows | 🔜 Coming soon |

## 📦 Installation

```bash
git clone https://github.com/Erikgavs/clapcrab.git
cd clapcrab
cargo build --release
```

## ▶️ Usage

```bash
# Run in the current directory
cargo run

# Specify a working directory for Claude Code
cargo run -- --dir ~/Documents/my-project
```

Clap twice and watch the magic happen ✨

## 🛣️ Roadmap

- [x] Real-time microphone audio capture
- [x] Single clap detection (volume spike + drop analysis)
- [x] Double clap pattern recognition with timing validation
- [x] Execute system commands on double clap (opens Claude Code in Terminal.app)
- [x] `--dir` flag to set working directory
- [ ] Custom sound patterns (triple clap, rhythms)
- [ ] Voice command detection 🗣️
- [ ] Configurable actions per pattern
- [ ] Background daemon mode

## 🧠 How it works

ClapCrab uses `cpal` for cross-platform audio capture. Each audio buffer is analyzed for volume peaks — a clap is identified as a **loud spike that drops off quickly**, which distinguishes it from sustained sounds like speech. Two claps within 100–700ms trigger the action.

## 🤝 Contributing

Contributions are welcome! Check out [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

MIT — do whatever you want with it.

---

*Built with 🦀 Rust and a lot of clapping.*
