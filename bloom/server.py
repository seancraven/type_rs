import logging
import socket
from functools import wraps
from time import sleep
from time import time
from typing import Generator

import torch
from transformers import AutoModelForCausalLM
from transformers import AutoTokenizer
from transformers import BloomForCausalLM
from transformers import BloomTokenizerFast


def log_time(
    func,
):
    """Time function evaluation send to consle if needed"""

    @wraps(func)
    def time_wrap(*args, **kwargs):
        start = time()
        result = func(*args, **kwargs)
        end = time()
        logging.debug("Evaluation time of %s: %s", func.__name__, end - start)
        return result

    return time_wrap


class Server:
    """Server listens for a get request this triggers inference to begin,
    sends the generative response back piecewise."""

    @log_time
    def __init__(self):
        device = "cuda" if torch.cuda.is_available() else "cpu"

        self.tokenizer = AutoTokenizer.from_pretrained("bigscience/bloom-560m")
        self.model = AutoModelForCausalLM.from_pretrained("bigscience/bloom-560m").to(
            device
        )

        self.logger = logging.getLogger()
        self.host = "127.0.0.1"
        self.port = 5087

    @log_time
    def send_response(self, conn: socket.SocketType):
        """The server will send the prompt, as a response to the client."""
        gen = FakeGenerator()
        i = 0
        for resp in gen:
            i += 1
            msg = str(f"Generator iteration {i}")
            msg += resp
            conn.sendall(msg.encode())
            if i > 10:
                break

    def listen(self):
        """Connect to a client, and send typing prompt"""
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as soc:
            soc.bind((self.host, self.port))
            soc.listen()
            connection, _ = soc.accept()
            with connection as conn:
                # 4096 is the number of bites recived
                data = conn.recv(4096)
                logging.info(data.decode())
                self.send_response(conn)


class FakeGenerator:
    """Debugging generator, that returns a prompt, after a fixed amount of time."""

    def __iter__(self) -> Generator[str, None, None]:
        while True:
            yield self.poll_response()

    def poll_response(self) -> str:
        """Generate a response."""
        sleep(2)
        return "Some prompt"


class WordGenerator:
    """Prompts take too long to generate a block of text for a fast and responsive app.

    The generator generates a small snippet of text, at a time and uses this as the next,
    prompt."""

    def __init__(
        self, model: BloomForCausalLM, tokenizer: BloomTokenizerFast, prompt, **kwargs
    ):
        self.model = model
        self.tokenizer = tokenizer
        self.prompt = prompt
        self.verbosity = kwargs.get("verbosity", 0)
        self.prompt_len = len(self.prompt)

    @log_time
    def __iter__(self):
        while True:
            device = "cuda" if torch.cuda.is_available() else "cpu"

            tokens: torch.Tensor = self.tokenizer.encode(
                self.prompt, return_tensors="pt"
            ).to(device)
            result_len = tokens.shape[1] + 50
            response = self.tokenizer.decode(
                self.model.generate(
                    tokens,
                    max_length=result_len,
                    num_beams=3,
                    no_repeat_ngram_size=3,
                    early_stopping=True,
                )[0]
            )
            self.prompt = response[-self.prompt_len :]
            yield response[-50:]


if __name__ == "__main__":
    logging.basicConfig(level=logging.DEBUG, filename="test.log", filemode="w")
    server = Server()
    server.listen()
