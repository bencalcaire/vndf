source project.conf

cd $RUST_SOURCE/pan &&
cargo build &&
echo "IDE built. Add it to your path by executing the following command:" &&
echo "export PATH=\$PATH:$BINARY_DIR"
