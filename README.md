<div style="font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; line-height: 1.65;">

  <h1 align="center" style="margin-bottom: 0.15em;">Rust Plotting Playground</h1>

  <p style="font-size: 0.98rem; max-width: 860px; margin: 0.25rem auto 0.75rem auto;">
    A minimal, extensible Rust project showcasing plotting with
    <strong>Plotters</strong>, starting with <strong>line plots</strong> and
    <strong>histograms</strong> implemented as standalone binaries:
    <code>src/bin/line.rs</code> and <code>src/bin/histogram.rs</code>.
    Additional plot types (pie, scatter, bar, heatmap, etc.) can be added as new files
    under <code>src/bin</code> without changing the overall project structure.
  </p>

  <p align="center" style="font-size: 0.95rem; color: #666; margin-top: 0;">
    Built with Rust (Edition 2021), Cargo, and Plotters.
    Works on Windows, Linux, and macOS.
  </p>

</div>

<hr />

<!-- ========================================================= -->
<!-- Table of Contents                                        -->
<!-- ========================================================= -->

<ul style="list-style: none; padding-left: 0; font-size: 0.96rem; line-height: 1.8;">
  <li> <a href="#about-this-repository">About this repository</a></li>
  <li> <a href="#quick-start">Quick start</a></li>
  <li> <a href="#installing-rust-and-adding-dependencies">Installing Rust and adding dependencies</a></li>
  <li> <a href="#project-structure">Project structure</a></li>
  <li> <a href="#build-system-how-rust-projects-build-on-all-platforms">Build system: how Rust projects build on all platforms</a></li>
  <li> <a href="#build-and-run-on-windows-shell-powershell-cmd-git-bash">Build and run on Windows (Shell: PowerShell / CMD / Git Bash)</a></li>
  <li> <a href="#build-and-run-on-windows-vs-code-f5">Build and run on Windows (VS Code / F5)</a></li>
  <li> <a href="#common-build-errors-on-windows-and-fixes">Common build errors on Windows and fixes</a></li>
  <li> <a href="#dependencies-cargotoml">Dependencies (Cargo.toml)</a></li>
  <li> <a href="#line-plots-module-srcbinliners">Line plots module (src/bin/line.rs)</a></li>
  <li> <a href="#histogram-plots-module-srcbinhistogramrs">Histogram plots module (src/bin/histogram.rs)</a></li>
  <li> <a href="#implementation-tutorial-video">Implementation tutorial video</a></li>
</ul>

<hr />

## About this repository

This repository is a practical playground for producing publication-quality plots in Rust using the
<a href="https://crates.io/crates/plotters" target="_blank" rel="noopener noreferrer">Plotters</a> ecosystem.

Current plot modules:

<ul>
  <li><code>src/bin/line.rs</code> — multi-series line plots, markers, tiled layouts, 3×2 subplot grid, and a CSV-driven scatter example.</li>
  <li><code>src/bin/histogram.rs</code> — binning rules, normalization modes, categorical histograms, overlays, PDF comparison, and a CSV-driven histogram.</li>
</ul>

Output images are written to an <code>output/</code> folder (created automatically).

<hr />

## Quick start

<div style="background: #f7f7f9; border: 1px solid #e5e7eb; border-radius: 10px; padding: 0.9rem 1rem; margin: 0.75rem 0 1rem 0;">
  <strong>Linux / macOS</strong>
  <div style="margin-top: 0.5rem;">
    <pre style="margin: 0; white-space: pre-wrap;"><code>git clone &lt;YOUR_REPO_URL&gt;
cd rust_plot

cargo run --bin line
cargo run --bin histogram</code></pre>
  </div>
</div>

<div style="background: #f7f7f9; border: 1px solid #e5e7eb; border-radius: 10px; padding: 0.9rem 1rem; margin: 0.75rem 0 1rem 0;">
  <strong>Windows</strong>
  <div style="margin-top: 0.5rem;">
    <pre style="margin: 0; white-space: pre-wrap;"><code>git clone &lt;YOUR_REPO_URL&gt;
cd rust_plot

# PowerShell (recommended): see the full Windows section for MSVC toolchain setup
cargo run --bin line
cargo run --bin histogram</code></pre>
  </div>
</div>

After running, check the <code>output/</code> directory for PNG files.

<hr />

## Installing Rust and adding dependencies

### Install Rust (all OS)

