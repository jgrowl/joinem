# dev install

cargo install systemfd cargo-watch


# dev run

systemfd --no-pid -s http::3030 -- cargo watch -x 'run'
