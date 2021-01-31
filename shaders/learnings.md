# DirectX 11 Learnings

## Effects

Effects appears to have been a deprecated concept in DirectX 11 and above.

In essence, you could have effect files (.fx), which, besides the shader functions for different stages, could also have the concept of a "technique", which included pass that could contain different combinations of shader function calls.

Now, you would have to manage all of this yourself. They were essentailly just higher-level concepts that could help manage state for a group of related shaders.