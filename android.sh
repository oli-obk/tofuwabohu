rustup target add armv7-linux-androideabi aarch64-linux-android i686-linux-android thumbv7neon-linux-androideabi x86_64-linux-android
rm -r /usr/local/cargo/registry # workaround for https://github.com/not-fl3/macroquad/issues/400
cargo quad-apk build --release
