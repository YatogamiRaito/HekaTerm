import os

def check_cargo_toml(filepath):
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Simple check if parking_lot is already in there
    if 'parking_lot' in content:
        return False
        
    lines = content.split('\n')
    deps_index = -1
    for i, line in enumerate(lines):
        if line.strip() == '[dependencies]' or line.strip() == '[dependencies.windows]':
            deps_index = i
            break
            
    if deps_index != -1:
        # Insert parking_lot.workspace = true after [dependencies]
        lines.insert(deps_index + 1, 'parking_lot.workspace = true')
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write('\n'.join(lines))
        print(f"Added parking_lot to {filepath}")
        return True
        
    return False

def main():
    # Only need to check crates that had modifications
    # The output of the previous script gave us a list of modified files.
    # We can just blindly add parking_lot to ALL Cargo.toml files in the workspace 
    # since workspace dependencies don't hurt if unused, but it's cleaner to just add to all
    
    count = 0
    for root, dirs, files in os.walk('.'):
        if 'target' in root or '.git' in root:
            continue
        if 'Cargo.toml' in files:
            path = os.path.join(root, 'Cargo.toml')
            if 'wezterm-gui' not in path: # we already added to gui
                if check_cargo_toml(path):
                    count += 1
    
    print(f"Added parking_lot to {count} Cargo.toml files.")

if __name__ == '__main__':
    main()
