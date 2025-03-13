pub trait ParagraphSplitter {
    fn split(&self, text: &str, max_len: usize) -> Vec<String>;
}
#[derive(Debug, Clone, Copy)]
pub enum ParagraphSplitStrategy {
    #[cfg(feature = "nlp")]
    Nlp,
    SimpleSplit,
    Truncate,
}

impl ParagraphSplitter for ParagraphSplitStrategy {
    fn split(&self, text: &str, max_len: usize) -> Vec<String> {
        match self {
            #[cfg(feature = "nlp")]
            ParagraphSplitStrategy::Nlp => nlp_split(text, max_len),

            ParagraphSplitStrategy::SimpleSplit => simple_split(text, max_len),
            ParagraphSplitStrategy::Truncate => truncate_split(text, max_len),
        }
    }
}

#[cfg(feature = "nlp")]
fn nlp_split(text: &str, _max_len: usize) -> Vec<String> {
    vec![text.to_string()] // Placeholder for real NLP logic
}

fn simple_split(text: &str, max_len: usize) -> Vec<String> {
    text.split('.')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

fn truncate_split(text: &str, max_len: usize) -> Vec<String> {
    text.chars()
        .collect::<Vec<_>>()
        .chunks(max_len)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn main() {
    let strategy: Box<dyn ParagraphSplitter> = Box::new(ParagraphSplitStrategy::SimpleSplit);
    let text = "This is a test. This is another sentence. And a final one.";
    let result = strategy.split(text, 10);

    println!("{:?}", result);
}
