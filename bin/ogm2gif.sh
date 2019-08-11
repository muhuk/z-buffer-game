#!/usr/bin/env sh

INPUT_FILE=$1
OUTPUT_FILE=$2

FPS=15
WIDTH=640

START_TIME=00:00:03
DURATION=00:00:05

ffmpeg -i $INPUT_FILE \
       -ss $START_TIME \
       -t $DURATION \
       -filter_complex "[0:v]fps=$FPS,scale=$WIDTH:-1:flags=lanczos,split [a][b];[a] palettegen [p];[b][p] paletteuse=dither=bayer:bayer_scale=5:diff_mode=rectangle" \
       -y $OUTPUT_FILE