The standard approach is to install Rust via <strong>rustup</strong> (the official toolchain installer/manager).
It installs:

<ul>
  <li><code>rustc</code> (the compiler)</li>
  <li><code>cargo</code> (the build tool and package manager)</li>
  <li><code>rustup</code> (toolchain manager)</li>
</ul>

After installation, verify:

<pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>rustc -V
cargo -V
rustup show</code></pre>

### Adding crates (dependencies)

Rust dependencies are declared in <code>Cargo.toml</code> under <code>[dependencies]</code>.

You have two common workflows:

<ol>
  <li>
    <strong>Edit Cargo.toml manually</strong> (deterministic, explicit):
    <pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>[dependencies]
plotters = { version = "0.3.7", default-features = true, features = ["histogram"] }</code></pre>
  </li>
  <li>
    <strong>Use cargo-add</strong> (optional; requires <code>cargo-edit</code>):
    <pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>cargo install cargo-edit
cargo add plotters@0.3.7</code></pre>
  </li>
</ol>

Cargo will resolve versions, download crates from crates.io, compile them, and cache build artifacts under <code>target/</code>.

<hr />

## Project structure

<pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>rust_plot/
├─ Cargo.toml
└─ src/
   └─ bin/
      ├─ line.rs
      └─ histogram.rs</code></pre>

### What each file does

<ul>
  <li>
    <strong><code>Cargo.toml</code></strong><br/>
    Project manifest describing package metadata (name/version/edition) and dependencies.
    Cargo uses this as the single source of truth for builds.
  </li>
  <li>
    <strong><code>src/bin/line.rs</code></strong><br/>
    A standalone binary target that generates multiple line plot examples and writes PNG outputs under <code>output/</code>.
  </li>
  <li>
    <strong><code>src/bin/histogram.rs</code></strong><br/>
    A standalone binary target that generates multiple histogram examples (including normalization + binning rules) and writes PNG outputs under <code>output/</code>.
  </li>
</ul>

### Why <code>src/bin</code>?

