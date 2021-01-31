// COMPILED OFFLINE USING:  fxc.exe /E VS /T vs_4_1 /Fo "compiled-vertex-shader.shader" ./vertex.hlsl

// DirectX shaders are written in the HLSL (high level shading language) language.
// These are text files saved in the .fx format.

// a "cbuffer" is a Constant Buffer.
// Constant buffers are blocks of memory which can store variables which can be 
// Accessed by a shader.
// Data is constant buffers doesn't vary per vertex, but stays the same.
cbuffer cbPerObject
{
    float4x4 gWorldViewProj;
};

// The input structure coming from the pipeline
struct VertexIn
{
    // :POSITION and :COLOR are parameter semantics which are mapping from the vertex buffer in the
    // pipeline.
    float3 PosL : POSITION;
    float4 Color : COLOR;
};

struct VertexOut
{
    // In the output structure, ":SV_POSITION" and ":COLOR" are also semantics.
    // These are used to map the Vertex shader output to the inputs of the next stages
    // such as the geometry shader of the pixel shader.
    // Semantics prefixed with "SV" are special, it stands for "System Value".
    float4 PosH : SV_POSITION;
    float4 Color : COLOR;
};

VertexOut VS(VertexIn vin)
{
    VertexOut vout;

    // Transform to homogenous clip space
    // Notice that the vertex shader, or any other shader, doesn't do the perspective divide.
    // The perspective divide is done by hardware at a later stage.
    // The vertex shader just does the projection matrix.
    //vout.PosH = mul(float4(vin.PosL, 1.0f), gWorldViewProj);

    // Simply pass our vertex position to the output
    vout.PosH = float4(vin.PosL, 1.0f);

    // Just pass vertex color into the pixel shader
    vout.Color = float4(0.5f, 0.5f, 0.5f, 1.0f);

    return vout;
}