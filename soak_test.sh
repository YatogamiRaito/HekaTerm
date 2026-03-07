#!/bin/bash
yes "this is a stress test line to flood the PTY $(date)" | head -n 1000000
