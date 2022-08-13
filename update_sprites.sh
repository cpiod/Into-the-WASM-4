w4 png2src --rust ui.png > src/sprcnst.rs
w4 png2src --rust atlas.png >> src/sprcnst.rs
w4 png2src --rust chars.png >> src/sprcnst.rs
sed -i -e "s/^const/pub const/" -e "/_HEIGHT/d" src/sprcnst.rs
./update.sh
