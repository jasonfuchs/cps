set datafile separator ","
set timefmt '%s'
set xdata time
set format x "%H:%M:%S"
set ylabel 'Temperature (in °C)'
set xlabel 'Time'

plot 'today.csv' using 1:2 with lines title "Today"

