Param (
    [string]$workingDirectory
)

# Compile vertex shader
fxc.exe /E VS /T vs_5_0 /Fo "${workingDirectory}\shaders\compiled-vertex-shader.shader" "${workingDirectory}\shaders\vertex_perspective.hlsl"

# Compile pixel shader
fxc.exe /E PS /T ps_5_0 /Fo "${workingDirectory}\shaders\compiled-pixel-shader.shader" "${workingDirectory}\shaders\pixel.hlsl"