//! Playfair cipher implementation in Rust

#![feature(iter_array_chunks)]

/// Bigram type. Used in the Playfair cipher by grouping characters and performing operations on
/// those pairs.
pub type Bigram = (char, char);

/// Position type. Used to store an X, Y value for use in a matrix.
pub type Position = (usize, usize);

/// The Matrix type is a 5 by 5 character array.
pub type Matrix = [[char; 5]; 5];

/// Cipher trait, enforces `encrypt` and `decrypt` methods.
pub trait Cipher {
    /// Encryption logic for a given plaintext
    fn encrypt(&self, plaintext: &str) -> String;
    /// Decryption logic for a given ciphertext
    fn decrypt(&self, ciphertext: &str) -> String;
}

/// Keyword structure, used in constructing the matrix in which the encryption is performed over.
#[derive(Debug, PartialEq)]
pub struct Keyword(String);

impl Keyword {
    /// Create a keyword from an initial input. This will have a size of 25 and will not have any
    /// duplicate letters, and equate the letter 'i' to the letter 'j'. This is to conform to the 5x5 matrix that
    /// the Playfair cipher is used on. The letter 'j' was chosen arbitrarily due to its low use in
    /// the English language. This will mean that upon decryption, you'll notice anything that once
    /// was a 'j' in the initial plain text is now an 'i'.
    ///`
    /// # Example
    /// Key: `playfair`
    /// Message: `Jane`
    /// Encrypted message: `bpun`
    /// Decrypted of encryption: `iane`
    ///
    /// Two things to note with this, it turns everything lowercase for easier searching and
    /// complexity, and j's are now converted to i's.
    ///
    /// ## TODO
    /// According to
    /// [this](https://users.rust-lang.org/t/fast-removing-chars-from-string/24554) post, using
    /// `.retain()` on the initial filitering we do may be faster in release builds. Investigate
    /// more.
    pub fn new(initial: &str) -> Self {
        // Create a string with the capacity of 25 since we know how big this will be. This will
        // eliminate the need for a reallocation, if Rust defaults the capacity to less than 25.
        let mut buffer = String::with_capacity(25);

        // Ensure we only take the alphabetic parts of the input string and
        // remove any instance of 'j'.
        let mut parsed: String = initial
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic() && *c != 'j')
            .collect();

        // Append the alphabet (equating 'i' = 'j', thus omitting 'j') to the initial input, to fill in the rest of the possible letters
        // that the initial input might not cover.
        parsed.push_str("abcdefghiklmnopqrstuvwxyz");

        // We only need 25 letters, so keep pushing to the buffer while we have less than 25
        // characters.
        while buffer.len() < 25 {
            // Loop over each character in the input and alphabet string, checking that the
            // character is alphabetic since we can't use numbers of symbols in our Matrix.
            for c in parsed.chars() {
                // Check that the character does not exist in the buffer
                if buffer.find(c).is_none() {
                    // If so, push to the buffer
                    buffer.push(c);
                }
            }
        }

        // Return the generated keyword
        Self(buffer)
    }

    /// Convert the keyword into a 5x5 [Matrix] array type in.
    /// TODO: This can be converted to a 1-d array
    pub fn to_matrix(&self) -> Matrix {
        // Initialize a matrix to null-bytes to start. They will all be overwritten
        let mut mtx: Matrix = [['\0'; 5]; 5];

        for (idx, chr) in self.0.char_indices() {
            // Perform the x-value calcuation by using modular arithmetic
            let x = idx % 5;
            // Perform the y-value calculation by using integer division
            let y = idx / 5;

            // Set the char at the given x, y value
            mtx[x][y] = chr;
        }

        // Return the matrix
        mtx
    }
}

/// Playfair cipher structure, stores data needed during the encryption/decryption
pub struct Playfair {
    /// The keyword in which we generate the matrix from.
    keyword: Keyword,
    /// The matrix which encryption/decryption is operated over
    matrix: Matrix,
}

impl Cipher for Playfair {
    /// Encryption logic for a given plaintext
    fn encrypt(&self, plaintext: &str) -> String {
        let mut buffer = String::new();
        let bigrams: Vec<Bigram> = Playfair::bigramify(plaintext);

        // Loop over each bigram
        for bigram in bigrams {
            // Get the positions of the characters, needed in performing the operations on swapping
            // or incrementing x & y values.
            let a_pos: Position = self.get_position_in_matrix(&bigram.0);
            let b_pos: Position = self.get_position_in_matrix(&bigram.1);

            if a_pos.0 == b_pos.0 {
                // Case 1: They are in the same column. In this case, we increment (with wrapping)
                // their y-values by 1.
                buffer.push(self.matrix[a_pos.0][(a_pos.1 + 1) % 5]);
                buffer.push(self.matrix[b_pos.0][(b_pos.1 + 1) % 5]);
            } else if a_pos.1 == b_pos.1 {
                // Case 2: They are in the same row. In this case, we increment (with wrapping)
                // their x-values by 1.
                buffer.push(self.matrix[(a_pos.0 + 1) % 5][a_pos.1]);
                buffer.push(self.matrix[(b_pos.0 + 1) % 5][b_pos.1]);
            } else {
                // Case 3: They are in different rows and columns, In this case, we swap the
                // x-values of each position and keep the same y-values.
                buffer.push(self.matrix[b_pos.0][a_pos.1]);
                buffer.push(self.matrix[a_pos.0][b_pos.1]);
            }
        }

        buffer
    }

