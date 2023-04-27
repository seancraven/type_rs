"""
This is the starting point for the Bloom service. I wan't to spin bloom up used some 
precomputed results, to make sure that there is enought time load the model and begin,
inference.
"""
from time import time

import torch
from transformers import AutoModelForCausalLM
from transformers import AutoTokenizer


def test_inf(response_len: int, prompt: str):
    inputs = tokenizer(prompt, return_tensors="pt")
    result_len = inputs["input_ids"].shape[1] + response_len
    device = "cuda" if torch.cuda.is_available() else "cpu"
    model.to(device)
    tokens = inputs["input_ids"].to(device)
    start = time()
    n_beams = tokenizer.decode(
        model.generate(
            tokens,
            max_length=result_len,
            num_beams=3,
            no_repeat_ngram_size=3,
            early_stopping=True,
        )[0]
    )
    end = time()
    n_beams_time = end - start
    print("N-Beams")
    print(n_beams)
    print(n_beams_time)


def make_prompt() -> str:
    with open("src/main.rs", "r") as f:
        lines = f.readlines()
    for i, line in enumerate(lines):
        if "fn main" in line:
            main_start = i
    return "".join(lines[main_start : main_start + 10])


if __name__ == "__main__":
    load_start = time()
    tokenizer = AutoTokenizer.from_pretrained("bigscience/bloom-560m")
    model = AutoModelForCausalLM.from_pretrained("bigscience/bloom-560m")
    load_end = time()
    print(f"Model loading time: {load_end- load_start:.2}")
    test_inf(100, make_prompt())
