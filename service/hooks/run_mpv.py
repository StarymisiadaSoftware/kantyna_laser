#!/usr/bin/python3
import subprocess
import os

# 1. Read URL from environment
#
# Will throw an exception if the env var does't exist
url = os.environ['KANTYNA_LASER_URL']

# 2. Run mpv
subprocess.call(["mpv",url])