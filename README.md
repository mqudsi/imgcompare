## `imgcompare`

`imgcompare` is a visual diff utility for images; useful for quickly determining whether or not two images are pixel-identical. Comparisons are performed in the `rgba` color space, support for explicitly specifying the comparison domain will be coming in a future version (see below).

### Usage

Usage is straightforward. Given two image files of supported formats (for example, `image1.png` and `image2.jpg`), the command `imgcompare image1.png image2.jpg` can be used to determine whether the two images are pixel-for-pixel identical. An exit code of `0` means the images are identical, while a non-zero exit code indicates that one or more differing pixels were found between the provided images.

### Installation

`imgcompare` is available for installation on supported platforms via the `cargo` package manager:

`> cargo install imgcompare`

Pre-compiled, signed binaries for select platforms are available from the imgcompare homepage at https://neosmart.net/imgcompare/

### Future Development

Currently, `imgcompare` is a binary diff utility in the sense that it only returns whether or not two images are identical. `imgcompare` does not currently support "perceptual hashing," and does not (yet) tell you just how dissimilar two images may be.

* Perceptual hashing/fuzzy diff
* Support for more filetypes
* Support for explicitly providing color space used for pixel comparison

### License & Credits

`imgcompare` is open source, published under the terms of the MIT license. `imgcompare` is developed and maintained by Mahmoud Al-Qudsi \<mqudsi@neosmart.net> of NeoSmart Technologies \<https://neosmart.net/>. `imgcompare` would not be possible without the existence of countless other open source libraries, graciously published for the community to benefit from and build upon.