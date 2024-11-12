import os
from tqdm import tqdm
import json
from difflib import SequenceMatcher
from main import clean

clean()

all = []
for file in os.listdir("jsonnew"):
    if ".json" in file:
        items = json.load(open(f"jsonnew/{file}"))
        all.extend(items)

# check the similarity of questions
def similar(a,b):
    return SequenceMatcher(None, a,b).ratio()

#counts = {}
items = []
total = len(all)

duplicates = 0

for item in tqdm(all):
    contains = [x for x in items if x['question'] == item['question'] and x['answers'] == item['answers'] and x['correct_answers'] == item['correct_answers']]
    if len(contains) == 0:
        items.append(item)
    else:
        print(f"Duplicate question found: {item['question']}")
        print(f"Location 1: {contains[0]['source']} {contains[0]['part']} {contains[0]['question_number']}")
        print(f"Location 2: {item['source']} {item['part']} {item['question_number']}\n")
        duplicates += 1

print(f"Total duplicates: {duplicates}")

print(f"Total questions: {total}")
print(f"Unique questions: {len(items)}")
"""
# filtering similar questions
for index, item in tqdm(enumerate(items), total=len(items)):
    string = item['question'] + "".join(item['answers'])
    for item2 in items[index+1:]:
        string2 = item2['question'] + "".join(item2['answers'])
        similarity = similar(string, string2) 
        if similarity > 0.98 and item['correct_answers'] == item2['correct_answers']:
            print(f"Similar questions found:\n{item['question']}\n{item2['question']}")
            print(similarity)
            print("".join(item['answers']))
            print("".join(item2['answers']))
            print(f"Location 1: {item['source']} {item['part']} {item['question_number']}")
            print(f"Location 2: {item2['source']} {item2['part']} {item2['question_number']}")
            exit()
"""

with open("all.json", "w", encoding='utf8') as f:
    json.dump(items, f, ensure_ascii=False, indent=4)