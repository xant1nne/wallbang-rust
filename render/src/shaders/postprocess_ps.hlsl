cbuffer PostProcessParams : register(b0) {
    float4 p0; // x=exposure, y=contrast, z=saturation, w=vibrance
    float4 p1; // x=temperature, y=tint, z=gamma, w=nightmode
    float4 p2; // x=vignette, y=sharpen, z=inv_w, w=inv_h
    float4 p3; // rgb=world_tint, a=world_tint_strength
    float4 p4; // camera blur UV xy, radial velocity blur, enabled
};

Texture2D gScene : register(t0);
SamplerState gSampler : register(s0);

struct PS_IN {
    float4 pos : SV_POSITION;
    float2 uv  : TEXCOORD0;
};

float4 main(PS_IN i) : SV_Target {
    float3 col = gScene.Sample(gSampler, i.uv).rgb;
    if (p4.w > 0.5f) {
        float2 direction = p4.xy + (i.uv - 0.5f) * p4.z;
        float3 sum = col * 5.0f;
        float weight = 5.0f;
        [unroll] for (int tap = 1; tap <= 4; ++tap) {
            float w = 5.0f - tap;
            float2 offset = direction * (tap / 4.0f);
            sum += (gScene.Sample(gSampler, saturate(i.uv + offset)).rgb
                  + gScene.Sample(gSampler, saturate(i.uv - offset)).rgb) * w;
            weight += w * 2.0f;
        }
        col = sum / weight;
    }

    // 1. CAS-like 4-tap unsharp sharpen
    if (p2.y > 0.001f) {
        float2 texel = p2.zw;
        float3 n = gScene.Sample(gSampler, i.uv + float2(0.0f, -texel.y)).rgb;
        float3 s = gScene.Sample(gSampler, i.uv + float2(0.0f,  texel.y)).rgb;
        float3 e = gScene.Sample(gSampler, i.uv + float2( texel.x, 0.0f)).rgb;
        float3 w = gScene.Sample(gSampler, i.uv + float2(-texel.x, 0.0f)).rgb;
        float3 blur = (n + s + e + w) * 0.25f;
        col = saturate(col + (col - blur) * p2.y);
    }

    // 2. Nightmode darkening with subtle cool ambient shift
    if (p1.w > 0.001f) {
        float nm = saturate(p1.w);
        col *= lerp(1.0f, 0.28f, nm);
        col *= lerp(float3(1.0f, 1.0f, 1.0f), float3(0.84f, 0.90f, 1.08f), nm);
    }

    // 2.5 World color tint modulation
    if (p3.a > 0.001f) {
        float3 tinted = col * p3.rgb;
        col = lerp(col, tinted, saturate(p3.a));
    }

    // 3. Exposure
    col *= exp2(p0.x);

    // 4. White balance: temperature (warm/cool) + tint (magenta/green)
    col.r *= 1.0f + p1.x * 0.18f + p1.y * 0.08f;
    col.g *= 1.0f - p1.y * 0.14f;
    col.b *= 1.0f - p1.x * 0.18f + p1.y * 0.08f;

    // 5. Contrast around mid-grey
    col = (col - 0.5f) * p0.y + 0.5f;
    col = max(col, 0.0f);

    // 6. Saturation & Vibrance
    float luma = dot(col, float3(0.2126f, 0.7152f, 0.0722f));
    float max_c = max(col.r, max(col.g, col.b));
    float min_c = min(col.r, min(col.g, col.b));
    float sat_mask = 1.0f - saturate(max_c - min_c);
    float total_sat = max(0.0f, p0.z + p0.w * sat_mask);
    col = lerp(float3(luma, luma, luma), col, total_sat);

    // 7. Gamma
    col = pow(saturate(col), max(0.2f, p1.z));

    // 8. Vignette
    if (p2.x > 0.001f) {
        float2 d = i.uv - 0.5f;
        float vig = saturate(1.0f - dot(d, d) * 2.2f * p2.x);
        col *= lerp(1.0f, vig, saturate(p2.x));
    }

    return float4(saturate(col), 1.0f);
}
