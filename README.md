# arith

## Contributors and Contributions

This assignment has been completed by Nicholas Mendes and Matthew Kelley.
Help was received from the sources below:

* Connor Gray: bitpacking, overview of assignment
* Issac Chen: troubleshooting bitshifts, clarification on assignment steps
* [rust-lang.com](https://www.rust-lang.org/): understanding `.to_be_bytes()` and `u32::from_be_bytes()`
* [rust-lang.com User Forum](https://users.rust-lang.org/t/convert-vec-to-array-with-tryfrom/50727): converting a Vec into an Array

## rpeg Implementation

### What is Working

* Compression: The program is able to accept `.ppm` files as input and produce compressed `.rpeg` files as output
* Decompression: The program is able to accept `.rpeg` files as input and produce decompressed `.ppm` files as output
* Bitpacking: The program is able to insert and extract signed and unsigned integer values into a 64 bit word, along with checking to ensure that
a particular value has enough space to be stored correctly
* Pixel Conversion: The program is able to convert between RGB and Component Video (YPbPr) representations
* Quantization: The program is able to perform discrete cosine transformation to convert between the luma values of a 2 x 2 pixel block and the
respective coefficients

### What is Not Working

* Nothing

## rpeg Architecture

### Compression

* The program reads in a file from either the command line or from standard input
* The image is read into an `RgbImage`, and the dimensions are rounded down to the nearest even number (e.g. 37 -> 36, but 44 -> 44)
* Manually looping over each of the indicies stepping by 2, the top left corner and the other pixels in the block are stored into a `PixelBlock`
struct and stored in a new `Array2<PixelBlock>`
* The Array2 of grouped pixels, the trimmed and halved dimensions, and the denominator of the image are produced
* The contents of of Array2 are looped over. For every `PixelBlock`, the Rgb pixels are extracted out into a Vec.
* For every Rgb, they are converted into a floating point representation of Rgb, and then into Component Video representation
* The Pb and Pr chroma values are then averaged together, and each of the luma are stored
* Each of the luma are turned into the cosine coefficients using discrete cosine transformation
* Each of the a, b, c, d, Pb, and Pr are turned into the appropriate integer representation and then stored into a 32 bit code word.
* Each of the 32 bit code words generated are then turned into 4 bytes in Big-Endian order before being set to write the image to standard out

### Decompression

* The program reads in a file from either the command line or from standard input
* The image is read and extracted into a `Vec<[u8; 4]>` and the associated dimensions
* Each of the arrays in the Vec are turned into 32 bit code words, and each word is assigned an appropriately scaled position in row major order
* All of the values stored in the 32 bit words (a, b, c, d, Pb, Pr) are extracted out and turned into their floating point representations
* a, b, c, and d are turned back into the luma of the individual pixels (y1, y2, y3, y4)
* Each of the luma, along with the Pb and Pr chroma, are turned back into floating point RGB, and then into `Rgb` pixels, ensuring that each of the floating point RGB values is on a scale from 0.0 - 1.0
* These pixels are then packed into a group and then unpacked to apply the coordinates to them, and all of the pixels are collected into a new `Vec`.
* Each of the pixel coordinates is turned into a row major index and the Vec is then sorted by that index before it is removed.
* A new `RgbImage` is created using the new `Vec`, the extracted dimensions, and a denominator of 255
* The new image is written to standard output

## Time Used

Approximately 5 hours was spent conceptually on understanding the problem and the process, deciding how best to implement the solution, and coming up with test cases.

Approximately 14 hours was spent writing code, tests, and documentation for the assignment, including manual testing of images and verifying their output.
