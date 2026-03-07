import subprocess
import time
import os

def get_rss(pid):
    try:
        with open(f"/proc/{pid}/statm", "r") as f:
            pages = int(f.read().split()[1])
            return pages * 4 # 4KB pages to KB
    except FileNotFoundError:
        return 0

print("Starting wezterm-gui...", flush=True)
proc = subprocess.Popen(["./target/release/wezterm-gui", "--config", "periodic_stat_logging=2", "start"])
pid = proc.pid
print(f"Spawned GUI with PID {pid}", flush=True)
time.sleep(3)

idle_rss = get_rss(pid)
print(f"IDLE RSS: {idle_rss} KB", flush=True)

print("Spawning stress payload...", flush=True)
subprocess.run(["./target/release/wezterm-gui", "cli", "spawn", "--", "bash", "-c", 'for i in {1..50000}; do echo "Stress Test Memory Footprint Line $i"; done; sleep 5'])

time.sleep(5)
active_rss_1 = get_rss(pid)
time.sleep(2)

print(f"ACTIVE RSS (1): {active_rss_1} KB", flush=True)

proc.terminate()
