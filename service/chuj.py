import sys
import subprocess

for line in sys.stdin:
    line = line.replace("\n","")
    subprocess.call(["mpv",line])
    
