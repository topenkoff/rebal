import socket
import time
import os
import logging
import sys

HOST = "0.0.0.0"
PORT = 5000
DELAY = os.getenv("DELAY", 1)


logging.basicConfig(stream=sys.stdout, level=logging.DEBUG)
logger = logging.getLogger()

def main():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind((HOST, PORT))
        s.listen()
        while True:
            conn, addr = s.accept()
            with conn:
                logger.info(f"Connected by {addr}")
                while True:
                    try:
                        #time.sleep(int(DELAY))
                        data = conn.recv(1024)
                        if not data:
                            logger.info("end data")
                            break
                        conn.sendall(data)
                    except Exception as e:
                        logger.error("WOOOPS")
                        logger.error(e)
                        return

if __name__ == "__main__":
    main()
