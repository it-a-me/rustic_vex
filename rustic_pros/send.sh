#/bin/bash
RUSTDIR="$(dirname $0)"
(cd $RUSTDIR && cargo build --release)
echo $0
cp -v $RUSTDIR/target/armv7a-none-eabi/release/librustic_pros.a $RUSTDIR/../firmware/.
(cd $RUSTDIR && pros b && pros u)

