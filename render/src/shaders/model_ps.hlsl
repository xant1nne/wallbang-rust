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
    float3 view_dir = normalize(gCamPos.xyz - i.world);

    // 3-point studio lighting: key + cool fill + warm/accent rim
    float3 key_dir = normalize(view_dir + float3(-0.35f, 0.65f, 0.15f));
    float3 fill_dir = normalize(float3(-key_dir.x * 0.85f, 0.35f, -key_dir.z * 0.6f + 0.5f));
    float3 hemi_amb = lerp(float3(0.38f, 0.40f, 0.44f), float3(0.60f, 0.62f, 0.66f), n.y * 0.5f + 0.5f);

    float key_ndl = saturate(dot(n, key_dir));
    float fill_ndl = saturate(dot(n, fill_dir));
    float3 half_vec = normalize(key_dir + view_dir);
    float spec = pow(saturate(dot(n, half_vec)), 32.0f) * 0.08f;

    float fresnel = 1.0f - saturate(dot(n, view_dir));
    float rim = pow(fresnel, 2.4f);
    float contour = smoothstep(0.68f, 0.92f, fresnel) * 0.04f;

    float3 lit = base * (hemi_amb + float3(1.0f, 0.97f, 0.92f) * key_ndl + float3(0.28f, 0.32f, 0.40f) * fill_ndl)
               + float3(spec, spec, spec)
               + (rim * 0.12f + contour) * float3(0.55f, 0.65f, 0.80f);
    return float4(pow(saturate(lit * gTint.rgb), 0.88f), 1.0f);
}
