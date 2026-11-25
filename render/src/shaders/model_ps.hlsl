cbuffer Scene : register(b0) {
    float4x4 gViewProj;
    float4x4 gWorld;
    float4 gLightDir;
    float4 gCamPos;
    float4 gTint;
};

Texture2D gAlbedo : register(t0);
SamplerState gSampler : register(s0);

struct PS_IN {
    float4 pos : SV_POSITION;
    float3 normal : NORMAL;
    float2 uv : TEXCOORD0;
    float3 world : TEXCOORD1;
};

float4 main(PS_IN i) : SV_Target {
    float3 base = gAlbedo.Sample(gSampler, i.uv).rgb;
    float3 n = normalize(i.normal);
    float3 light = normalize(gLightDir.xyz);
    float ndl = saturate(dot(n, light));
    float3 view_dir = normalize(gCamPos.xyz - i.world);
    float rim = pow(1.0f - saturate(dot(n, view_dir)), 2.6f);
    float3 color = base * (0.30f + 0.78f * ndl) + rim * float3(0.24f, 0.34f, 0.52f);
    return float4(color * gTint.rgb, 1.0f);
}
