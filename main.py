import os
import json

def clean():
    for file in os.listdir("json"):
        if ".json" in file:
            items = json.load(open(f"json/{file}"))
            source = ""
            if "_part" in file:
                source = 'awslagi'
            elif "examtopics" in file:
                source = 'examtopics'
            # get just digits from file name 
            part = int(''.join(filter(str.isdigit, file)))
            print(file, part)
            for index, item in enumerate(items):
                item['source'] = source
                item['part'] = part
                item['question_number'] = index+1
                
            with open(f"jsonnew/{file}", "w", encoding='utf8') as f:
                json.dump(items, f, ensure_ascii=False, indent=4)
        