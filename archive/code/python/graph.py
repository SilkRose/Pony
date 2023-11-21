#!/usr/bin/env python3

# Originally written by Chat GPT, updated by Silk Rose.
# I don't really like python that much, so I switched to using gnuplot after python.

import json
import matplotlib.pyplot as plt
from matplotlib.ticker import FuncFormatter
from datetime import datetime


def format_y_ticks(value, _):
	if value >= 1_000_000:
		return f"{value/1e6:.0f}M"
	elif value >= 1000:
		return f"{value/1000:.0f}K"
	else:
		return str(int(value))


with open("dist/api/v1/pony-commits.json", "r") as file:
	data = json.load(file)

times = [datetime.utcfromtimestamp(entry["unix_time"]) for entry in data][::-1]
word_counts = [entry["words"] for entry in data][::-1]

plt.style.use("dark_background")

plt.plot(times, word_counts, color="#CC9CDF")

plt.title("Word Count Over Time")
plt.xlabel("Time")
plt.ylabel("Word Count")

start_date = min(times)
end_date = max(times)
date_range = end_date - start_date
num_ticks = 6
date_interval = date_range / (num_ticks - 1)

tick_dates = [start_date + i * date_interval for i in range(num_ticks)]
tick_labels = [date.strftime("%Y-%m-%d") for date in tick_dates]

plt.xticks(tick_dates, tick_labels, rotation=35)

plt.gca().yaxis.set_major_formatter(FuncFormatter(format_y_ticks))

plt.tight_layout()
plt.show()
