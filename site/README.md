# npm 설치 
```
npm install tailwindcss @tailwindcss/cli

```
# 운영 배포시
```
fetch 시 요청을 backend로 보내느방법:
dotenv 사용이 안됨으로 실행시에 변수를 주어야함. 
이렇게 export BACKEND_ADDR=http://192.0.2.10:8081 && trunk build --release 
현재 GIT ACTION 에서 .env.prod 파일을 읽어 export 해주는  방식임. 
BACKEND_ADDR 셋팅을 안하고 실행시 빈값으로 채워짐.

```

# 아이콘
https://heroicons.com/    


# 참고
```
https://tailwindcss.com/docs/transition-timing-function#controlling-the-easing-curve
https://dev.to/fayaz/making-a-navigation-drawer-sliding-sidebar-with-tailwindcss-blueprint-581l
https://yew.rs/docs/concepts/function-components/hooks
https://github.com/yewstack/yew/discussions/2291
https://ko.legacy.reactjs.org/

```




# Yew Trunk Template

This is a fairly minimal template for a Yew app that's built with [Trunk].

## Usage

For a more thorough explanation of Trunk and its features, please head over to the [repository][trunk].

### Installation

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show: [Trunk].
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

Rebuilds the app whenever a change is detected and runs a local server to host it.

There's also the `trunk watch` command which does the same thing but without hosting it.

### Release

```bash
trunk build --release
```

This builds the app in release mode similar to `cargo build --release`.
You can also pass the `--release` flag to `trunk serve` if you need to get every last drop of performance.

Unless overwritten, the output will be located in the `dist` directory.

## Using this template

There are a few things you have to adjust when adopting this template.

### Remove example code

The code in [src/main.rs](src/main.rs) specific to the example is limited to only the `view` method.
There is, however, a fair bit of Sass in [index.scss](index.scss) you can remove.

### Update metadata

Update the `name`, `version`, `description` and `repository` fields in the [Cargo.toml](Cargo.toml) file.
The [index.html](index.html) file also contains a `<title>` tag that needs updating.

Finally, you should update this very `README` file to be about your app.

### License

The template ships with both the Apache and MIT license.
If you don't want to have your app dual licensed, just remove one (or both) of the files and update the `license` field in `Cargo.toml`.

There are two empty spaces in the MIT license you need to fill out: `` and `cheolgyu <cheolgyu@users.noreply.github.com>`.

[trunk]: https://github.com/thedodd/trunk
