<h1 align="center">🛠 Contributing to rust-goome 🛠</h1>

<p align="center">
Thanks for wanting to help! Small PRs that fix one thing are the easiest for me to check and merge, so keep them small and theyll get in way faster.<br>
This repo is only the Rust side. If the bug is in goome itself (drawing, a backend, the ui kit) it goes to <a href="https://github.com/goome-ui/c-goome">c-goome</a> instead, docs fixes go to <a href="https://github.com/goome-ui/docs">goome-ui/docs</a> (thats <a href="https://goome-ui.github.io/docs/">goome-ui.github.io/docs</a>) and new example projects to <a href="https://github.com/goome-ui/examples">goome-ui/examples</a>.
</p>

<details>
<summary align="center"><b>📋 The short version 📋</b></summary>

<p align="center">
• One thing per PR. One fix or one feature, if it does two things it gets closed<br>
• Open your PR against the <b>dev</b> branch, not main<br>
• Never touch <code>goome-sys/native/</code>, goome.h in there is copied from c-goome<br>
• Dont touch the protected files, a bot closes the PR if you do<br>
• Test it and say how you tested it, and on which OS<br>
• CI has to pass<br>
• AI is fine, but say you used it, and you need to know what every line does
</p>
</details>

<details>
<summary align="center"><b>💬 Before you start 💬</b></summary>

<p align="center">
Small fixes can go straight to a PR.<br>
For a new feature or anything big, <a href="https://github.com/goome-ui/rust-goome/issues">open an issue</a> first so we can talk about it before you spend a weekend on it.<br> <br>
Same if you want to rename something or change what a function takes or returns, people already use this so I want to talk that through first.
</p>
</details>

<details>
<summary align="center"><b>🌿 Branches 🌿</b></summary>

<p align="center">
Stuff goes dev, then beta, then main. I move it along, you only ever PR into <b>dev</b>.
</p>
</details>

<details>
<summary align="center"><b>🚫 What gets closed automatically 🚫</b></summary>

<p align="center">
A bot checks every PR and closes it with a comment if it hits any of these, fix it and open a new one.<br> <br>
It has to be against <b>dev</b>, main and beta only get stuff that already sat on dev for a while.<br> <br>
It cant touch <code>.github/</code>, since thats CI and these rules and a PR could just turn them off for itself. <code>goome-sys/native/</code> is off limits too, <code>goome.h</code> in there gets copied from c-goome by <code>tools/update_native.sh</code> and built on everyones computer when they build the crate. Same for every <code>build.rs</code>, every <code>Cargo.toml</code>, <code>Cargo.lock</code>, <code>.cargo/</code> and <code>tools/</code>, they run on peoples computers or pick what does. The <code>LICENSE</code> and the logo in <code>docs/</code> stay mine to change.<br> <br>
• <b>More than 3000 changed files</b> - way too big to check<br> <br>
Renaming or moving any of those files counts too. Think one of them is wrong? <a href="https://github.com/goome-ui/rust-goome/issues">Open an issue</a> and I'll fix it.
</p>
</details>

<details>
<summary align="center"><b>🤖 Using AI 🤖</b></summary>

<p align="center">
Using AI to help write your PR is allowed, as long as you say so in the PR and you know what your code does. If you get asked about something you should be able to answer without going back to the AI.<br> <br>
And test it yourself. Untested AI dumps get closed
</p>
</details>

<details>
<summary align="center"><b>🧪 Testing 🧪</b></summary>

<p align="center">
Run the tests. They all run on goome's test backend, so nothing shows up on your screen:
</p>

```sh
cargo test
GOOME_BACKEND=test GOOME_TEST_SCRIPT="screen 800x600 1; 2 click 100,100; end 8" cargo run --example hello
```

<p align="center">
<code>goome/tests/layout.rs</code> checks every struct in <code>goome-sys</code> and the <code>Theme</code> / <code>Config</code> mirrors against what C says, plus every constant and that every function in goome.h is declared. The C side of that lives in <code>goome-ref/c/ref.c</code>.<br> <br>
The drawing and ui tests draw the same thing from Rust and from C (in <code>goome-ref</code>) and check the hashes match, and some of them are the exact numbers from c-goome's <code>tests/unit.c</code>. If one moves, the binding is passing the wrong thing. Dont just update the number.<br> <br>
Tests run one at a time (<code>.cargo/config.toml</code> sets that), goome isnt thread safe and the test backend is one per process.<br> <br>
In the PR write a few lines on what you ran and on which OS and Rust version. If it changes something you can see (an example, how something draws) add a before and after screenshot.<br> <br>
CI runs on linux (plus Rust 1.77), windows and both macs when you open the PR. It has to be green before I merge.
</p>
</details>

<details>
<summary align="center"><b>✏️ Code style ✏️</b></summary>

<p align="center">
• Rust 1.77 and up, edition 2021<br>
• No dependencies besides <code>cc</code> for the build. Nothing else gets added without talking first<br>
• Every function in goome.h stays reachable. New C functions go in <code>goome-sys/src/lib.rs</code> and get a safe version in <code>goome</code>, new struct fields go in both plus <code>goome-ref/c/ref.c</code> if the test misses them<br>
• No <code>unsafe</code> in the public API. If something can crash C from safe Rust, its a bug<br>
• Anything that holds native memory frees it in <code>Drop</code>, anything with a begin and an end in C hands back a guard that calls the end exactly when <a href="https://goome-ui.github.io/docs/ui-app/">the docs</a> say C wants it<br>
• Run <code>cargo fmt</code> (the repo has a rustfmt.toml), match the code around it<br>
• No comments unless something really isnt obvious<br>
• Dont reformat whole files, it makes the diff impossible to read
</p>
</details>

<details>
<summary align="center"><b>💬 Discussions and first issues 💬</b></summary>
<p align="center">
Questions and stuff you made with goome go in <a href="https://github.com/goome-ui/c-goome/discussions">Discussions</a>.<br>
If you want something to work on, the <a href="https://github.com/goome-ui/rust-goome/labels/good%20first%20issue">good first issue</a> ones are small. Comment on one before you start.
</p>
</details>

<p align="center">
Found a security hole? Dont open a public issue or PR for it, read <a href="https://github.com/goome-ui/.github/blob/main/SECURITY.md">SECURITY.md</a> and tell me privately first.<br> <br>
By sending a PR you agree your code goes out under the same <a href="LICENSE">GPL-3.0 license</a> as the rest of rust-goome.
</p>