Any file under <code>src/bin/*.rs</code> becomes an independently runnable binary.
That makes the repository naturally modular: adding a new plot type is as simple as adding a new file, e.g. <code>src/bin/pie.rs</code>, then running:

<pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>cargo run --bin pie</code></pre>

<hr />

## Build system: how Rust projects build on all platforms

Rust uses a unified build tool (<strong>Cargo</strong>) across platforms.

### Key concepts

<ul>
  <li><strong>Crate</strong>: a compilation unit. A package may produce one or more crates (library and/or binaries).</li>
  <li><strong>Package</strong>: the project described by <code>Cargo.toml</code>.</li>
  <li><strong>Target</strong>: an output artifact (binary or library). This repo has two bin targets: <code>line</code> and <code>histogram</code>.</li>
  <li><strong>Profiles</strong>: build configurations like <code>dev</code> (default for <code>cargo run</code>) and <code>release</code> (<code>cargo run --release</code>).</li>
  <li><strong>Features</strong>: dependency feature flags that enable optional functionality (example: Plotters histogram support).</li>
  <li><strong>Build scripts</strong>: some crates compile C/C++ code or probe the system (<code>build.rs</code>). For example, cryptography/networking stacks may build native code (e.g., <code>ring</code> via <code>cc</code>).</li>
</ul>

### What happens when you run <code>cargo build</code> / <code>cargo run</code>?

<ol>
  <li>Cargo reads <code>Cargo.toml</code>, resolves dependencies, and generates/updates <code>Cargo.lock</code>.</li>
  <li>Crates are downloaded to a shared registry cache (per-user).</li>
  <li>Compilation occurs into <code>target/&lt;profile&gt;/</code>.</li>
  <li>Rust code is compiled by <code>rustc</code>. If a dependency contains native code, it is compiled with the system compiler toolchain.</li>
  <li>Finally, the linker produces the executable (and PDBs on Windows, if applicable).</li>
</ol>

### Toolchain differences by OS

<ul>
  <li>
    <strong>Linux</strong>: typically uses <code>gcc</code> or <code>clang</code> as the system toolchain when native code is involved. Install build essentials if you see native compile failures.
  </li>
  <li>
    <strong>macOS</strong>: uses Apple Clang via Xcode Command Line Tools (<code>xcode-select --install</code>).
  </li>
  <li>
    <strong>Windows</strong>: you can build with:
    <ul>
      <li><strong>MSVC toolchain</strong> (<code>x86_64-pc-windows-msvc</code>) — recommended for ecosystem compatibility.</li>
      <li><strong>GNU toolchain</strong> (<code>x86_64-pc-windows-gnu</code>) — works, but can be more sensitive to PATH/tool collisions (MSYS2/Git/MinGW).</li>
    </ul>
  </li>
</ul>

<hr />

<section id="build-and-run-on-windows-shell-powershell-cmd-git-bash"></section>

## Build and run on Windows (Shell: PowerShell / CMD / Git Bash)

On Windows, the most reliable setup for Rust crates that compile native code is the <strong>MSVC toolchain</strong>.

### Step 1 — Select the MSVC Rust toolchain

In PowerShell:

<pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>rustup toolchain install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc</code></pre>

### Step 2 — Install Visual Studio Build Tools and Windows SDK

Install <strong>Build Tools for Visual Studio 2022</strong> (or full Visual Studio) and include:

<ul>
  <li>MSVC v143 (x64/x86 build tools)</li>
  <li>Windows 10/11 SDK</li>
</ul>

These provide:
<ul>
  <li><code>cl.exe</code> (C/C++ compiler)</li>
  <li><code>link.exe</code> (MSVC linker)</li>
  <li>Windows libraries such as <code>kernel32.lib</code></li>
  <li>Standard headers such as <code>stddef.h</code></li>
</ul>

### Step 3 — One-session build/run in PowerShell (robust and copy-paste friendly)

<h2 style="margin-top: 1.5rem;">Windows (PowerShell) — MSVC toolchain build &amp; run</h2>

  <p style="max-width: 900px;">
    This section provides a reliable PowerShell workflow for building and running this project on Windows using the
    <strong>MSVC</strong> toolchain. The goal is to avoid common issues where MSYS2/Git Bash tools shadow MSVC tooling
    (especially <code>link.exe</code>), and to ensure the linker can find critical Windows SDK libraries such as
    <code>kernel32.lib</code>.
  </p>

  <div style="padding: 0.85rem 1rem; border-left: 4px solid #d0d7de; background: #f6f8fa; border-radius: 8px; margin: 1rem 0;">
    <p style="margin: 0;">
      <strong>Important:</strong> In the commands below, some paths are <em>system-dependent</em>.
      If you copy/paste these commands, you must update the paths/versions according to your own installation.
      The comments explain exactly what to change and how to locate the correct values.
    </p>
  </div>

  <h3 style="margin-top: 1.25rem;">How to find the required paths on your system</h3>

  <ul style="max-width: 950px; line-height: 1.65;">
    <li>
      <strong>Find your MSVC toolset version (<code>$msvcVer</code>)</strong><br />
      Navigate to:
      <code>C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\</code><br />
      The folder name(s) inside (example: <code>14.44.35207</code>) are your MSVC versions.
      Pick the newest one.
    </li>
    <li>
      <strong>Find your Windows SDK version (<code>$sdkVer</code>)</strong><br />
      Navigate to:
      <code>C:\Program Files (x86)\Windows Kits\10\Lib\</code><br />
      The folder name(s) inside (example: <code>10.0.22621.0</code>) are SDK versions.
      The script below selects the newest automatically.
    </li>
    <li>
      <strong>Find MSVC <code>link.exe</code> path</strong><br />
      It is typically located at:<br />
      <code>C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\&lt;MSVC_VERSION&gt;\bin\Hostx64\x64\link.exe</code><br />
      Replace <code>&lt;MSVC_VERSION&gt;</code> with the same value used in <code>$msvcVer</code>.
    </li>
    <li>
      <strong>Project folder path</strong><br />
      Update the <code>cd</code> command to the folder where your project’s <code>Cargo.toml</code> is located.
      (In this example, it is a folder under the Desktop.)
    </li>
  </ul>

  <h3 style="margin-top: 1.25rem;">PowerShell commands</h3>

  <pre style="background:#0b1020; color:#e6edf3; padding: 1rem; border-radius: 10px; overflow:auto; font-size: 0.92rem; line-height: 1.55;">
# 1) MSVC version you have (taken from your MSVC folder name)
#    Change this to match your system.
#    How to find it:
#      Open:   C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\
#      Pick the newest folder name (example: 14.44.35207)
$msvcVer = "14.44.35207"

# 2) MSVC lib directory (x64)
#    This is the MSVC toolchain library directory used by the linker.
#    If you installed Visual Studio in a different edition/location, you may need to adjust the base path.
$msvcLib = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\$msvcVer\lib\x64"

# 3) Windows SDK (Windows Kits) library directories
#    The script below auto-selects the newest installed SDK version under:
#      C:\Program Files (x86)\Windows Kits\10\Lib\
#    If your Windows Kits is installed elsewhere, update $sdkRoot.
$sdkRoot = "C:\Program Files (x86)\Windows Kits\10\Lib"
$sdkVer  = (Get-ChildItem $sdkRoot -Directory | Sort-Object Name -Descending | Select-Object -First 1).Name

#    These two are required for Windows system libraries:
#      - um\x64   (contains kernel32.lib, user32.lib, etc.)
#      - ucrt\x64 (contains Universal CRT libraries)
$umLib   = Join-Path $sdkRoot "$sdkVer\um\x64"
$ucrtLib = Join-Path $sdkRoot "$sdkVer\ucrt\x64"

# 4) Sanity checks
#    These should print: True / True
#    If either prints False, your installation is missing components or paths are wrong.
Test-Path (Join-Path $umLib "kernel32.lib")
Test-Path (Join-Path $msvcLib "msvcrt.lib")

# 5) Tell the linker where to find .lib files (critical on Windows)
#    We prepend MSVC + Windows SDK library paths to the LIB environment variable.
#    This helps avoid "kernel32.lib not found" and related linker errors.
$env:LIB = "$msvcLib;$ucrtLib;$umLib;$env:LIB"

# 6) Force Cargo to use MSVC's linker (NOT MSYS2/Git's link.exe)
#    Change this path if your MSVC version differs or if Visual Studio is installed elsewhere.
$env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64\link.exe"

# 7) Build and run from the repository root (where Cargo.toml is)
#    Update this path to your clone location.
cd "$env:USERPROFILE\Desktop\Rust_Plot-main\Rust_Plot-main"

#    Clean (optional) then run the binaries.
cargo clean
cargo run --bin line
cargo run --bin histogram
  </pre>

  <div style="padding: 0.85rem 1rem; border-left: 4px solid #0969da; background: #eef6ff; border-radius: 8px; margin: 1rem 0;">
    <p style="margin: 0;">
      <strong>Expected result:</strong> If everything is configured correctly, the executables will run and the generated
      plot images will be written into the <code>output/</code> folder under the repository root.
    </p>
  </div>

### Step 4 — CMD.exe (classic Developer Command Prompt style)

You can do the same in CMD.exe by chaining <code>VsDevCmd.bat</code> and Cargo.

<pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>REM In cmd.exe from repo root:
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
for /f "usebackq delims=" %i in (`"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do set "VSPATH=%i"
call "%VSPATH%\Common7\Tools\VsDevCmd.bat" -arch=x64 -host_arch=x64

REM Optional: force Cargo linker to MSVC link.exe
for /f "delims=" %i in ('where link.exe') do (set "CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER=%i" & goto :done)
:done

cargo run --bin line
cargo run --bin histogram</code></pre>

### Step 5 — Git Bash notes

Git Bash is convenient for Git operations but can introduce tool collisions because Git ships a Unix-like environment.
If you run into linker confusion in Git Bash, the simplest approach is to call CMD from Git Bash:

<pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code># In Git Bash from repo root:
cmd.exe /c "cargo run --bin line"
cmd.exe /c "cargo run --bin histogram"</code></pre>

If you need MSVC environment loading, run the PowerShell approach above, or call <code>VsDevCmd.bat</code> from CMD as shown.

<hr />

<section id="build-and-run-on-windows-vs-code-f5"></section>

## Build and run on Windows (VS Code / F5)

VS Code is a strong workflow for this repository because it can:

<ul>
  <li>Build with tasks that load the MSVC environment</li>
  <li>Launch the compiled executable with one keypress (<strong>F5</strong>)</li>
  <li>Keep output paths consistent by setting <code>cwd</code> to the workspace folder</li>
</ul>

### Recommended extensions

<ul>
  <li><strong>rust-analyzer</strong> (language server)</li>
  <li><strong>C/C++</strong> (only needed if you want <code>cppvsdbg</code> debugging or MSVC debugging integration)</li>
</ul>

### Minimal F5 workflow

A typical setup is:

<ol>
  <li>VS Code runs a <strong>preLaunchTask</strong> that loads the MSVC environment and runs <code>cargo build --bin line</code> (or another bin).</li>
  <li>VS Code launches the resulting <code>target\debug\line.exe</code> directly.</li>
</ol>

This approach avoids accidentally “debugging PowerShell” and keeps the debug output clean.

### Output folder behavior

If your plot programs write to a relative path like <code>output/...</code>, then setting the launch configuration’s working directory (<code>cwd</code>) to the repository root ensures that images land in:

<ul>
  <li><code>${workspaceFolder}\output\</code> (Windows)</li>
  <li><code>./output/</code> (Linux/macOS)</li>
</ul>

<hr />

## Common build errors on Windows and fixes

This section focuses on errors that can occur when native toolchains collide (MSYS2/Git/MinGW/Visual Studio) or when required SDK components are missing.

<hr />

### 1) Cargo picks the wrong <code>link.exe</code> (MSYS2 / Git)

<strong>Symptom</strong> (typical):

<ul>
  <li>Error mentions <code>C:\msys64\usr\bin\link.exe</code> or <code>...\Program Files\Git\usr\bin\link.exe</code></li>
  <li>Error contains messages like: <code>/usr/bin/link: extra operand</code></li>
</ul>

<strong>Why it happens</strong><br/>
Those environments ship a Unix-like <code>link.exe</code> that is not the MSVC linker expected by the Rust MSVC toolchain.

<strong>How to diagnose</strong>

<pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>where.exe link.exe
Get-Command link.exe</code></pre>

<strong>Fix</strong>

<ul>
  <li>Load the MSVC environment using <code>VsDevCmd.bat</code> (see Windows shell section).</li>
  <li>Force Cargo to use MSVC link.exe for the session:
    <pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>$env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER = (where.exe link.exe | Select-Object -First 1)</code></pre>
  </li>
  <li>If necessary, remove Git/MSYS2 <code>usr\bin</code> entries from your PATH for the session.</li>
</ul>

<hr />

### 2) <code>LNK1181: cannot open input file 'kernel32.lib'</code>

<strong>Symptom</strong>: MSVC <code>link.exe</code> runs, but cannot find Windows system libraries.

<strong>Most common causes</strong>

<ul>
  <li>Windows SDK not installed</li>
  <li>Build environment not initialized (missing <code>LIB</code> paths)</li>
</ul>

<strong>Fix</strong>

<ul>
  <li>Install the Windows 10/11 SDK via Visual Studio Installer.</li>
  <li>Run builds from an environment initialized with <code>VsDevCmd.bat</code> (recommended).</li>
  <li>If you must set LIB manually (advanced), add:
    <ul>
      <li>MSVC: <code>...\VC\Tools\MSVC\&lt;ver&gt;\lib\x64</code></li>
      <li>SDK UCRT: <code>...\Windows Kits\10\Lib\&lt;sdkver&gt;\ucrt\x64</code></li>
      <li>SDK UM: <code>...\Windows Kits\10\Lib\&lt;sdkver&gt;\um\x64</code></li>
    </ul>
  </li>
</ul>

<hr />

### 3) <code>fatal error C1083: Cannot open include file: 'stddef.h'</code> (often from <code>ring</code> / native crates)

<strong>Symptom</strong>: a dependency that compiles C code fails because MSVC headers are missing.

<strong>Why it happens</strong><br/>
The MSVC toolset is incomplete or the environment lacks INCLUDE paths.

<strong>Fix</strong>

<ul>
  <li>In Visual Studio Installer / Build Tools, install:
    <ul>
      <li>MSVC v143 (x64/x86 build tools)</li>
      <li>Windows 10/11 SDK</li>
    </ul>
  </li>
  <li>Load MSVC environment with <code>VsDevCmd.bat</code> before running Cargo.</li>
</ul>

<hr />

### 4) Network / CSV download errors from <code>reqwest</code> (example: Windows socket permission 10013)

The line and histogram modules include examples that fetch an Iris dataset CSV via HTTPS.
If your environment blocks outbound connections, you may see errors such as:

<ul>
  <li><code>Os { code: 10013, kind: PermissionDenied, message: "An attempt was made to access a socket in a way forbidden..." }</code></li>
</ul>

<strong>Likely causes</strong>

<ul>
  <li>Corporate firewall / endpoint protection policy</li>
  <li>VPN restrictions or proxy requirements</li>
  <li>Local firewall rules blocking the executable</li>
</ul>

<strong>Practical fixes</strong>

<ul>
  <li>Try running once from a different network (e.g., home network) to confirm the cause.</li>
  <li>Allow the binary in Windows Defender Firewall (outbound rules) if policy permits.</li>
  <li>Use a local CSV file instead of downloading. Both files contain comments showing how to read from disk using <code>csv::Reader::from_path(...)</code>.</li>
</ul>

<hr />

### 5) Useful Cargo commands for troubleshooting

<pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code># Clean build artifacts
cargo clean

# Print the exact linker and flags used
cargo build --bin line --verbose

# Build release
cargo run --release --bin histogram</code></pre>

<hr />

<section id="dependencies-cargotoml"></section>

## Dependencies (Cargo.toml)

Your current manifest:

<pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>[package]
name = "rust_plot"
version = "0.1.0"
edition = "2021"

[dependencies]
plotters = { version = "0.3.7", default-features = true, features = ["histogram"] }
rand = "0.9"
rand_distr = "0.5"
csv = "1.3"
reqwest = { version = "0.12", default-features = false, features = ["blocking", "rustls-tls"] }</code></pre>

### What these dependencies are used for

<ul>
  <li>
    <strong>plotters</strong><br/>
    The plotting backend. This repo uses <code>BitMapBackend</code> to render directly to PNG files.
    The <code>histogram</code> feature is enabled to support histogram-related types/helpers.
  </li>
  <li>
    <strong>rand</strong> and <strong>rand_distr</strong><br/>
    Random number generation and sampling from distributions (Normal distribution) for synthetic data.
  </li>
  <li>
    <strong>csv</strong><br/>
    Reading CSV data (e.g., the Iris dataset) from network or local sources.
  </li>
  <li>
    <strong>reqwest</strong> (blocking + rustls-tls)<br/>
    Downloading CSV content over HTTPS using a pure-Rust TLS stack (rustls) to reduce OS-level TLS dependencies.
  </li>
</ul>

<hr />

<section id="line-plots-module-srcbinliners"></section>

## Line plots module (<code>src/bin/line.rs</code>)

This section documents the line plot implementation and is designed to be modular so future plot types can follow a similar pattern.

### Overview

<code>line.rs</code> generates multiple line-based figures and writes them as PNG files into <code>output/</code>.
It demonstrates:

<ul>
  <li>Multiple series on shared axes</li>
  <li>Markers (circle/triangle/cross) layered over lines</li>
  <li>Tiled layout (2×1)</li>
  <li>3×2 subplot grid with different examples per panel</li>
  <li>CSV-driven scatter plot (Iris dataset) using <code>reqwest</code> + <code>csv</code></li>
</ul>

### Imports and what they provide

<ul>
  <li><code>plotters::prelude::*</code> — the primary Plotters API (ChartBuilder, shapes, colors, series).</li>
  <li><code>plotters::coord::Shift</code> — coordinate system type required by the bitmap drawing backend.</li>
  <li><code>plotters::coord::types::RangedCoordf64</code> — coordinate range type for floating-axis charts.</li>
  <li><code>rand::prelude::*</code> — RNG for minor styling variation in scatter points.</li>
  <li><code>std::fs</code> — create the <code>output</code> directory.</li>
  <li><code>reqwest</code> + <code>csv</code> — network fetch + parsing for the CSV example.</li>
</ul>

### Output sizing strategy (300 DPI style)

The module defines constants that approximate “print-ready” canvases:

<ul>
  <li><code>FIG_300DPI = (2400, 1600)</code> ≈ 8×5.33 inches at 300 DPI</li>
  <li><code>BIG_GRID_300DPI = (3600, 2400)</code> for multi-panel figures</li>
</ul>

### Core helper functions

<ul>
  <li>
    <strong><code>linspace(start, end, n)</code></strong><br/>
    Generates evenly spaced samples (useful for parametric curves and function plots).
  </li>
  <li>
    <strong><code>ensure_output_dir()</code></strong><br/>
    Creates <code>output/</code> if it does not exist. This keeps output stable across all launch methods.
  </li>
  <li>
    <strong><code>with_png_root(path, size)</code></strong><br/>
    Creates a PNG drawing surface using <code>BitMapBackend</code>, and fills the background with white.
  </li>
  <li>
    <strong><code>draw_mesh_f64(chart, x_desc, y_desc)</code></strong><br/>
    Applies consistent mesh styling (labels, axis captions, font sizes) for floating-point axes.
  </li>
</ul>

### Example functions and generated outputs

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">

<strong>Example 1 — Multiple line plots on the same axes</strong><br/>
File: <code>output/line_1_multiple.png</code><br/>
Demonstrates layering multiple <code>LineSeries</code> plus markers using <code>Circle</code> on sampled indices.

</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">

<strong>Example 2 — Plotting from a collection of vectors</strong><br/>
File: <code>output/line_2_vectors.png</code><br/>
Loops over a <code>Vec&lt;Vec&lt;f64&gt;&gt;</code> and draws each series with a distinct palette selection:
<code>Palette99::pick(idx).stroke_width(3)</code>.

</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">

<strong>Example 3 — Sin family plots</strong><br/>
File: <code>output/line_3_sin_family.png</code><br/>
Shows multiple sinusoidal curves with phase shifts. Useful for comparing series styling and mesh configuration.

</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">

<strong>Example 4 — Sin family with markers</strong><br/>
File: <code>output/line_4_sin_markers.png</code><br/>
Overlays different marker shapes (<code>Circle</code>, <code>TriangleMarker</code>, <code>Cross</code>) on each series at regular intervals.

</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">

<strong>Example 5 — Simple 2×1 tiled layout</strong><br/>
File: <code>output/line_5_tiled.png</code><br/>
Uses <code>split_evenly((2, 1))</code> to create a two-row figure. Each panel builds its own chart with independent captions and axes.

</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">

<strong>Example 6 — 3×2 grid of subplots</strong><br/>
File: <code>output/line_6_grid_3x2.png</code><br/>
Creates six panels using <code>split_evenly((3, 2))</code>. Panels include:
<ul>
  <li>sin(x) with marker indices</li>
  <li>tan(sin(x)) − sin(tan(x)) with markers</li>
  <li>cos(5x)</li>
  <li>a “time plot” using custom x tick labels</li>
  <li>sin(5x)</li>
  <li>a parametric circle with axis-range correction for aspect ratio</li>
</ul>

</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">

<strong>Example 7 — CSV scatter plot (Iris dataset)</strong><br/>
File: <code>output/line_7_csv_scatter.png</code><br/>
Downloads <code>iris.csv</code> via HTTPS, parses the headers to locate:
<code>sepal_length</code> (x) and <code>petal_length</code> (y), then renders each row as a semi-transparent scatter point.
If network access is blocked, switch to local CSV reading (see troubleshooting).
</div>

### How to run just the line module

<pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>cargo run --bin line</code></pre>

<hr />

<section id="histogram-plots-module-srcbinhistogramrs"></section>

## Histogram plots module (<code>src/bin/histogram.rs</code>)

This section documents the histogram implementation and is designed to remain modular for future additions (KDE plots, box plots, violin plots, etc.).

### Overview

<code>histogram.rs</code> builds histogram plots “from first principles”:

<ul>
  <li>Computes bin edges</li>
  <li>Counts samples per bin</li>
  <li>Applies multiple normalization modes</li>
  <li>Renders bars using Plotters rectangles</li>
</ul>

This gives you fine-grained control over histogram semantics and styling.

### Imports and what they provide

<ul>
  <li><code>plotters::prelude::*</code> — chart building, shapes, colors.</li>
  <li><code>plotters::coord::Shift</code> and <code>RangedCoordf64</code> — bitmap coordinate plumbing for float axes.</li>
  <li><code>rand</code> + <code>rand_distr::Normal</code> — generate synthetic Normal data sets.</li>
  <li><code>std::thread</code> + <code>std::time::Duration</code> — optional pause between example outputs.</li>
</ul>

### Core statistical helpers

<ul>
  <li><code>mean</code>, <code>std_dev</code> — summary statistics used by binning rules.</li>
  <li><code>quantile</code> and <code>iqr</code> — used to compute the Freedman–Diaconis rule bin width.</li>
  <li><code>data_min_max</code> — determines stable plot ranges, and expands ranges when data is constant.</li>
</ul>

### Binning rules implemented

<ul>
  <li><strong>Sturges</strong> (<code>bins_sturges</code>)</li>
  <li><strong>Square-root rule</strong> (<code>bins_sqrt</code>)</li>
  <li><strong>Scott</strong> (<code>bins_scott</code>)</li>
  <li><strong>Freedman–Diaconis</strong> (<code>bins_fd</code>)</li>
  <li><strong>Auto</strong> (<code>bins_auto</code>) — uses a conservative max of Sturges and FD</li>
</ul>

### Normalization modes

The <code>Normalization</code> enum provides several interpretations of histogram “height”:

<ul>
  <li><code>Count</code> — raw bin counts</li>
  <li><code>CountDensity</code> — count divided by bin width</li>
  <li><code>Probability</code> — fraction of total samples per bin</li>
  <li><code>Pdf</code> — probability density (probability divided by bin width)</li>
</ul>

### Rendering strategy: <code>draw_histogram(...)</code>

The <code>draw_histogram</code> function:

<ul>
  <li>Builds a Cartesian chart based on <code>edges</code> for x-range and max height for y-range</li>
  <li>Draws a mesh (axes + labels)</li>
  <li>Renders each bin as a rectangle from <code>(x0, 0)</code> to <code>(x1, height)</code></li>
</ul>

### Example functions and generated outputs

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">
<strong>Example 1 — Basic histogram (auto binning)</strong><br/>
File: <code>output/histogram_1.png</code><br/>
Generates 10,000 standard-normal samples and plots raw counts with automatically chosen bins.
</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">
<strong>Example 2 — 2×3 panel comparison of binning rules</strong><br/>
File: <code>output/histogram_2.png</code><br/>
Renders six histograms side-by-side using different binning strategies (including an “integers rule”).
</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">
<strong>Example 3 — Demonstrating bin count changes</strong><br/>
Files: <code>output/histogram_3_step_0.png</code> and <code>output/histogram_3_step_1.png</code><br/>
Writes multiple outputs while changing the bin count.
</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">
<strong>Example 4 — Custom bin edges + count density</strong><br/>
File: <code>output/histogram_4.png</code><br/>
Uses explicitly defined non-uniform bin edges and normalizes by bin width (count density).
</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">
<strong>Example 5 — Categorical histogram (bar chart)</strong><br/>
File: <code>output/histogram_5.png</code><br/>
Counts string categories (<code>yes</code>, <code>no</code>, <code>undecided</code>) and renders them as bars.
</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">
<strong>Example 6 — Overlaid normalized histograms</strong><br/>
File: <code>output/histogram_6.png</code><br/>
Computes probability-normalized histograms for two distributions and overlays the bars with translucency.
</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">
<strong>Example 7 — PDF-normalized histogram + theoretical normal PDF curve</strong><br/>
File: <code>output/histogram_7.png</code><br/>
Uses PDF normalization and overlays a theoretical normal distribution curve using a line series.
</div>

<div style="border-left: 4px solid #e5e7eb; padding-left: 1rem; margin: 0.5rem 0 1rem 0;">
<strong>Example 8 — CSV-driven histogram (Iris dataset)</strong><br/>
File: <code>output/histogram_8_csv.png</code><br/>
Downloads <code>iris.csv</code>, extracts the <code>sepal_length</code> column, and plots a histogram of that feature.
</div>

### How to run just the histogram module

<pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>cargo run --bin histogram</code></pre>

<hr />

## Extending the repository (adding new plot types)

To add a new plot type (e.g., pie chart), follow this pattern:

<ol>
  <li>Create a new file: <code>src/bin/pie.rs</code></li>
  <li>Implement a <code>main()</code> that writes outputs to <code>output/</code> (use <code>fs::create_dir_all("output")</code>)</li>
  <li>Run it with Cargo:
    <pre style="background:#0b1020; color:#e6edf3; padding:0.75rem 0.9rem; border-radius:10px; overflow:auto;"><code>cargo run --bin pie</code></pre>
  </li>
  <li>Add a new section to this README mirroring the modular structure of the Line/Histogram sections.</li>
</ol>

<hr />

## Implementation tutorial video

Replace <code>YOUR_VIDEO_ID</code> below with your YouTube video ID once it is available.

<a href="https://www.youtube.com/watch?v=GLQJT52KAnc" target="_blank" rel="noopener noreferrer">
  <img
    src="https://i.ytimg.com/vi/GLQJT52KAnc/maxresdefault.jpg"
    alt="Rust Plotting Playground - Implementation Tutorial"
    style="max-width: 100%; border-radius: 10px; box-shadow: 0 6px 20px rgba(0,0,0,0.15); margin-top: 0.6rem;"
  />
</a>
