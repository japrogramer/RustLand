import json

def read_data(self):
    speaker = [i for i in range(10)]
    s = json.dumps(speaker)
    print("python print", speaker)
    return s
