// Import `Segmenter` trait.
use crate::segmenter::Segmenter;
use tokenizer::{Tokenizer, th};
use once_cell::sync::Lazy;


/*
Make a small documentation of the specialized Segmenter like below.
*/
/// <Script/Language> specialized [`Segmenter`].
///
/// This Segmenter uses [`<UsedLibraryToSegment>`] internally to segment the provided text.
/// <OptionalAdditionnalExplanations>
//
//TIP: Name the Segmenter with its purpose and not its internal behavior:
//     prefer JapaneseSegmenter (based on the Language) instead of LinderaSegmenter (based on the used Library).
//     Same for the filename, prefer `japanese.rs` instead of `lindera.rs`.

pub struct ThaiSegmenter;
static SOMCHAI: Lazy<tokenizer::th::Tokenizer> = Lazy::new(||

{
    let tokenizer = th::Tokenizer::new("C:\\Users\\macth\\Desktop\\Rust-Playground\\src\\words_th.txt").expect("Dictionary file not found");
    tokenizer
});

/*
All specialized segmenters only need to implement the method `segment_str` of the `Segmenter` trait.
*/
impl Segmenter for ThaiSegmenter {
    fn segment_str<'o>(&self, to_segment: &'o str) -> Box<dyn Iterator<Item = &'o str> + 'o> {
        // Create the iterator that will segment the provided text.
        // Assuming dictinoary contains "ภาษาไทย" and "นิดเดียว" but not "ง่าย"
        //let somchai = th::Tokenizer::new("C:\\Users\\macth\\Desktop\\Rust-Playground\\src\\words_th.txt").expect("Dictionary file not found");
    
        let segmented = SOMCHAI.tokenize(to_segment);

        // Return the created iterator wrapping it in a Box.
        Box::new(segmented.into_iter())
    }
}


//TIP: Some segmentation Libraries need to initialize a instance of the Segmenter.
//     This initialization could be time-consuming and shouldn't be done at each call of `segment_str`.
//     In this case, you may want to store the initialized instance in a lazy static like below and call it in `segment_str`.
//     Otherwise, just remove below lines.
//
// Put this import at the top of the file.
// use once_cell::sync::Lazy;
//
// static LIBRARY_SEGMENTER: Lazy<LibrarySegmenter> = Lazy::new(|| LibrarySegmenter::new());

// Publish the newly implemented Segmenter:
//	   - import module by adding `mod dummy;` (filename) in `segmenter/mod.rs`
//	   - publish Segmenter by adding `pub use dummy::DummySegmenter;` in `segmenter/mod.rs`
//     - running `cargo doc --open` you should see your Segmenter in the segmenter module

// Test the segmenter:
#[cfg(test)]
mod test {
    use crate::segmenter::test::test_segmenter;

    // Original version of the text.
    const TEXT: &str = "ภาษาไทยง่ายนิดเดียว";

    // Segmented version of the text.
    const SEGMENTED: &[&str] = &["ภาษาไทย", "ง่าย", "นิดเดียว"];

    // Segmented and normalized version of the text.
    const TOKENIZED: &[&str] = SEGMENTED;
    // Macro that run several tests on the Segmenter.
    test_segmenter!(ThaiSegmenter, TEXT, SEGMENTED, TOKENIZED, Script::Thai, Language::Tha);
}

// Include the newly implemented Segmenter in the tokenization pipeline:
//	   - assign Segmenter to a Script and a Language by adding it in `SEGMENTERS` in `segmenter/mod.rs`
//	   - check if it didn't break any test or benhchmark

// Your Segmenter will now be used on texts of the assigned Script and Language. Thank you for your contribution, and congratulation! 🎉
