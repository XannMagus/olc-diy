# Purpose

This repository is a Rust-based implementation of the DIY language series by @OneLoneCoder on Youtube.

I didn't want to follow the tutorial too closely and wanted to rather take in the lessons of the videos and make them my own.

# Differences

So far, I have two major functional discrepancies with the reference implementation:
- I don't allow `.` in symbols yet. I have a feeling it might become necessary in the future but for now I'm avoiding it until I have to add it.
- I haven't added any boilerplate for `Keywords` yet. I will implement this as it becomes necessary when we start handling actual keywords.

Other than that, the differences mainly fall under the category "coding style". I tend to segregate more than David did in his video.
Also I'm trying to write idiomatic Rust but I'm a beginner so any feedback is welcome.