set terminal svg enhanced font 'arial,12' size 400,400
set output 'words_average.svg'

set object 1 rect from screen 0,0 to screen 1,1 behind fc rgb "#ffffff" fillstyle solid noborder

unset key
unset xtics
set style fill solid 0.5
set style boxplot nooutliers

plot 'box.dat' using (1):($1 != 0 ? $1 : 1/0) with boxplot lt 1 lc rgb "#9156A9"
