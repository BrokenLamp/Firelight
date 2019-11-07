# Firelight

React + Electron + TypeScript + Rust

## Prerequisites

- [Node.js](https://nodejs.org)
- [Rust](https://www.rust-lang.org/tools/install)
- [Python 2.7](https://www.python.org/download/releases/2.7/)

### Linux

- [Make](https://www.gnu.org/software/make/)
- [Clang](https://clang.llvm.org/) or [GCC](http://gcc.gnu.org)

### MacOS

- [XCode](https://developer.apple.com/xcode/download/)

### Windows

- `npm install --global --production windows-build-tools`

OR

- [.NET Framework](http://www.microsoft.com/en-us/download/details.aspx?id=40773)
- [Visual C++ Build Tools](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2019)
- `npm config set python python2.7`
- `npm config set msvs_version 2019`

## Running

```bash
npm i
npm start
```

## Editing the User Interface

After running, the interface should update immediately upon saving the file.
If it doesn't, try restarting the application with `npm start`.

## Editing the Game Logic

Simply running `npm start` isn't enough to compile the Rust code.
You'll need to follow these steps to see your game logic changes:

```bash
npm run build:core
npm start
```
