set terminal svg enhanced font 'arial,12' size 800,400
set output 'words_over_time.svg'

set object 1 rect from screen 0,0 to screen 1,1 behind fc rgb "#ffffff" fillstyle solid noborder

unset key
set xlabel 'Date'
set ylabel 'Words'
set title 'Words Over Time'
set style line 1 lc rgb '#CC9CDF'

stats 'line.dat' using 1:2 nooutput
min_value = STATS_min_y
max_value = STATS_max_y

set xdata time
set timefmt '%s'
set format x "%Y-%m-%d"

num_ticks = 6
first_date = STATS_min_x
last_date = STATS_max_x
date_range = last_date - first_date
tick_interval = date_range / (num_ticks - 1)

set xrange [STATS_min_x - tick_interval / 4:STATS_max_x + tick_interval / 4]

t1 = first_date
t2 = first_date + tick_interval
t3 = first_date + 2 * tick_interval
t4 = first_date + 3 * tick_interval
t5 = first_date + 4 * tick_interval
t6 = last_date

set xtics (t1, t2, t3, t4, t5, t6)

plot 'line.dat' using 1:2 with lines title 'Words'
