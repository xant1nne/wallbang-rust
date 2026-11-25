cbuffer Scene : register(b0) {
    float4x4 gViewProj;
    float4x4 gWorld;
    float4 gLightDir;
    float4 gCamPos;
    float4 gTint;
};

cbuffer Bones : register(b1) {
    float4x4 gBones[128];
};

struct VS_IN {
    float3 pos : POSITION;
    float3 nrm : NORMAL;
    float2 uv : TEXCOORD0;
    uint4 joints : JOINTS0;
    float4 weights : WEIGHTS0;
};

struct VS_OUT {
    float4 pos : SV_POSITION;
    float3 normal : NORMAL;
    float2 uv : TEXCOORD0;
    float3 world : TEXCOORD1;
};

VS_OUT main(VS_IN i) {
    VS_OUT o;
    float4 skinned = float4(i.pos, 1.0f);
    float3 n = i.nrm;
    float weights = i.weights.x + i.weights.y + i.weights.z + i.weights.w;
    if (weights > 0.0001f) {
        float4 p = float4(0.0f, 0.0f, 0.0f, 0.0f);
        float3 nn = float3(0.0f, 0.0f, 0.0f);
        [unroll] for (int k = 0; k < 4; ++k) {
            float w = i.weights[k] / weights;
            float4x4 bone = gBones[i.joints[k]];
            p += mul(bone, float4(i.pos, 1.0f)) * w;
            nn += mul((float3x3)bone, i.nrm) * w;
        }
        skinned = p;
        n = nn;
    }
    float4 world = mul(gWorld, skinned);
    o.pos = mul(gViewProj, world);
    o.normal = normalize(mul((float3x3)gWorld, n));
    o.uv = i.uv;
    o.world = world.xyz;
    return o;
}
