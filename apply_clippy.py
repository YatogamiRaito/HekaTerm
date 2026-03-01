import json
import os

with open("clippy.json", "r") as f:
    lines = f.readlines()

replacements = []

for line in lines:
    try:
        data = json.loads(line)
    except:
        continue
    
    if data.get("reason") != "compiler-message":
        continue
    
    msg = data.get("message", {})
    code = msg.get("code")
    if not code or code.get("code") != "clippy::manual_let_else":
        continue
    
    # The actual replacement is in a child message
    for child in msg.get("children", []):
        if child.get("level") == "help" and ("consider writing" in child.get("message", "")):
            for span in child.get("spans", []):
                if span.get("is_primary"):
                    replacement = span.get("suggested_replacement")
                    if replacement is not None:
                        file_name = span["file_name"]
                        byte_start = span["byte_start"]
                        byte_end = span["byte_end"]
                        
                        replacements.append({
                            "file": file_name,
                            "byte_start": byte_start,
                            "byte_end": byte_end,
                            "replacement": replacement
                        })

files_to_replace = {}
for r in replacements:
    f = r["file"]
    if f not in files_to_replace:
        files_to_replace[f] = []
    files_to_replace[f].append(r)

count = 0
for f, reps in files_to_replace.items():
    reps.sort(key=lambda x: x["byte_start"], reverse=True)
    
    with open(f, "rb") as file_in:
        content = bytearray(file_in.read())
    
    for r in reps:
        start = r["byte_start"]
        end = r["byte_end"]
        rep_bytes = r["replacement"].encode("utf-8")
        content[start:end] = rep_bytes
        count += 1
        
    with open(f, "wb") as file_out:
        file_out.write(content)

print(f"Applied {count} fixes across {len(files_to_replace)} files.")
