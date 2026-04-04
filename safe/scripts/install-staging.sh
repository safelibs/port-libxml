#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
STAGE="${1:-$ROOT/safe/target/stage}"
ARTIFACTS_ENV="$ROOT/safe/target/build-artifacts.env"

if [[ ! -f "$ARTIFACTS_ENV" ]]; then
  cargo build --manifest-path "$ROOT/safe/Cargo.toml" --release
fi

# shellcheck disable=SC1090
source "$ARTIFACTS_ENV"

TRIPLET="${LIBXML2_TRIPLET:-$(gcc -print-multiarch)}"
LIBDIR="$STAGE/usr/lib/$TRIPLET"
BINDIR="$STAGE/usr/bin"
INCLUDEDIR="$STAGE/usr/include/libxml2/libxml"
PKGDIR="$LIBDIR/pkgconfig"
ACLOCALDIR="$STAGE/usr/share/aclocal"

rm -rf "$STAGE"
mkdir -p "$LIBDIR" "$BINDIR" "$INCLUDEDIR" "$PKGDIR" "$ACLOCALDIR"

cp "$LIBXML2_NATIVE_STATIC" "$LIBDIR/libxml2.a"
cc -shared \
  -Wl,--no-undefined \
  -Wl,-soname,libxml2.so.2 \
  -Wl,--version-script,"$ROOT/safe/abi/libxml2.syms" \
  -o "$LIBDIR/libxml2.so.$LIBXML2_VERSION" \
  -Wl,--whole-archive \
  "$LIBXML2_NATIVE_STATIC" \
  -Wl,--no-whole-archive \
  -lz -llzma -lm -ldl -lpthread
ln -s "libxml2.so.$LIBXML2_VERSION" "$LIBDIR/libxml2.so.2"
ln -s "libxml2.so.2" "$LIBDIR/libxml2.so"

cp -a "$ROOT/safe/include/libxml/." "$INCLUDEDIR/"
cp "$ROOT/safe/share/aclocal/libxml2.m4" "$ACLOCALDIR/libxml2.m4"

cat >"$PKGDIR/libxml-2.0.pc" <<EOF
prefix=/usr
exec_prefix=\${prefix}
libdir=\${exec_prefix}/lib/$TRIPLET
includedir=\${prefix}/include
modules=1

Name: libXML
Version: $LIBXML2_VERSION
Description: libXML library version2.
Requires:
Libs: -L\${libdir} -lxml2
Libs.private: -lz -llzma -lm -ldl -lpthread
Cflags: -I\${includedir}/libxml2
EOF

cat >"$LIBDIR/xml2Conf.sh" <<EOF
#
# Configuration file for using the XML library in GNOME applications
#
XML2_LIBDIR="-L/usr/lib/$TRIPLET"
XML2_LIBS="-lxml2 -lz  -llzma   -lm "
XML2_INCLUDEDIR="-I/usr/include/libxml2"
MODULE_VERSION="xml2-$LIBXML2_VERSION"
EOF

cat >"$BINDIR/xml2-config" <<EOF
#!/usr/bin/env bash
set -euo pipefail

prefix=/usr
exec_prefix=\${prefix}
includedir=\${prefix}/include
libdir=\${exec_prefix}/lib/$TRIPLET
cflags=
libs=

usage() {
  cat <<USAGE
Usage: xml2-config [OPTION]

Known values for OPTION are:

  --prefix=DIR        change libxml prefix [default \$prefix]
  --exec-prefix=DIR   change libxml exec prefix [default \$exec_prefix]
  --libs              print library dynamic linking information
  --cflags            print pre-processor and compiler flags
  --modules           module support enabled
  --help              display this help and exit
  --version           output version information
USAGE
  exit "\${1:-0}"
}

if [[ \$# -eq 0 ]]; then
  usage 1
fi

while [[ \$# -gt 0 ]]; do
  case "\$1" in
    --prefix=*)
      prefix="\${1#*=}"
      includedir="\$prefix/include"
      libdir="\$prefix/lib/$TRIPLET"
      ;;
    --prefix)
      printf '%s\n' "\$prefix"
      ;;
    --exec-prefix=*)
      exec_prefix="\${1#*=}"
      libdir="\$exec_prefix/lib/$TRIPLET"
      ;;
    --exec-prefix)
      printf '%s\n' "\$exec_prefix"
      ;;
    --version)
      printf '%s\n' "$LIBXML2_VERSION"
      exit 0
      ;;
    --help)
      usage 0
      ;;
    --cflags)
      cflags="-I\${includedir}/libxml2"
      ;;
    --libtool-libs)
      :
      ;;
    --modules)
      printf '1\n'
      ;;
    --libs)
      libs="-lxml2"
      ;;
    *)
      usage 1
      ;;
  esac
  shift
done

if [[ -n "\$cflags\$libs" ]]; then
  printf '%s\n' "\$cflags \$libs" | xargs
fi
EOF
chmod +x "$BINDIR/xml2-config"

compile_stage_binary() {
  local source="$1"
  local output="$2"
  cc -DHAVE_CONFIG_H \
    -I"$ROOT/safe/include" \
    -I"$ROOT/original" \
    -I"$STAGE/usr/include/libxml2" \
    "$source" \
    -L"$LIBDIR" \
    -Wl,-rpath,'$ORIGIN/../lib/'"$TRIPLET" \
    -Wl,--enable-new-dtags \
    -lxml2 -lz -llzma -lm -ldl -lpthread \
    -o "$output"
}

compile_stage_binary "$ROOT/original/xmllint.c" "$BINDIR/xmllint"
compile_stage_binary "$ROOT/original/xmlcatalog.c" "$BINDIR/xmlcatalog"
