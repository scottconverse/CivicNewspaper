with open("search_response.html", "r", encoding="utf-8") as f:
    html = f.read()

import re
links = re.findall(r'<a\s+[^>]*href="([^"]+)"[^>]*>(.*?)</a>', html, re.DOTALL)
print(f"Found {len(links)} total links:")
for href, inner in links[:30]:
    inner_clean = re.sub(r'<[^>]*>', '', inner).strip()
    if 'duckduckgo.com' not in href or 'uddg=' in href:
        print(f"HREF: {href}")
        print(f"TEXT: {inner_clean}")
        print("---")
