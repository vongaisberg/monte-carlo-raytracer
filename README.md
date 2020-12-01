# monte-carlo-raytracer
A monte-carlo raytracer written in Rust.

Monte-Carlo raytracing (or pathtracing) is a way to synthesize physically accurate images.
Many effects that would be hard to do with image rasterization or traditional raytracing happen automatically in my raytracer.

Examples of those effects include: 

- Soft shadows
- Indirect lighting
- Non-point-like light sources
- Complex surface models (Any BRDF works)
- Depth of field
- Any color model you like (does not have to be RGB or even limited to three primary colors)
- Complex caustics (I could simulate a working telescope without cheating)


The code is very modular, but there are a few things missing:

- Support for objects other than spheres
- Acceleration data structures (Given a number of objects in a scene n, my code runs in O(n), but accelerator structures should take it to O(log(n)))
- True concurrency


