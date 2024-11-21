import json

with open('/Users/felix/.aws-clf-c02-quiz/progress.json') as f:
    progress = json.load(f)

    # search for duplicates in the already_answered_uuids array
    uuids = progress['already_answered_uuids']
    # check if set length is equal to list length
    if len(set(uuids)) != len(uuids):
        print("Duplicates found")
    else:
        print("No duplicates found")