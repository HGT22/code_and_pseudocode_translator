# Release Checklist (Quick)

## 1) Prepare local repo
- git status
- git pull
- cargo test (if Rust is available)
- Confirm README download links are up to date

## 2) Build binaries/assets
Recommended asset names:
- code-translator-windows-x64.exe
- code-translator-macos-arm64.dmg
- code-translator-macos-x64.dmg
- code-translator-linux-x64.AppImage
- code-translator-android-arm64.apk

## 3) Tag and push
- git tag v0.1.0
- git push origin main
- git push origin v0.1.0

## 4) Create GitHub Release
- Go to GitHub → Releases → Draft new release
- Choose tag: v0.1.0
- Title: Release v0.1.0 - Initial Public Release
- Paste content from RELEASE_TEMPLATE.md
- Upload assets (same names as above)
- Publish release

## 5) Validate links
Check these links open/download correctly:
- https://github.com/HGT22/code_and_pseudocode_translator/releases/latest/download/code-translator-windows-x64.exe
- https://github.com/HGT22/code_and_pseudocode_translator/releases/latest/download/code-translator-macos-arm64.dmg
- https://github.com/HGT22/code_and_pseudocode_translator/releases/latest/download/code-translator-macos-x64.dmg
- https://github.com/HGT22/code_and_pseudocode_translator/releases/latest/download/code-translator-linux-x64.AppImage
- https://github.com/HGT22/code_and_pseudocode_translator/releases/latest/download/code-translator-android-arm64.apk

## 6) Post-release
- Update README if needed
- Announce release in repo description/discussions
- Open issue for next milestone (v0.1.1)
