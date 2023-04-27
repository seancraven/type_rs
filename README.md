# type_rs
Not to be confused with types this is a small command line typing app that I am working on as a side project. 
I got sick and tired of how slow monkeytype was. I also would prefer to type in sentences than random words. 
However quotes seem to general, I think that there is space to have a generative model end point for the typing.
# Plan
Goal, have a generative model backend that makes semantically sensible text to practice typing.
- [x] Simple typing app.
- [x] Simple analytics
- [ ] BLOOM for text generation.
 - [ ] BLOOM server is starting to go
 - [ ] Need to decide on some messageing protocall.
 - [ ] Need to implement client once I have implemented the server.
# Todo:





# Notes
 - Using gpu, can get inference down to one second, big overhead is loading the model into memory. 
 - Possibly preload the model as a server, and then across localhost shoot a message over the net. 
 - Inference for whole thing is huge bottleneck, think I need to get my head around the encoding and masking.
 - Probably a good idea to get the small gpt model up and running for a good idea about what is going on. 
 - Current state is I can generate semantically sensible text, from any file that I have written.
 - I think with a few smart prompt writes I can get something that writes convincing classes/ functions in python and rust. 
Prompts along the lines of this. 
```python
def main():
  with open("file.txt") as f:
    # do some file parsing.

```
Could scrape small number of functions from github and have them be prompts and use the model in a more generative context. 
Same with some short story website.
