import urllib.parse
import urllib.request
import re

query = "Brighton CO city council agenda"
url = "https://html.duckduckgo.com/html/?q=" + urllib.parse.quote(query)

headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
}

try:
    req = urllib.request.Request(url, headers=headers)
    with urllib.request.urlopen(req) as response:
        html = response.read().decode('utf-8')
    
    with open("search_response.html", "w", encoding="utf-8") as f:
        f.write(html)
        
    print("Response saved. Length:", len(html))
except Exception as e:
    print("Error:", e)
