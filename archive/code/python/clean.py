#!/usr/bin/env python3

# Originally written by Chat GPT, updated by Silk Rose.

import os
import re


def process_file(file):
	single_quote_regex = "[‘’`´ʹ]"
	double_quote_regex = "[“”‟″]"
	ellipsis_regex = r"\.{3}"
	comma_star_regex = r",\*"
	comma_underscore_regex = ",_"
	en_dash_regex = "--"
	em_dash_regex = "---"

	with open(file, "r", encoding="utf-8") as f:
		content = f.read()

	modified_content = re.sub(single_quote_regex, "'", content)
	modified_content = re.sub(double_quote_regex, '"', modified_content)
	modified_content = re.sub(ellipsis_regex, "…", modified_content)
	modified_content = re.sub(em_dash_regex, "—", modified_content)
	modified_content = re.sub(en_dash_regex, "–", modified_content)
	modified_content = re.sub(comma_star_regex, "*,", modified_content)
	modified_content = re.sub(comma_underscore_regex, "_,", modified_content)

	with open(file, "w", encoding="utf-8") as f:
		f.write(modified_content)


directory = "./"

for root, _, files in os.walk(directory):
	for file in files:
		if file.endswith(".md"):
			file_path = os.path.join(root, file)
			process_file(file_path)

print("Cleaning completed successfully.")