    /// Decryption logic for a given ciphertext
    fn decrypt(&self, ciphertext: &str) -> String {
        let mut buffer = String::new();
        let bigrams: Vec<Bigram> = Playfair::bigramify(ciphertext);

        // Loop over the bigrams
        for bigram in bigrams {
            // Get the positions of the characters, needed in performing the operations on swapping
            // or decrementing x & y values.
            let a_pos: Position = self.get_position_in_matrix(&bigram.0);
            let b_pos: Position = self.get_position_in_matrix(&bigram.1);

            if a_pos.0 == b_pos.0 {
                // Case 1: They are in the same column. In this case, we increment (with wrapping)
                // their y-values by 1.

                // Subtract 1, producing an optional with the value from the operation. If we try
                // to subtract 1 from 0, .checked_sub() would result in a None being returned, in
                // which case .unwrap_or() will give us a 4, effectively giving us this 'reverse'
                // modular arithmetic
                let a_y = a_pos.1.checked_sub(1).unwrap_or(4);
                let b_y = b_pos.1.checked_sub(1).unwrap_or(4);

                buffer.push(self.matrix[a_pos.0][a_y]);
                buffer.push(self.matrix[b_pos.0][b_y]);
            } else if a_pos.1 == b_pos.1 {
                // Case 2: They are in the same row. In this case, we increment (with wrapping)
                // their x-values by 1.

                // Subtract 1, producing an optional with the value from the operation. If we try
                // to subtract 1 from 0, .checked_sub() would result in a None being returned, in
                // which case .unwrap_or() will give us a 4, effectively giving us this 'reverse'
                // modular arithmetic
                let a_x = a_pos.0.checked_sub(1).unwrap_or(4);
                let b_x = b_pos.0.checked_sub(1).unwrap_or(4);

                buffer.push(self.matrix[a_x][a_pos.1]);
                buffer.push(self.matrix[b_x][b_pos.1]);
            } else {
                // Case 3: They are in different rows and columns, In this case, we swap the
                // x-values of each position and keep the same y-values.
                buffer.push(self.matrix[b_pos.0][a_pos.1]);
                buffer.push(self.matrix[a_pos.0][b_pos.1]);
            }
        }

        buffer
    }
}

impl Playfair {
    /// Generates a new Playfair cipher structure with the keyword and appropriate alphabet padding to
    /// ensure it can fit into the matrix.
    pub fn new(kw: &str) -> Self {
        // Generate the keyword from the given input
        let keyword = Keyword::new(kw);
        // Construct a matrix from the keyword.
        let matrix = keyword.to_matrix();

        // Return the playfair cipher
        Self { keyword, matrix }
    }

    /// Bigramify takes in a string input, converts it to an even length, and splits the input into
    /// groups of 2-tuples of characters. This is then used in the encryption/decryption
    /// algorithms.
    fn bigramify(input: &str) -> Vec<Bigram> {
        let mut buffer: Vec<Bigram> = vec![];
        // Ensure the input is only alphabetic
        let mut input: String = input
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect();

        // Loop over the characters of the input 2 at a time. If there are duplicates insert an 'x'
        // to seperate the duplicates
        for idx in (0..input.len()).step_by(2) {
            let a = input.chars().nth(idx).unwrap();

            if let Some(b) = input.chars().nth(idx + 1) {
                if a == b {
                    input.insert(idx + 1, 'x');
                }
            }
        }

        // If we are still at an odd length, append a 0 at the end of the input.
        if input.len() % 2 != 0 {
            input.push('x');
        }

        // Break the input into chunks of 2. We know everything will be covered because before this
        // we ensure that the input is of even length.
        let chunks = input.chars().array_chunks::<2>();

        // For each chunk, convert it to a 2-tuple and push to the buffer
        chunks.for_each(|chunk| {
            // Push the pair to the buffer
            buffer.push((chunk[0], chunk[1]));
        });

        // Return the buffer
        buffer
    }

    /// Get the position of a given character withing the matrix. Returns a [Position] type, which is an
    /// (x, y) pair of where the character is in the function. Since i = j in this implementation,
    /// whenever the letter 'j' is searched for, just search for 'i' instead.
    fn get_position_in_matrix(&self, to_search: &char) -> Position {
        // Loop over each column and item.
        for (idx, column) in self.matrix.iter().enumerate() {
            for (jdx, chr) in column.iter().enumerate() {
                // Check if we found a match
                if to_search == chr {
                    // Return the position
                    return (idx, jdx);
                }
            }
        }

        // If no position was found, we were probably searching for a 'j', which in our current
        // implementation, i = j, so  return the result for searching for 'i'.
        self.get_position_in_matrix(&'i')
    }

