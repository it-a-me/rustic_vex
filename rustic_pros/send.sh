#/bin/bash
RUSTDIR="$(dirname $0)"
(cd $RUSTDIR && cargo build)
echo $0
cp -v $RUSTDIR/target/armv7a-none-eabi/debug/librustic_pros.a $RUSTDIR/../firmware/.
(cd $RUSTDIR && pros b && pros u)

