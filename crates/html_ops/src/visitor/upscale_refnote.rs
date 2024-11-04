
/// We have an html file with <sup> tags that reference notes in the document.
// Wr assume that the sup tags are u16 numbers, but occasionally the tags
// may be in forms like "1-5" which we want to expand to "1", "2", "3", "4", "5".
// Forms like "1-4, 6, 8-10" are also possible.
// <sup> elements are often used for other purposes (i.e superscripts!), so we need to be
// careful to validate that the <sup> tag is actually a reference to a note by both pattern
// matching, and also looking for a corresponding referent (i.e the thing that the reference refers to).
// The referent is probably in a <li> as part of an <ol> element, where its reference id is implcitly given
// by its position in the list. We generally should know the container name (i.e "notes" or "references"), otherwise
// we'd have some difficulty finding the referent.
// Once we find the referent, we want to store the inner html for later validtion purposes (i.e. as a fallback), but we'd 
// ultimately like to generate a json object which represents the data within the referent, as it make it easier to later reformat.
// We can do this by searching reference databases, like Pubmed, using the referent string, and extractign the data, if there is a 
// We need to generate a unique id for each reference, and we need to make sure that the id is stable across multiple runs of the program. 
// One approach is to use the md5 hash of the referent string as the id, but that should be a fallback. It would be useful to use the PUBMMED id, 
// if there is one, and generally having ids that relate to the origin of the reference. As such, it is likely that we'll create a rust
// enum for different types of references, and have a method for extracting the id from the reference string.

//starting at our
