# rust_iron_practice

# Debug
```
cargo run
sudo target/debug/rust_iron_practice
```

Access http://localhost through some browser.

# Release for armv7 (Raspberry Pi 2, 3)

## Setup
```
rustup target add armv7-unknown-linux-gnueabihf
sudo apt install gcc-arm-linux-gnueabihf
echo "
[target.armv7-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
" >> ~/.cargo/config
```

## Release
```
./scripts/release_armv7.sh
```

Send `armv7.zip` for ARM device.

## Use
On ARM device.
```
unzip armv7.zip
cd armv7
sudo ./rust_iron_practice
```

Access device ip through some browser.

# References
- [iron](https://github.com/iron/iron)
- [iron/examples/time](https://github.com/iron/iron/blob/master/examples/time.rs)
- [RustでWebプログラミング No.2 ~ Routerをつかって複数ルート~](http://poketo7878-dev.hatenablog.com/entry/2016/09/24/112929)
- [iron/router](https://github.com/iron/router)
- [RustでWebプログラミング No.3 ~ HandlebarでHTMLテンプレート~](http://poketo7878-dev.hatenablog.com/entry/2016/09/24/185005)
- [handlebars-iron](https://github.com/sunng87/handlebars-iron)
- [handlebars-iron/examples/templates/index.hbs](https://github.com/sunng87/handlebars-iron/blob/master/examples/templates/index.hbs)
- [handlebars-rust/examples/partials.rs](https://github.com/sunng87/handlebars-rust/blob/master/examples/partials.rs)
- [handlebars-rust/examples/partials/base0.hbs](https://github.com/sunng87/handlebars-rust/blob/master/examples/partials/base0.hbs)
- [handlebars-rust/examples/partials/template2.hbs](https://github.com/sunng87/handlebars-rust/blob/master/examples/partials/template2.hbs)
- [handlebars](https://handlebarsjs.com/partials.html)
- [Rust’s Iron Framework: Serving Static Content](https://medium.com/@ericdreichert/rusts-iron-framework-serving-static-content-e996186717b7)
- [iron/staticfile](https://github.com/iron/staticfile)
- [iron/params](https://github.com/iron/params)
- [IronでWebサービスを作るために必要だったもの](https://qiita.com/nacika_ins/items/fa1a8a72d78398dc4661)
- [Download - Bootstrap(4.3)](https://getbootstrap.com/docs/4.3/getting-started/download/)
- [File hierarchy](https://doc.rust-lang.org/rust-by-example/mod/split.html)
- [Implementing Serialize](https://serde.rs/impl-serialize.html)
