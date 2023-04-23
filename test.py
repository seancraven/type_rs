"""Simple file to make random variables."""
import random


class Bernoulli:
    """
    A Class for generating Bernoulli random variables.
    """

    def __init__(self, probs):
        self.probs = probs

    def sample(self, size):
        """Sample from a Bernoulli distribution."""
        uniform_random_vars = [random.random() for i in range(size)]
        return [1 if i < self.probs else 0 for i in uniform_random_vars]

    def mean(self):
        """Return the mean of a Bernoulli distribution."""
        return self.probs
