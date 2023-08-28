#!/bin/bash

VERSION="v20211022"
RESOURCE_URL="http://www.crac.org.cn/userfiles/file/20211022/20211022155018_8471.zip"

current_dir=$(pwd)
tempdir=$(mktemp -d)

cd $tempdir
curl $RESOURCE_URL -o resource.zip;

unzip resource.zip
cd "TXT题库包(${VERSION})"

if [ -d "$current_dir/public/resources" ]; then
  rm -rf "$current_dir/public/resources"
fi

mkdir -p "$current_dir/public/resources";
iconv --from-code GB18030 "A类题库(${VERSION}).txt" > "$current_dir/public/resources/class_a.txt"
iconv --from-code GB18030 "B类题库(${VERSION}).txt" > "$current_dir/public/resources/class_b.txt"
iconv --from-code GB18030 "C类题库(${VERSION}).txt" > "$current_dir/public/resources/class_c.txt"
iconv --from-code GB18030 "总题库(${VERSION}).txt" > "$current_dir/public/resources/class_all.txt"

mkdir -p "$current_dir/public/resources/pictures";
rsync -a "总题库附图(${VERSION})/" "$current_dir/public/resources/pictures/"

cd $current_dir
rm -rf $tempdir