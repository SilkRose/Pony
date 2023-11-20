set terminal svg enhanced font 'arial,12' size 400,400
set output 'words_average.svg'

set object 1 rect from screen 0,0 to screen 1,1 behind fc rgb "#ffffff" fillstyle solid noborder

set style boxplot outliers pointtype 7

unset key
set boxwidth 1.5
set style fill solid 0.5

#stats 'box.dat' using 1:1 nooutput
#min_value = STATS_min_y
#max_value = STATS_max_y
#set yrange [STATS_min_y - 100:STATS_max_y + 100]

plot 'box.dat' using (1):($1 != 0 ? $1 : 1/0) with boxplot lt 1 lc rgb "#9156A9"
