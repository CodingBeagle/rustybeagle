# DirectX 11 Learnings

## Shaders

### Vertex Shaders

<https://docs.microsoft.com/en-us/windows/win32/direct3d11/vertex-shader-stage>

The Vertex Shader stage processes vertices from the Input-Assembler stage.

*It must always be active for the pipeline to execute.*

It does per-vertex operations such as transformations, moprhing, per-vertex lighting, etc.

A vertex shader will always:

- Take a single vertex as input.
- Produce a single vertex as output.

Of course a vertex can consist of many different values as a vector, etc.

## Rasterizer Stage

Vertices come into the Rasterizer Stage as four-component vectors (x, y, z, w). They are assumed to be in homogenous clip-space. In this coordinate space:

- The X axis points right.
- The Y axis points up.
- The Z axis points away from the camera (So a positive Z value moves you further away from the camera).


The rasterization stage will convert vector information of shapes or primitives into a raster image (an image composed of pixels). The purpose of this is to then be able to display those pixels on screen as a rendered image.

The rasterization stage will also perform a perspective divide, map primitives to a 2D viewport, clip vertices to the view frustrum, and determine how to invoke the pixel shader.

### Viewports

A viewport maps vertex positions in *clip space* into render target positions.

**Having a 2D viewport bound to the pipeline state is required for the rasterization stage to work**.

## Output-Merger Stage

The Output-Merger stage generates the final rendered pixel color.

The OM stage is the final step for determining which pixels are visible (with depth-stencil testing) and blending the final pixel colors.

The process of using the depth buffer to determine which pixel should be drawn is called **depth buffering**, or sometimes **z-buffering**. Typically, it will cause pixels closer to the camera to be drawn.

Depth values reaching the output-merger stage from interpolation or from the pixel shader, are always clamped as:

z = min(Viewport.MaxDepth, max(Viewport.MinDepth, z))

Meaning, z will never be larger than Viewport.MaxDepth, and never smaller than Viewport.MinDepth.

The clamped depth value is compared against the existing depth-buffer value.

## Effects

Effects appears to have been a deprecated concept in DirectX 11 and above.

In essence, you could have effect files (.fx), which, besides the shader functions for different stages, could also have the concept of a "technique", which included pass that could contain different combinations of shader function calls.

Now, you would have to manage all of this yourself. They were essentailly just higher-level concepts that could help manage state for a group of related shaders.