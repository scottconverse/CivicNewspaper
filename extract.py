import json
with open(r'C:\Users\scott\.gemini\antigravity\brain\8c1c193d-574f-4850-900e-f7ccf6f0f1a1\.system_generated\logs\transcript.jsonl', 'r', encoding='utf-8') as f:
    for line in f:
        data = json.loads(line)
        if data.get('type') == 'USER_INPUT' and 'CivicNews v0.2.0-beta' in data.get('content', ''):
            with open('instructions.txt', 'w', encoding='utf-8') as out:
                out.write(data['content'])
            break
