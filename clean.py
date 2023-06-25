import os
import re

def process_file(file):
    quote_regex = r'[“”"‟″"]'
    ellipsis_regex = r'\.{3}'
    comma_star_regex = r',\*'

    with open(file, 'r', encoding='utf-8') as f:
        content = f.read()

    modified_content = re.sub(quote_regex, r'"', content)
    modified_content = re.sub(ellipsis_regex, '…', modified_content)
    modified_content = re.sub(comma_star_regex, '*,', modified_content)

    lines = modified_content.split('\n')
    new_lines = []

    for line in lines:
        modified_line = ''
        inside_quote = False

        for char in line:
            if char == '"':
                if inside_quote:
                    modified_line += '”'
                    inside_quote = False
                else:
                    modified_line += '“'
                    inside_quote = True
            else:
                modified_line += char

        new_lines.append(modified_line)

    updated_content = '\n'.join(new_lines)

    with open(file, 'w', encoding='utf-8') as f:
        f.write(updated_content)

directory = './'

for root, _, files in os.walk(directory):
    for file in files:
        if file.endswith('.md'):
            file_path = os.path.join(root, file)
            process_file(file_path)

print('Cleaning completed successfully.')
