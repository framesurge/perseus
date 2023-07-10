# Install


https://framesurge.sh/perseus/en-US/docs/

```
rustup target add wasm32-unknown-unknown
cargo install perseus-cli
perseus new my-app
cd my-app/
perseus serve -w

```
- rustup show

```

D:\perseus_homepage_test> rustup show
Default host: x86_64-pc-windows-msvc
rustup home:  C:\Users\users\.rustup

installed targets for active toolchain
--------------------------------------

wasm32-unknown-unknown
x86_64-pc-windows-msvc
x86_64-unknown-linux-musl

active toolchain
----------------

stable-x86_64-pc-windows-msvc (default)
rustc 1.70.0 (90c541806 2023-05-31)

```

- nvm list

```
nvm list

    20.4.0
  * 18.16.1 (Currently using 64-bit executable)

```

- nvm use 18

```
nvm use 18
```


- npx tailwindcss init

```
npx tailwindcss init
```
https://tailwindcss.com/docs/configuration

- config full
  - https://github.com/tailwindlabs/tailwindcss/blob/master/stubs/config.full.js
  - tailwindcss Plugins
    - https://tailwindcss.com/docs/plugins

- packages.json setting

```
$ npm i -D tailwind-hamburgers tailwindcss
```


# cargo add perseus-tailwind

```
cargo add perseus-tailwind
```


- perseus test(Localhost)

http://127.0.0.1:8080/

```
perseus serve -w 
```

```
PS D:\csstest> npx tailwindcss init
Need to install the following packages:
  tailwindcss@3.3.2
Ok to proceed? (y) y

Created Tailwind CSS config file: tailwind.config.js
npm notice
npm notice New minor version of npm available! 9.7.2 -> 9.8.0
npm notice Changelog: https://github.com/npm/cli/releases/tag/v9.8.0
npm notice Run npm install -g npm@9.8.0 to update!
npm notice
PS D:\csstest> ls

    Directory: D:b\csstest

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d----        2023-07-10  오후 3:06                .cargo
d----        2023-07-10  오후 3:06                src
-a---        2023-07-10  오후 3:06             13 .gitignore
-a---        2023-07-10  오후 3:06            665 Cargo.toml
-a---        2023-07-10  오후 3:59            128 tailwind.config.js


PS D:\csstest> cargo add perseus-tailwind
    Updating crates.io index
      Adding perseus-tailwind v0.4.7 to dependencies.
    Updating crates.io index

PS D:\csstest> perseus serve -w
```

# file tree & bug

## Don't forget ```npm install```

Overwrite file ```tailwind.css``` of dist folder to file ```tailwind.css``` of static folder. I want to do it in a static folder right away, but I have to copy and put it in to see if there is a bug.

- Save it as ```dist/tailwind.css``` and overwrite the updated ```static/tailwind.css``` with ```dist/tailwind.css```.

```
├─.cargo
├─dist
│  ├─tailwind.css
│
│
├─static
│  ├─styles
│       ├─style.css
├─src
│  └─templates
│        ├─index.rs
│  ├─main.rs
│  ├─tailwind.css
└─static
    └─tailwind.css
```


<hr>

# Error 

https://framesurge.sh/perseus/en-US/docs/0.4.x/fundamentals/error-views/


# website 

https://github.com/framesurge/perseus/tree/main/website


# Docker setting

https://github.com/framesurge/perseus/blob/f3e3f824530d7103fe776282367e0b872ac0f921/docs/0.3.0-0.3.3/en-US/deploying/docker.md?plain=1#L44-L45

# perseus-tailwind

https://crates.io/crates/perseus-tailwind
