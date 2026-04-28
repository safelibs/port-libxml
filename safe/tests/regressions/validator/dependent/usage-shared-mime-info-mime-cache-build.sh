#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:?missing repository root}"
STAGE="${2:?missing staged install root}"

if ! command -v update-mime-database >/dev/null 2>&1; then
  printf 'missing required dependent client: update-mime-database\n' >&2
  exit 1
fi

triplet="${LIBXML2_TRIPLET:-}"
if [[ -z "$triplet" ]] && command -v gcc >/dev/null 2>&1; then
  triplet="$(gcc -print-multiarch)"
fi
if [[ -z "$triplet" ]]; then
  printf 'failed to resolve library triplet\n' >&2
  exit 1
fi

libdir="$STAGE/usr/lib/$triplet"
includedir="$STAGE/usr/include/libxml2"
if [[ ! -r "$libdir/libxml2.so.2" ]]; then
  printf 'missing staged libxml2.so.2 under %s\n' "$libdir" >&2
  exit 1
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

cat >"$tmpdir/hash_scan_remove.c" <<'C'
#include <libxml/xmlstring.h>
#include <libxml/dict.h>
#include <libxml/hash.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
    xmlHashTablePtr table;
    int count;
} ScanCtx;

static void remove_current(void *payload, void *data, const xmlChar *name,
                           const xmlChar *name2, const xmlChar *name3) {
    (void) payload;
    (void) name3;
    ScanCtx *ctx = (ScanCtx *) data;
    ctx->count++;
    if (xmlHashRemoveEntry2(ctx->table, name, name2, NULL) != 0) {
        fprintf(stderr, "failed to remove %s/%s\n",
                (const char *) name, (const char *) name2);
        exit(3);
    }
}

int main(void) {
    xmlDictPtr dict = xmlDictCreate();
    if (dict == NULL)
        return 2;
    xmlHashTablePtr table = xmlHashCreateDict(1, dict);
    xmlDictFree(dict);
    if (table == NULL)
        return 3;
    if (xmlHashAddEntry2(table, BAD_CAST "mime-type", BAD_CAST "a", (void *) 1) != 0)
        return 4;
    if (xmlHashAddEntry2(table, BAD_CAST "mime-type", BAD_CAST "b", (void *) 2) != 0)
        return 5;

    ScanCtx ctx = { table, 0 };
    xmlHashScanFull(table, remove_current, &ctx);
    int remaining = xmlHashSize(table);
    xmlHashFree(table, NULL);

    if (ctx.count != 2 || remaining != 0) {
        fprintf(stderr, "scan count=%d remaining=%d\n", ctx.count, remaining);
        return 6;
    }
    return 0;
}
C

cc -O2 -I"$includedir" "$tmpdir/hash_scan_remove.c" \
  -L"$libdir" -lxml2 -Wl,-rpath,"$libdir" \
  -o "$tmpdir/hash_scan_remove"
"$tmpdir/hash_scan_remove"

mkdir -p "$tmpdir/mime/packages"
cp /usr/share/mime/packages/*.xml "$tmpdir/mime/packages/"
LD_LIBRARY_PATH="$libdir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" \
  update-mime-database "$tmpdir/mime" >/dev/null

if [[ ! -r "$tmpdir/mime/mime.cache" || ! -r "$tmpdir/mime/globs2" ]]; then
  printf 'update-mime-database did not create expected MIME cache outputs\n' >&2
  exit 1
fi
grep -F "text/plain" "$tmpdir/mime/globs2" >/dev/null
grep -F "application/xml" "$tmpdir/mime/globs2" >/dev/null

printf 'shared-mime-info dependent regression passed\n'
