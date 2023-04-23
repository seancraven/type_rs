# Plan

Simple command line typing app, start off very basic. Just pass in a file and type it, from the cmd line.


First job is to render the text, onto the console.


# Implementing a Sliding Window Veiw Over a file


```Rust
impl<'a> IntoIter FileLines<'a> {

    fn into_iter(self, n:usize) -> Iterator{
    // Make a sliding window view of the file that can then be displayed. 
    
    }
  }
```
