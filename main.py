import os
import json
import uuid

def add_missing_uuids():
    for file in os.listdir("json"):
        finished_items = []
        if ".json" in file:
            with open(f"json/{file}", "r", encoding='utf8') as f:
                items = json.load(f)
                for item in items:
                    if 'uuid' not in item:
                        item['uuid'] = str(uuid.uuid4())
                    finished_items.append(item)
            with open(f"json/{file}", "w", encoding='utf8') as f:
                json.dump(finished_items, f, ensure_ascii=False, indent=4)

def clean():
    for file in os.listdir("json"):
        if ".json" in file:
            items = json.load(open(f"json/{file}"))
            source = file.split('-')[0]
            # get just digits from file name 
            part = int(''.join(filter(str.isdigit, file)))
            print(file, part)
            for index, item in enumerate(items):
                item['source'] = source
                item['part'] = part
                item['question_number'] = index+1
                
            with open(f"jsonnew/{file}", "w", encoding='utf8') as f:
                json.dump(items, f, ensure_ascii=False, indent=4)
        