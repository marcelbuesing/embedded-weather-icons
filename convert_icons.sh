#!/bin/bash

echo "use tinybmp::Bmp;" > "./src/lib.rs";

for file in ./weather-icons/svg/*.svg
do
  filename=$(basename "$file" ".svg" | xargs);
  filename_snake=$(echo $filename | sed 's/-/_/g');
  for size in "16x16" "32x32" "64x64" "128x128"
  do
    echo "Converting $file to ./assets/weather-icons/bmp/${filename}_${size}.bmp";
    cat >> ./src/lib.rs <<-EOM

#[cfg(feature = "icons${size}")]
pub fn ${filename_snake}_${size}() -> Result<Bmp<'static>, ()> {
  const IMAGE: &'static [u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/weather-icons/bmp/${filename}_${size}.bmp"));
  Bmp::from_slice(IMAGE)
}
EOM
    # Two step process to turn it into a monochrome picture first
    convert "$file" -resize ${size} -define bmp:subtype=RGB565 -monochrome "./assets/weather-icons/bmp/${filename}_${size}.tmp.bmp" && \
    convert "./assets/weather-icons/bmp/${filename}_${size}.tmp.bmp" -type TrueColor -depth 8 -define bmp:subtype=RGB565 "./assets/weather-icons/bmp/${filename}_${size}.bmp" && \
    rm "./assets/weather-icons/bmp/${filename}_${size}.tmp.bmp" &
  done
done