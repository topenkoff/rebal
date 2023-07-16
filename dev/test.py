import socket
import time
import concurrent.futures as pool
import sys

HOST = '172.42.0.100'
PORT = 5000
UPSTREAMS = 5

def send():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        start = time.perf_counter()
        s.connect((HOST, PORT))
        s.sendall(b'ping')
        data = s.recv(1024)
        e = time.perf_counter() - start
        print(data)
        print(f"time = {e}")

def ping():
    send()

def load():
    executor = pool.ThreadPoolExecutor(max_workers=5)
    for _ in range(1, 10_000):
        executor.submit(send)

def main():
    args = sys.argv
    mode = sys.argv[1]
    if mode == "ping":
        ping()
    else:
        load()

if __name__ == "__main__":
    main()