    /// Get a copy of the keyword of the Playfair structure
    pub fn keyword(&self) -> &str {
        self.keyword.0.as_str()
    }

    /// Allow updating the current keyword of the Playfair object. This may be useful if you are
    /// encrypting and decrypting amonst multiple parties at once, and have numerous different
    /// keywords / matricies to operate over.
    pub fn update_keyword(&mut self, kw: &str) {
        // Generate the new keyword from the input
        let kw = Keyword::new(kw);
        // Generate a new matrix from the keyword
        let mx = kw.to_matrix();

        // Update the current keyword
        self.keyword = kw;
        // Update the current matrix to the new matrix
        self.matrix = mx;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_no_dups() {
        let initial = "abcdefg";
        let kw = Keyword::new(initial);

        assert_eq!(kw.0.len(), 25);
        assert_eq!(kw.0, "abcdefghiklmnopqrstuvwxyz");
    }

    #[test]
    fn test_keyword_with_dups() {
        let initial = "aabbccddee";
        let kw = Keyword::new(initial);

        assert_eq!(kw.0.len(), 25);
        assert_eq!(kw.0, "abcdefghiklmnopqrstuvwxyz");
    }

    #[test]
    fn test_keyword_wiki_example() {
        let initial = "playfair example";
        let kw = Keyword::new(initial);

        assert_eq!(kw.0.len(), 25);
        assert_eq!(kw.0, "playfirexmbcdghknoqstuvwz");
    }

    #[test]
    fn test_keyword_weird_input() {
        let initial = "play!!!fa123ir ex^&*ample";
        let kw = Keyword::new(initial);

        assert_eq!(kw.0.len(), 25);
        assert_eq!(kw.0, "playfirexmbcdghknoqstuvwz");
    }

    #[test]
    fn test_keyword_one_letter_input() {
        let initial = "iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii";
        let kw = Keyword::new(initial);

        assert_eq!(kw.0.len(), 25);
        assert_eq!(kw.0, "iabcdefghklmnopqrstuvwxyz");
    }

    #[test]
    fn test_getting_keyword_pf_struct() {
        let initial = "playfair example";
        let pf = Playfair::new(initial);

        assert_eq!(pf.keyword.0, "playfirexmbcdghknoqstuvwz");
    }

    #[test]
    fn test_bigraming_even_length() {
        let initial = "abcd";
        let big = Playfair::bigramify(initial);

        assert_eq!(big, vec![('a', 'b'), ('c', 'd')]);
    }

    #[test]
    fn test_bigraming_odd_length() {
        let initial = "abc";
        let big = Playfair::bigramify(initial);

        assert_eq!(big, vec![('a', 'b'), ('c', 'x')]);
    }

    #[test]
    fn test_bigramming_wiki() {
        let initial = "hide the gold in the tree stump";
        let big = Playfair::bigramify(initial);

        assert_eq!(
            big,
            vec![
                ('h', 'i'),
                ('d', 'e'),
                ('t', 'h'),
                ('e', 'g'),
                ('o', 'l'),
                ('d', 'i'),
                ('n', 't'),
                ('h', 'e'),
                ('t', 'r'),
                ('e', 'x'),
                ('e', 's'),
                ('t', 'u'),
                ('m', 'p'),
            ]
        );
    }

    #[test]
    fn test_matrix_generation() {
        let initial = "playfair example";
        let kw = Keyword::new(initial);

        assert_eq!(kw.0.len(), 25);
        assert_eq!(kw.0, "playfirexmbcdghknoqstuvwz");

        let mx = kw.to_matrix();
        assert_eq!(
            mx,
            [
                ['p', 'i', 'b', 'k', 't'],
                ['l', 'r', 'c', 'n', 'u'],
                ['a', 'e', 'd', 'o', 'v'],
                ['y', 'x', 'g', 'q', 'w'],
                ['f', 'm', 'h', 's', 'z']
            ]
        );
    }

    #[test]
    fn test_finding_in_matrix() {
        let initial = "playfair example";
        let pf = Playfair::new(initial);

        let pos = pf.get_position_in_matrix(&'a');

        assert_eq!(pos, (2, 0));
    }

    #[test]
    fn test_finding_i_j_equivalence() {
        let initial = "playfair example";
        let pf = Playfair::new(initial);

        let pos_1 = pf.get_position_in_matrix(&'i');
        let pos_2 = pf.get_position_in_matrix(&'j');

        assert_eq!(pos_1, pos_2);
    }

    #[test]
    fn test_updating_keyword() {
        let initial = "init";
        let mut pf = Playfair::new(initial);

        assert_eq!(pf.keyword(), "intabcdefghklmopqrsuvwxyz");

        let new = "playfair example";
        pf.update_keyword(new);

        assert_eq!(pf.keyword(), "playfirexmbcdghknoqstuvwz");
    }
}
