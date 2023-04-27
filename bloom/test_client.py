import socket


if __name__ == "__main__":
    HOST = "127.0.0.1"
    PORT = 5087
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as client:
        client.connect((HOST, PORT))
        client.sendall(b"get on with it")
        recieved = ""
        data = client.recv(4096)
        recieved += str(data)
        while data:
            data = client.recv(4096)
            recieved += str(data)

    print("Client has got all it needs")
    print(f"Client recieved:\n{recieved}")
