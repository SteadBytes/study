# GENERAL SETTINGS
set key off
set rmargin 5
set grid ytics noxtics nocbtics back
set border 3 back lw 2 lc rgbcolor "#222222"

# X AXIS
set xlabel "Allocation size (bytes)"
set logscale x 2
set xtics nomirror out

# Y AXIS
set ylabel "Time taken for malloc() to return"
set logscale y
set yrange [0.00004 to 0.0002]
set ytics ( \
    "40 {/Symbol m}s"   0.00004, \
    "50 {/Symbol m}s"   0.00005, \
    "75 {/Symbol m}s"   0.000075, \
    "100 {/Symbol m}s"  0.0001, \
    "200 {/Symbol m}s"  0.0002, \
    "250 {/Symbol m}s"  0.00025, \
    "500 {/Symbol m}s"  0.0005, \
)
set ytics nomirror out

# OUTPUT
set term png
plot allocs_tsv using 3:5 pt 7 linecolor rgbcolor "#aafa2a2a"
